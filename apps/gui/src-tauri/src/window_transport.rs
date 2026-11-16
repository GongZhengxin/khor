//! The window's own loopback transport, for commands Tauri's IPC cannot carry.
//!
//! **Why this exists** (measured 2026-11-16, Windows 11 x64, WebView2 157).
//! The window's page sends every command as a `fetch` to
//! `http://ipc.localhost/<command>`, and that transport **stops after the
//! startup burst**. Instrumented evidence, identical across many runs:
//!
//! - `[ipc]` lines printed at the top of `invoke_handler` always stop at the
//!   same nine commands, after which nothing ever arrives again.
//! - The page meanwhile made 38 `sessions` calls; none reached Rust; no error
//!   was raised on either side — the promises simply stayed pending.
//! - A page-side heartbeat of 15 invokes reported `{sent:15, ok:0, err:0,
//!   pending:13}`; the arrival count stayed at 9.
//! - **A bare `fetch` to `ipc.localhost` — no `invoke`, no command, no
//!   callback bookkeeping — failed 40 of 40 times**, each timing out at the
//!   abort deadline. So the defect is in wry's `WebResourceRequested`
//!   handling (every matching request takes a `deferral` that is completed
//!   only when the response is delivered back to the page) and not in Tauri's
//!   dispatch or in Khor's commands.
//! - Ruled out by experiment: the backend itself (80–120 ms for every
//!   arrangement), `RootPermit` self-contention (126 ms), the tokio blocking
//!   pool (`off_main` traces enter in 14 ms and settle in 310 ms), a corrupted
//!   WebView2 profile (a brand-new profile behaves the same), and the window
//!   being minimised (restoring it does not help).
//! - The same frontend over plain HTTP works: the web face rendered the full
//!   session list in a real browser.
//!
//! The page already has three transports (`apps/gui/src/api.ts`, `call()`):
//! the dev bridge, Tauri's `invoke`, and the web face's `/api/`. The first is a
//! loopback HTTP server dispatching to the very same
//! [`khor_gui_core::api::answer`] table the others use. This module is that
//! server, run inside the app: the window is pointed at
//! `index.html?bridge=<port>`, so `call()` takes the loopback road and the dead
//! custom protocol is never touched.
//!
//! **What does not change**: the page, its assets, events, window controls,
//! drag regions and the native menu — those are Tauri's own APIs, not IPC
//! commands, and `?bridge=` leaves them alone. `window.__TAURI_INTERNALS__` is
//! still injected; `call()` simply checks `bridge` first.
//!
//! **What this is not**: a product web server. It binds loopback in this
//! process and answers this window. The LAN face is `khor_web`'s job.

use std::io::Read;

/// The port the window's `?bridge=` names.
///
/// **Not 1431** — the dev bridge's default (`BRIDGE_PORT`); a developer running
/// both would collide. Not 5467 either: that is the resident's web face, which
/// a person may have open in a browser at the same time. This one belongs to
/// the window.
pub const PORT: u16 = 1432;

fn cors<R: Read>(response: tiny_http::Response<R>) -> tiny_http::Response<R> {
    // The page's origin is `tauri.localhost`, not this loopback port, so every
    // request is cross-origin and the preflight has to be answered.
    response
        .with_header(
            "Access-Control-Allow-Origin: *"
                .parse::<tiny_http::Header>()
                .unwrap(),
        )
        .with_header(
            "Access-Control-Allow-Headers: content-type"
                .parse::<tiny_http::Header>()
                .unwrap(),
        )
        .with_header(
            "Access-Control-Allow-Methods: POST, OPTIONS"
                .parse::<tiny_http::Header>()
                .unwrap(),
        )
}

fn json(outcome: Result<String, String>) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let (code, payload) = match outcome {
        Ok(json) => (200, json),
        Err(why) => (
            400,
            serde_json::to_string(&why).unwrap_or_else(|_| "\"error\"".into()),
        ),
    };
    cors(
        tiny_http::Response::from_string(payload)
            .with_status_code(code)
            .with_header(
                "Content-Type: application/json"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            ),
    )
}

/// One command's answer, through the shared table.
///
/// The belt under the handler, copied from the dev bridge for the same reason:
/// a panic in one command must be that request's 500, not the death of the
/// transport the whole window depends on.
fn answer(cmd: &str, args: &serde_json::Value) -> Result<String, String> {
    let root = khor_node::Node::root_from_env();
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(error) => return Err(error.to_string()),
    };
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        khor_gui_core::api::answer(&rt, &root, cmd, args)
    }))
    .unwrap_or_else(|_| Err("panic in handler".to_owned()))
}

/// Start the window's loopback transport.
///
/// A port already taken means another window of this app owns it, which is
/// fine: the second window talks to the first's server, and both reach the same
/// store.
pub fn start() {
    let server = match tiny_http::Server::http(("127.0.0.1", PORT)) {
        Ok(server) => server,
        Err(error) => {
            eprintln!("khor: the window's loopback transport did not bind {PORT}: {error}");
            return;
        }
    };
    eprintln!("khor: window transport on 127.0.0.1:{PORT}");
    std::thread::spawn(move || {
        for mut request in server.incoming_requests() {
            if request.method() == &tiny_http::Method::Options {
                let _ = request.respond(cors(
                    tiny_http::Response::from_string("").with_status_code(204),
                ));
                continue;
            }
            let url = request.url().trim_start_matches('/').to_owned();
            let (cmd, query) = url.split_once('?').unwrap_or((url.as_str(), ""));
            let cmd = cmd.to_owned();
            let query = query.to_owned();
            // The raw-body routes carry a file, which is not UTF-8 and must
            // never pass through a String. Each keeps the road the dev bridge
            // gives it (`bridge.rs`), so the two faces cannot drift.
            let raw = matches!(
                cmd.as_str(),
                "upload" | "chat_put" | "term_drop_stage" | "term_drop_stage_into"
            );
            if raw {
                let root = khor_node::Node::root_from_env();
                let outcome: Result<String, String> = if cmd == "term_drop_stage_into" {
                    khor_gui_core::drop_stage_into_args(&query)
                        .ok_or_else(|| "term_drop_stage_into needs folder and path".to_owned())
                        .and_then(|(folder, path)| {
                            khor_gui_core::term::term_drop_stage_into(
                                &root,
                                &folder,
                                &path,
                                request.as_reader(),
                            )
                            .map(|id| serde_json::to_string(&id).unwrap_or_default())
                        })
                } else if cmd == "term_drop_stage" {
                    khor_gui_core::drop_stage_args(&query)
                        .ok_or_else(|| "term_drop_stage needs a name query arg".to_owned())
                        .and_then(|name| {
                            khor_gui_core::term::term_drop_stage(
                                &root,
                                &name,
                                request.as_reader(),
                            )
                            .map(|id| serde_json::to_string(&id).unwrap_or_default())
                        })
                } else if cmd == "chat_put" {
                    khor_gui_core::chat_put_args(&query)
                        .ok_or_else(|| "chat_put needs id, lease and name query args".to_owned())
                        .and_then(|(id, lease, name)| {
                            khor_gui_core::chat::chat_put(
                                &root,
                                &id,
                                &lease,
                                &name,
                                &khor_gui_core::into_arg(&query),
                                request.as_reader(),
                            )
                            .map(|paths| serde_json::to_string(&paths).unwrap_or_default())
                        })
                } else {
                    khor_gui_core::upload_args(&query)
                        .ok_or_else(|| "upload needs a name query arg".to_owned())
                        .and_then(|name| {
                            khor_gui_core::stage_upload(&root, &name, request.as_reader())
                                .map(|id| serde_json::to_string(&id).unwrap_or_default())
                        })
                };
                let _ = request.respond(json(outcome));
                continue;
            }
            let mut body = String::new();
            if let Err(error) = request.as_reader().read_to_string(&mut body) {
                let _ = request.respond(json(Err(error.to_string())));
                continue;
            }
            let args: serde_json::Value = if body.trim().is_empty() {
                serde_json::Value::Null
            } else {
                match serde_json::from_str(&body) {
                    Ok(value) => value,
                    Err(error) => {
                        let _ = request.respond(json(Err(error.to_string())));
                        continue;
                    }
                }
            };
            let _ = request.respond(json(answer(&cmd, &args)));
        }
    });
}
