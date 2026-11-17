//! The conversations DeepSeek Harness already has on this machine.
//!
//! DSH stores one directory per session under a directory per project:
//!
//! ```text
//! ~/.dsh/sessions/<project>/<session>/session.v4.jsonl.zstd
//! ```
//!
//! The file is zstd-compressed JSON lines whose **first line is the session
//! header** — `{type:"session", id, createdAt, cwd, parentSession?,
//! origin?}` — so a listing needs the first compressed frame and nothing
//! more: no full decompression, no decoding of the directory name (which
//! encodes a path with its own escaping that nothing here would want to
//! re-implement).
//!
//! # What this is and is not
//!
//! It is **inventory**: what conversations exist, in which directory, when
//! they started. It is not a transcript reader — the message events later
//! in the file are a different job — and it is not the ACP `session/list`
//! path: DSH is not registered as an agent, so this reads the store it
//! writes, the way the Codex and Claude adaptors read theirs.
//!
//! # Why `ruzstd`
//!
//! Decompression only, in Rust, no C toolchain: the file is read once for
//! its header line and thrown away. A build dependency that needs a C
//! compiler would put a toolchain requirement on every platform khor
//! ships to, for a read this crate does not need.

use std::io::Read;
use std::path::{Path, PathBuf};

/// One DSH session, as its own header states it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DshSession {
    /// `session-<uuid>` in newer stores, a bare uuid in older ones.
    pub id: String,
    /// The directory the session was recorded in — its project, for khor.
    pub cwd: String,
    pub created_at_ms: i64,
    /// A subagent's session names the session it was delegated from.
    pub parent: Option<String>,
    /// `origin` when DSH wrote one (`"subagent"` today).
    pub origin: Option<String>,
    /// Where the file is, for a later reader.
    pub path: String,
}

/// The header line's fields this cares about. Unknown fields are ignored
/// — DSH's own format is the authority and it grows.
#[derive(serde::Deserialize)]
struct Header {
    #[serde(rename = "type")]
    kind: String,
    id: String,
    #[serde(rename = "createdAt", default)]
    created_at: i64,
    cwd: String,
    #[serde(rename = "parentSession", default)]
    parent_session: Option<String>,
    #[serde(default)]
    origin: Option<String>,
}

/// Every session in this machine's DSH home, newest first.
///
/// `DSH_HOME` decides the home when it is set (the CLI honours it, so a
/// person running an isolated profile sees that profile); otherwise
/// `~/.dsh`. A missing home is an empty list, not an error: most machines
/// do not run DSH at all.
pub fn sessions() -> Result<Vec<DshSession>, String> {
    let Some(home) = home() else {
        return Ok(Vec::new());
    };
    sessions_in(&home)
}

/// [`sessions`] against a stated home, so the scan is testable without
/// this machine's own DSH deciding the answer.
pub(crate) fn sessions_in(home: &Path) -> Result<Vec<DshSession>, String> {
    let root = home.join("sessions");
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for project in dirs(&root) {
        for session in dirs(&project) {
            // Newer stores name the file `session.v4.jsonl.zstd`; the meta
            // file of an old one may be named differently, and a directory
            // without one is not a session.
            let Some(file) = file_in(&session) else {
                continue;
            };
            match read_header(&file) {
                Ok(header) if header.kind == "session" => out.push(DshSession {
                    id: header.id,
                    cwd: header.cwd,
                    created_at_ms: header.created_at,
                    parent: header.parent_session,
                    origin: header.origin,
                    path: file.display().to_string(),
                }),
                // **A file this cannot read is skipped, not fatal.** One
                // corrupt or newer-format session must not hide the rest,
                // and there is no sentence a list could show per file that
                // would be worth more than the other fourteen rows.
                Ok(_) | Err(_) => continue,
            }
        }
    }
    out.sort_by(|a, b| b.created_at_ms.cmp(&a.created_at_ms));
    Ok(out)
}

fn home() -> Option<PathBuf> {
    if let Some(told) = std::env::var_os("DSH_HOME") {
        return Some(PathBuf::from(told));
    }
    std::env::home_dir().map(|home| home.join(".dsh"))
}

fn dirs(path: &Path) -> Vec<PathBuf> {
    let Ok(read) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    read.filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect()
}

fn file_in(session: &Path) -> Option<PathBuf> {
    let read = std::fs::read_dir(session).ok()?;
    read.filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("session.") && name.ends_with(".jsonl.zstd"))
        })
}

/// The first line of the file's first frame.
///
/// Bounded on both sides: at most [`LIMIT`] decompressed bytes are taken,
/// and a frame that produces no newline is given up on rather than read
/// further — a header is a few hundred bytes.
const LIMIT: usize = 64 * 1024;

fn read_header(file: &Path) -> Result<Header, String> {
    let opened = std::fs::File::open(file).map_err(|error| error.to_string())?;
    let mut decoder =
        ruzstd::decoding::StreamingDecoder::new(opened).map_err(|error| error.to_string())?;
    let mut text = Vec::new();
    let mut buffer = [0u8; 8 * 1024];
    while text.len() < LIMIT {
        match decoder.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                text.extend_from_slice(&buffer[..count]);
                if text.contains(&b'\n') {
                    break;
                }
            }
            Err(error) => {
                if text.is_empty() {
                    return Err(error.to_string());
                }
                break;
            }
        }
    }
    let line = text.split(|byte| *byte == b'\n').next().unwrap_or(&[]);
    serde_json::from_slice::<Header>(line).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The header is parsed from its fields, not by position: DSH adds
    /// fields over time and a reader that assumed an order would break on
    /// the next release.
    #[test]
    fn a_header_is_read_from_its_fields() {
        let line = br#"{"type":"session","version":4,"id":"session-0b5f9684","createdAt":1794729585272,"cwd":"F:\\#Gong\\#AI4S\\khor","parentSession":"session-1d61","isSeeded":false,"origin":"subagent","agentPreset":"standard"}"#;
        let header: Header = serde_json::from_slice(line).expect("a real header shape");
        assert_eq!(header.kind, "session");
        assert_eq!(header.id, "session-0b5f9684");
        assert_eq!(header.created_at, 1794729585272);
        assert_eq!(header.cwd, r"F:\#Gong\#AI4S\khor");
        assert_eq!(header.parent_session.as_deref(), Some("session-1d61"));
        assert_eq!(header.origin.as_deref(), Some("subagent"));
    }

    /// A home that does not exist is an empty list: most machines do not
    /// run DSH, and "no DSH here" is not a failure to report.
    #[test]
    fn a_machine_without_dsh_has_no_sessions() {
        let missing = std::env::temp_dir().join("khor-dsh-nonexistent-home");
        assert!(sessions_in(&missing).unwrap().is_empty());
    }

    /// A directory that is not a session, and a session whose file is not
    /// zstd, are both skipped: one bad entry must not fail the listing.
    #[test]
    fn entries_it_cannot_read_are_skipped_rather_than_failing_the_list() {
        let scratch = std::env::temp_dir().join(format!("khor-dsh-scan-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&scratch);
        let project = scratch.join("sessions").join("--proj--");
        let broken = project.join("not-a-session");
        let empty = project.join("empty");
        std::fs::create_dir_all(&broken).unwrap();
        std::fs::create_dir_all(&empty).unwrap();
        std::fs::write(broken.join("notes.txt"), b"x").unwrap();
        std::fs::write(
            empty.join("session.v4.jsonl.zstd"),
            b"this is not a zstd frame",
        )
        .unwrap();
        let listed = sessions_in(&scratch).expect("a scan that tolerates junk");
        assert!(listed.is_empty(), "{listed:?}");
        let _ = std::fs::remove_dir_all(&scratch);
    }
}
