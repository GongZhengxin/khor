//! What agents this machine actually has, beside what khor knows about.
//!
//! `agents::known` is a snapshot of the ACP registry: *khor knows an argv
//! that starts this thing*. This answers the question a person asks
//! first — **which of them are on this machine, where, and what can khor
//! do with each** — and the two must not be folded together. A registry
//! entry exists whether or not anything is installed, and an installed
//! agent may not be in that registry at all: DSH is on this machine and
//! has never been an ACP entry.
//!
//! # Detection is a lookup, not a probe
//!
//! Nothing here starts an agent. Running 10 CLIs to find out whether they
//! answer would cost seconds on every open of the page, and most of them
//! refuse to do anything until somebody logs in — the registry's own probe
//! notes 19 of 32 doing exactly that. So a row is built from two facts a
//! filesystem can answer: **is the program on this machine's `PATH` (or in
//! a directory its installer uses)**, and **does its configuration home
//! exist**. A row that says "not installed" is reporting a lookup that
//! failed, not an agent that cannot be spoken to.
//!
//! # Roads, in the order they are preferred
//!
//! `builtin` — khor's own shim (`_codexagent` → `codex app-server`), no
//! npm. `native-acp` — the tool ships ACP itself (`gemini --acp`), no npm.
//! `registry-acp` — an adapter run through a package runner, which is the
//! only road that needs one. `terminal` — the floor: any program can be
//! typed into a session, and that is what remains true for every agent
//! that has no protocol at all.
//!
//! Capabilities are claimed only where this codebase can deliver them
//! today. A missing capability is not "khor cannot talk to it": it is
//! "khor has no road built yet", and the row says which.

use serde::Serialize;
use std::path::Path;

/// The package runner khor would use for registry adapters.
///
/// **`npx` only, for now.** The launcher hands a registry entry's argv to
/// the agent unchanged (`Node::agent_launch`), and every such argv begins
/// with `npx`; a machine with `pnpm` but no `npx` would therefore be told
/// "this road works" and fail at spawn. Listing only the runner the
/// launcher actually uses keeps the answer true. Adding `pnpm dlx` here
/// means teaching the launcher to rewrite that first word first — the
/// independent review found this inconsistency before it shipped.
const RUNNERS: &[(&str, &str)] = &[("npx", "npx")];

/// One agent khor knows how to find, and the roads to it.
struct Known {
    /// The ACP registry's id where one exists; otherwise the word a person
    /// types. It is the row's identity everywhere.
    id: &'static str,
    name: &'static str,
    /// The program name a shell would resolve.
    command: &'static str,
    /// Directories the program may sit in, relative to home. Installers
    /// that do not put their binary on `PATH` live here.
    program_dirs: &'static [&'static str],
    /// Directories this agent keeps configuration/state in, relative to
    /// home. Presence is evidence the tool has been used on this machine.
    config_dirs: &'static [&'static str],
    /// `command` plus these arguments starts the tool in its own ACP mode.
    native_acp: Option<&'static [&'static str]>,
    /// The registry id of an ACP adapter that launches it over a package
    /// runner, when one exists.
    registry: Option<&'static str>,
    /// khor's own shim, when this vendor has one. Runs here, no npm.
    builtin: Option<&'static str>,
    /// khor can open an existing conversation of it in a terminal.
    resume: Option<&'static str>,
    /// khor can deliver a message into an existing conversation.
    send: Option<&'static str>,
    /// khor can enumerate its conversations.
    list: bool,
    /// khor can read its transcripts.
    read: bool,
}

/// `argv` shown to people (a placeholder in `<angle>` marks the input).
const KNOWN: &[Known] = &[
    Known {
        id: "codex",
        name: "Codex",
        command: "codex",
        program_dirs: &[],
        config_dirs: &[".codex"],
        native_acp: None,
        registry: Some("codex-acp"),
        builtin: Some("_codexagent"),
        resume: Some("codex resume <id>"),
        send: Some("codex queue --thread <id> --message <text>"),
        list: true,
        read: true,
    },
    Known {
        id: "claude",
        name: "Claude Code",
        command: "claude",
        program_dirs: &[],
        config_dirs: &[".claude"],
        native_acp: None,
        registry: Some("claude-acp"),
        builtin: Some("_cagent"),
        resume: None,
        send: None,
        list: true,
        read: true,
    },
    Known {
        id: "dsh",
        name: "DeepSeek Harness",
        command: "dsh",
        program_dirs: &[],
        config_dirs: &[".dsh"],
        native_acp: None,
        registry: None,
        builtin: None,
        resume: Some("dsh tui --resume <session>"),
        send: Some("dsh --profile headless --session-id <session> \"<text>\""),
        // Its stored conversations are listed from their header frames and
        // read frame by frame (`dsh_sessions`), and a subagent's label or a
        // provider title is the name — both implemented, so both claimed.
        // Round N had these `false` after the code existed; the driver's
        // own test compares the two answers now.
        list: true,
        read: true,
    },
    Known {
        id: "gemini",
        name: "Gemini CLI",
        command: "gemini",
        program_dirs: &[],
        config_dirs: &[".gemini"],
        native_acp: Some(&["--acp"]),
        registry: Some("gemini"),
        builtin: None,
        resume: None,
        send: None,
        list: false,
        read: false,
    },
    Known {
        id: "opencode",
        name: "OpenCode",
        command: "opencode",
        program_dirs: &[],
        config_dirs: &[".config/opencode", ".opencode"],
        native_acp: Some(&["acp"]),
        registry: None,
        builtin: None,
        resume: None,
        send: None,
        list: false,
        read: false,
    },
    Known {
        id: "qwen",
        name: "Qwen Code",
        command: "qwen",
        program_dirs: &[],
        config_dirs: &[".qwen"],
        native_acp: None,
        registry: Some("qwen-code"),
        builtin: None,
        resume: None,
        send: None,
        list: false,
        read: false,
    },
    Known {
        id: "copilot",
        name: "GitHub Copilot CLI",
        command: "copilot",
        program_dirs: &[],
        config_dirs: &[".copilot"],
        native_acp: Some(&["--acp"]),
        registry: Some("github-copilot-cli"),
        builtin: None,
        resume: None,
        send: None,
        list: false,
        read: false,
    },
    Known {
        id: "pi",
        name: "Pi",
        command: "pi",
        program_dirs: &[],
        config_dirs: &[".pi"],
        native_acp: None,
        registry: Some("pi-acp"),
        builtin: None,
        resume: None,
        send: None,
        list: false,
        read: false,
    },
    Known {
        id: "aider",
        name: "Aider",
        command: "aider",
        program_dirs: &[],
        config_dirs: &[".aider"],
        native_acp: None,
        registry: None,
        builtin: None,
        resume: None,
        send: None,
        list: false,
        read: false,
    },
];

/// Where khor would start this agent, and how.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Launch {
    /// `builtin` | `native-acp` | `registry-acp` | `terminal`
    pub road: &'static str,
    /// The argv a person can read, placeholders in `<angle>`.
    pub said: String,
}

/// One row of the machine's own roster.
#[derive(Clone, Debug, Serialize)]
pub struct FleetRow {
    pub id: String,
    pub name: String,
    /// Absolute path of the program, when this machine has it.
    pub program: Option<String>,
    /// The configuration directory that exists, when one does. A machine
    /// can have the settings without the program (an uninstalled tool
    /// leaves them behind), which is why this is its own field.
    pub config: Option<String>,
    /// What khor would do with it, when it is installed.
    pub launch: Option<Launch>,
    /// `run`, `acp`, `list`, `read`, `resume`, `send`, `terminal`.
    pub capabilities: Vec<&'static str>,
    /// The registry id, so a face can look up the entry it came from.
    pub registry: Option<&'static str>,
}

/// The roster plus the one fact that decides how much of it works.
#[derive(Clone, Debug, Serialize)]
pub struct Fleet {
    /// `Some("npx")` / `Some("pnpm dlx")` when a package runner is here;
    /// `None` means every `registry-acp` road on this machine is closed
    /// and says so instead of failing at spawn time.
    pub runner: Option<&'static str>,
    pub agents: Vec<FleetRow>,
}

/// Finds the runner, then every agent, against this machine.
pub fn fleet() -> Fleet {
    let home = real_home();
    let path = crate::Node::user_path();
    fleet_at(home.as_deref(), path)
}

/// [`fleet`] against a stated home and `PATH`, so it is testable without
/// this machine's own installations deciding the answer.
pub(crate) fn fleet_at(home: Option<&Path>, path: &str) -> Fleet {
    let runner = RUNNERS
        .iter()
        .find(|(program, _)| crate::find_program(program, path).is_some())
        .map(|(_, said)| *said);
    let agents = KNOWN
        .iter()
        .map(|known| row_of(known, home, path, runner))
        .collect();
    Fleet { runner, agents }
}

fn row_of(
    known: &Known,
    home: Option<&Path>,
    path: &str,
    runner: Option<&'static str>,
) -> FleetRow {
    let program = locate_program(known, home, path);
    let config = locate_config(known, home);
    // Strongest claim first, the floor last: a reader scanning a row wants
    // to know khor can list and resume its conversations before being told
    // it can also be typed into a terminal.
    let mut capabilities: Vec<&'static str> = Vec::new();
    let mut launch = None;
    if program.is_some() {
        if let Some(shim) = known.builtin {
            launch = Some(Launch {
                road: "builtin",
                said: format!("khor {shim}"),
            });
            capabilities.push("acp");
        } else if let Some(args) = known.native_acp {
            launch = Some(Launch {
                road: "native-acp",
                said: format!("{} {}", known.command, join(args)),
            });
            capabilities.push("acp");
        } else if let Some(registry) = known.registry
            && runner.is_some()
        {
            launch = Some(Launch {
                road: "registry-acp",
                said: format!("{registry} ({})", runner.unwrap_or("")),
            });
            capabilities.push("acp");
        } else {
            launch = Some(Launch {
                road: "terminal",
                said: known.command.to_owned(),
            });
        }
        capabilities.push("run");
        // A builtin shim needs the vendor's own binary only; a registry
        // adapter needs a runner, and without one the row can still be
        // typed into a terminal.
        if launch.as_ref().is_some_and(|l| l.road == "registry-acp") {
            capabilities.push("needs-runner");
        }
        if known.list {
            capabilities.push("list");
        }
        if known.read {
            capabilities.push("read");
        }
        if known.resume.is_some() {
            capabilities.push("resume");
        }
        if known.send.is_some() {
            capabilities.push("send");
        }
    }
    capabilities.push("terminal");
    FleetRow {
        id: known.id.to_owned(),
        name: known.name.to_owned(),
        program,
        config,
        launch,
        capabilities,
        registry: known.registry,
    }
}

fn locate_program(known: &Known, home: Option<&Path>, path: &str) -> Option<String> {
    if let Some(found) = crate::find_program(known.command, path) {
        return Some(found);
    }
    let home = home?;
    for dir in known.program_dirs {
        let base = home.join(dir).join(known.command);
        // `find_program` gets the platform's own extensions from
        // `PATHEXT`; a hand-built path has to spell it out, and an
        // installer on Windows writes `aider.exe`, never `aider`.
        for candidate in [base.clone(), base.with_extension(exe_extension())] {
            if candidate.is_file() {
                return Some(candidate.display().to_string());
            }
        }
    }
    None
}

/// The suffix `std::env::consts::EXE_SUFFIX` without its leading dot, which
/// `Path::with_extension` wants.
fn exe_extension() -> &'static str {
    std::env::consts::EXE_SUFFIX.trim_start_matches('.')
}

fn locate_config(known: &Known, home: Option<&Path>) -> Option<String> {
    let home = home?;
    known
        .config_dirs
        .iter()
        .map(|dir| home.join(dir))
        .find(|candidate| candidate.is_dir())
        .map(|found| found.display().to_string())
}

fn join(args: &[&str]) -> String {
    args.join(" ")
}

/// The person's own home, not the store's: this roster is about the
/// machine, and a run with an isolated `KHOR_HOME` still sits on a machine
/// whose agents were installed by its owner.
fn real_home() -> Option<std::path::PathBuf> {
    let key = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os(key).map(std::path::PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A scratch home and `PATH` whose answer does not depend on what this
    /// machine happens to have installed.
    struct Scratch(std::path::PathBuf);
    impl Scratch {
        fn new(name: &str) -> Self {
            let dir =
                std::env::temp_dir().join(format!("khor-fleet-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            Scratch(dir)
        }
        fn dir(&self, rel: &str) -> std::path::PathBuf {
            let path = self.0.join(rel);
            std::fs::create_dir_all(&path).unwrap();
            path
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A file `find_program` will call runnable.
    ///
    /// Windows only asks that it be a file; unix asks for the executable
    /// bit, and a test that wrote a plain file passed on Windows and would
    /// have failed in CI — the independent review caught it.
    fn write_program(path: &std::path::Path) {
        std::fs::write(path, b"x").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    /// A `codex` on `PATH` and its home: the row must find both and offer
    /// khor's own shim rather than an npm adapter, because one exists.
    #[test]
    fn an_installed_agent_with_a_builtin_road_offers_it_without_a_runner() {
        let scratch = Scratch::new("builtin");
        let bin = scratch.dir("bin");
        let exe = bin.join(format!("codex{}", std::env::consts::EXE_SUFFIX));
        write_program(&exe);
        scratch.dir(".codex");
        let fleet = fleet_at(Some(&scratch.0), &bin.display().to_string());
        let row = fleet.agents.iter().find(|r| r.id == "codex").unwrap();
        assert_eq!(
            row.program.as_deref(),
            Some(exe.display().to_string().as_str())
        );
        assert!(row.config.is_some(), "the config home is its own fact");
        let launch = row.launch.as_ref().unwrap();
        assert_eq!(launch.road, "builtin");
        assert!(row.capabilities.contains(&"acp"));
        assert!(!row.capabilities.contains(&"needs-runner"));
        assert_eq!(fleet.runner, None, "nothing here needs a package runner");
    }

    /// With no program and no config, the row is present and says so: an
    /// agent khor knows about does not disappear from the roster.
    #[test]
    fn an_agent_that_is_not_installed_is_reported_rather_than_omitted() {
        let scratch = Scratch::new("absent");
        let bin = scratch.dir("bin");
        let fleet = fleet_at(Some(&scratch.0), &bin.display().to_string());
        let row = fleet.agents.iter().find(|r| r.id == "qwen").unwrap();
        assert_eq!(row.program, None);
        assert_eq!(row.config, None);
        assert_eq!(row.launch, None);
        assert_eq!(row.capabilities, vec!["terminal"]);
    }

    /// A tool whose own installer never puts its binary on `PATH` is still
    /// found where it lives — the directory list exists for exactly that.
    #[test]
    fn a_program_in_a_known_install_directory_is_found_off_the_path() {
        let scratch = Scratch::new("offpath");
        let bin = scratch.dir("bin");
        let elsewhere = scratch.dir("apps");
        let exe = elsewhere.join(format!("aider{}", std::env::consts::EXE_SUFFIX));
        write_program(&exe);
        // `aider` has no `program_dirs` in the shipped table, so this uses
        // a table-shaped lookup of its own rather than pretending the real
        // entry would find it.
        let known = Known {
            id: "aider",
            name: "Aider",
            command: "aider",
            program_dirs: &["apps"],
            config_dirs: &[".aider"],
            native_acp: None,
            registry: None,
            builtin: None,
            resume: None,
            send: None,
            list: false,
            read: false,
        };
        let row = row_of(&known, Some(&scratch.0), &bin.display().to_string(), None);
        assert_eq!(
            row.program.as_deref(),
            Some(exe.display().to_string().as_str())
        );
    }

    /// A registry-only agent with a runner present is offered that road and
    /// says it needs one; with no runner the same row offers a terminal
    /// instead of a spawn that would fail.
    #[test]
    fn a_registry_adapter_needs_a_runner_and_falls_back_to_a_terminal_without_one() {
        let scratch = Scratch::new("registry");
        let bin = scratch.dir("bin");
        let exe = bin.join(format!("qwen{}", std::env::consts::EXE_SUFFIX));
        write_program(&exe);
        let npx = bin.join(format!("npx{}", std::env::consts::EXE_SUFFIX));
        write_program(&npx);
        let path = bin.display().to_string();

        let with_runner = fleet_at(Some(&scratch.0), &path);
        let row = with_runner.agents.iter().find(|r| r.id == "qwen").unwrap();
        assert_eq!(with_runner.runner, Some("npx"));
        assert_eq!(row.launch.as_ref().unwrap().road, "registry-acp");
        assert!(row.capabilities.contains(&"needs-runner"));

        // The same machine without the runner: the row stays, the road
        // changes, and nothing claims an adapter it cannot start.
        std::fs::remove_file(&npx).unwrap();
        let without = fleet_at(Some(&scratch.0), &path);
        let row = without.agents.iter().find(|r| r.id == "qwen").unwrap();
        assert_eq!(without.runner, None);
        assert_eq!(row.launch.as_ref().unwrap().road, "terminal");
        assert!(!row.capabilities.contains(&"acp"));
    }

    /// The shipped table is internally consistent: ids are unique, a road
    /// that promises a protocol has one, and every command shown to a
    /// person marks its input so it cannot be pasted as if it were whole.
    #[test]
    fn the_shipped_table_does_not_promise_what_it_cannot_deliver() {
        let mut seen = std::collections::BTreeSet::new();
        for known in KNOWN {
            assert!(seen.insert(known.id), "duplicate id {}", known.id);
            assert!(!known.name.is_empty(), "{} has no name", known.id);
            let has_road =
                known.builtin.is_some() || known.native_acp.is_some() || known.registry.is_some();
            if !has_road {
                // **A claim needs a way to deliver it**: either a road to
                // the agent, or khor's own reader for that vendor's store
                // (`agent_driver`). DSH has no road and still lists and
                // reads, because both are done here from the files it
                // writes — that is a different way to keep the promise, not
                // an absence of one.
                for (claimed, verb) in [(known.list, "list"), (known.read, "read")] {
                    if claimed {
                        assert!(
                            crate::agent_driver::has(known.id, verb),
                            "{} claims {verb} with no road and no reader",
                            known.id
                        );
                    }
                }
            }
            for shown in [known.resume, known.send].into_iter().flatten() {
                assert!(
                    shown.contains('<'),
                    "{} shows {shown:?} with no placeholder for the input",
                    known.id
                );
            }
        }
    }
}
