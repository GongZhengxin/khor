//! Putting a message into a conversation the vendor keeps.
//!
//! `khor send`/`khor tell` drive **khor's own** sessions. This is the other
//! case: a conversation Codex owns, on a machine where it is running, and
//! a person wants to say one thing to it without taking it over. Codex
//! ships exactly that — `codex queue --thread <id> --message <text>` — and
//! the write goes through Codex's own daemon, so there is no second writer
//! and no lock to fight (the reason a takeover of a *loaded* thread is
//! refused elsewhere in this crate).
//!
//! # Why an argv and not a line
//!
//! The text is a person's sentence: quotes, newlines, `%`, `!` are all
//! ordinary. It is handed to `Command` as one argument, never to a shell,
//! so nothing in it can become syntax. There is no shell to quote for on
//! any platform — `codex` is resolved by `find_program` and executed
//! directly.
//!
//! # What "supported" means here
//!
//! Only Codex, and only because this tree has verified its argv. An
//! unknown agent is refused by name rather than guessed at: a wrong flag
//! would look like a message that was sent.

use khor_catalog::msg;
use std::process::Stdio;

/// The longest message this will carry. Codex takes more; a person
/// pasting a file by accident should be told, not have it injected.
const MAX_TEXT: usize = 16 * 1024;

/// Sends `text` into `native_id` through the vendor that owns it.
///
/// Answers the vendor's own receipt line when it printed one (so a person
/// sees Codex's words, not ours), and its stderr when it refused.
pub fn send(agent: &str, native_id: &str, text: &str) -> Result<String, String> {
    let argv = command_for(agent, native_id, text)?;
    let path = crate::Node::user_path();
    let program =
        crate::find_program(&argv[0], path).ok_or_else(|| msg::native_send_no_program(&argv[0]))?;
    let mut command = crate::console::hidden(&program);
    command
        .args(&argv[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let out = command
        .output()
        .map_err(|error| msg::wont_start(&argv[0], error))?;
    let said = String::from_utf8_lossy(&out.stdout);
    let trouble = String::from_utf8_lossy(&out.stderr);
    if out.status.success() {
        let line = said
            .lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("")
            .trim();
        return Ok(if line.is_empty() {
            msg::native_send_accepted(&argv[0])
        } else {
            line.to_owned()
        });
    }
    let why = if trouble.trim().is_empty() {
        said.trim().to_owned()
    } else {
        trouble.trim().to_owned()
    };
    Err(if why.is_empty() {
        msg::native_send_refused(&argv[0], out.status)
    } else {
        why
    })
}

/// The argv for one vendor, or the sentence saying why there is none.
///
/// Split out so the argument shape is testable without starting a process
/// — a wrong `--thread` would be refused by Codex with a sentence about a
/// missing argument, which reads like the id being wrong.
pub(crate) fn command_for(agent: &str, native_id: &str, text: &str) -> Result<Vec<String>, String> {
    if text.trim().is_empty() {
        return Err(msg::native_send_empty().into());
    }
    if text.len() > MAX_TEXT {
        return Err(msg::native_send_long(text.len(), MAX_TEXT));
    }
    if text.contains('\0') || native_id.contains('\0') || native_id.is_empty() {
        return Err(msg::native_send_nul().into());
    }
    match agent {
        "codex" => Ok(vec![
            "codex".to_owned(),
            "queue".to_owned(),
            "--thread".to_owned(),
            native_id.to_owned(),
            "--message".to_owned(),
            text.to_owned(),
        ]),
        other => Err(msg::native_send_unknown(other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_gets_one_argument_for_the_message() {
        let argv = command_for(
            "codex",
            "01a1de3a-93d7-7791-93da-5d5a31cfc817",
            "hi \"there\"\n!",
        )
        .expect("codex is known");
        assert_eq!(argv[0], "codex");
        assert_eq!(argv[1], "queue");
        assert_eq!(argv[2], "--thread");
        assert_eq!(argv[3], "01a1de3a-93d7-7791-93da-5d5a31cfc817");
        assert_eq!(argv[4], "--message");
        // The sentence stays one argument: quotes and a newline in it are
        // data, not syntax, because no shell ever sees this.
        assert_eq!(argv[5], "hi \"there\"\n!");
        assert_eq!(argv.len(), 6);
    }

    #[test]
    fn an_unknown_vendor_is_refused_by_name_and_nothing_is_invented() {
        let why = command_for("claude", "x", "hi").expect_err("no claude path yet");
        assert!(why.contains("claude"), "{why}");
    }

    #[test]
    fn empty_and_oversized_messages_are_refused_before_anything_starts() {
        assert!(command_for("codex", "id", "   ").is_err());
        assert!(command_for("codex", "id", &"x".repeat(MAX_TEXT + 1)).is_err());
        assert!(command_for("codex", "", "hi").is_err());
    }
}
