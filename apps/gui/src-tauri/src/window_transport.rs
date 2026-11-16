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
use std::sync::OnceLock;
// `get_webview_window` is a `Manager` method; without this the call does not
// resolve on `&AppHandle`.
use tauri::Manager;

/// The window this transport answers for.
///
/// Set once, when the window is built. The loopback server starts earlier (the
/// port has to be listening before the page's first command), so it cannot take
/// the handle at construction time.
static WINDOW: OnceLock<tauri::AppHandle> = OnceLock::new();

/// Tell the transport which app it serves. Called from the window's build.
pub fn attach(app: tauri::AppHandle) {
    let _ = WINDOW.set(app);
}

/// The commands that are about *this window* rather than about the store.
///
/// **They cannot go through [`khor_gui_core::api::answer`]**, which is the
/// store's table: minimise, maximise, close and the title-row measurements are
/// Tauri window methods, and the page asks for them like any other command
/// (`apps/gui/src/api.ts`). Before the loopback transport they reached the
/// skin's own `#[tauri::command]`s; with the window on `?bridge=` they arrive
/// here instead, so this is where they have to be answered — otherwise the
/// borderless window's own three buttons do nothing at all (found 2026-11-16,
/// the day the decorations came off).
///
/// `None` means "not a window verb, ask the store's table".
fn window_verb(cmd: &str) -> Option<Result<String, String>> {
    let window = || {
        WINDOW
            .get()
            .ok_or_else(|| "the window is not attached yet".to_owned())?
            .get_webview_window("main")
            .ok_or_else(|| "no main window".to_owned())
    };
    let answer = match cmd {
        "window_minimize" => window().and_then(|w| w.minimize().map_err(|e| e.to_string())),
        "window_maximize_toggle" => window().and_then(|w| {
            let full = w.is_maximized().map_err(|e| e.to_string())?;
            if full {
                w.unmaximize().map_err(|e| e.to_string())
            } else {
                w.maximize().map_err(|e| e.to_string())
            }
        }),
        // `close`, not `destroy`: closing is a request, and the draft guard
        // hangs on it (`lib.rs`'s own note on `window_close`).
        "window_close" => window().and_then(|w| w.close().map_err(|e| e.to_string())),
        // **Dragging.** Tauri's own `drag.js` calls
        // `invoke('plugin:window|start_dragging')` on a `data-tauri-drag-region`
        // mousedown, which is the same dead custom-protocol road as every other
        // invoke — so a borderless window could not be moved at all (reported
        // 2026-11-16). The page's own window controls already come through here,
        // so the drag does too: `drag_shim` posts to this verb.
        "window_drag" => window().and_then(|w| w.start_dragging().map_err(|e| e.to_string())),
        // Windows has no system buttons standing over the title row, so the
        // row reserves nothing for them (the same zero the skin answers off
        // macOS).
        "traffic_room" => return Some(Ok("0".to_owned())),
        // No vibrancy and no backdrop material on this platform; saying so is
        // the honest answer, and the page already branches on it.
        "window_appearance_support" => {
            return Some(Ok(r#"{"supported":false,"blur":false}"#.to_owned()));
        }
        // Accepted and nothing to paint: the window is drawn by the page.
        "set_window_theme" | "set_window_appearance" => {
            return Some(Ok("null".to_owned()));
        }
        _ => return None,
    };
    Some(answer.map(|()| "null".to_owned()))
}

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

/// How many requests the transport answers at once.
///
/// **Not one.** The window polls `sessions` every 2 s, `devices` and
/// `day_strips` on their own beats, and `tabs_list` twice a second — and some
/// commands are slow: the W7 matrix measured `session_create` at 6.4 s and
/// `cli_doctor` at 5.9 s on this platform. A single-threaded loop meant one slow
/// command held every poll behind it, which is what "the app feels stuck" looks
/// like from outside (reported by the owner 2026-11-16, and the same reason the
/// dev bridge has its own note about workers).
///
/// Four is enough for the poll set plus one person-initiated command: the polls
/// are the ones that must never wait, and `tiny_http` hands each thread its own
/// request.
pub const WORKERS: usize = 4;

/// One command's answer, through the shared table.
///
/// The belt under the handler, copied from the dev bridge for the same reason:
/// a panic in one command must be that request's 500, not the death of the
/// transport the whole window depends on.
///
/// **The runtime is the caller's**, built once per worker thread. Building one
/// per request (what this did first) paid a runtime setup on every poll and had
/// nowhere to keep a driver between them.
fn answer(
    rt: &tokio::runtime::Runtime,
    cmd: &str,
    args: &serde_json::Value,
) -> Result<String, String> {
    let root = khor_node::Node::root_from_env();
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        khor_gui_core::api::answer(rt, &root, cmd, args)
    }))
    .unwrap_or_else(|_| Err("panic in handler".to_owned()))
}

/// The initialization script that restores window dragging.
///
/// Tauri ships its own (`tauri-2.11.5/src/window/scripts/drag.js`) and it is
/// correct — but it starts the drag with
/// `window.__TAURI_INTERNALS__.invoke('plugin:window|start_dragging')`, which is
/// the custom-protocol IPC that stalls on this platform. With the window
/// borderless there is nothing else to grab, so it has to be started over the
/// same loopback road the other commands take.
///
/// **Capture phase, and it stops propagation**: tauri's listener is on
/// `document` in the bubble phase, and letting both run would start a drag and
/// a double-click maximise at once. Stopping in the capture phase means only one
/// of them ever sees the event.
///
/// The region rules are tauri's own, restated: a clickable element blocks a
/// drag unless it carries the attribute itself, `="false"` disables, `="deep"`
/// makes the whole subtree draggable, and a bare/`"true"` value drags only when
/// the press landed on that very element.
pub fn drag_shim(port: u16) -> String {
    format!(
        r#"(function () {{
  var PORT = {port};
  var ATTR = 'data-tauri-drag-region';
  var CLICKABLE = new Set(['A','BUTTON','INPUT','SELECT','TEXTAREA','LABEL','SUMMARY']);
  var ROLES = new Set(['button','link','menuitem','tab','checkbox','radio','switch','option']);
  function clickable(el) {{
    return CLICKABLE.has(el.tagName)
      || (el.hasAttribute('contenteditable') && el.getAttribute('contenteditable') !== 'false')
      || (el.hasAttribute('tabindex') && el.getAttribute('tabindex') !== '-1')
      || ROLES.has(el.getAttribute('role'));
  }}
  function dragRegion(path) {{
    for (var i = 0; i < path.length; i++) {{
      var el = path[i];
      if (!el || el.nodeType !== 1) continue;
      var attr = el.getAttribute(ATTR);
      if (clickable(el) && attr === null) return false;
      if (attr === null) continue;
      if (attr === 'false') return false;
      if (attr === 'deep') return true;
      if (attr === '' || attr === 'true') return el === path[0];
    }}
    return false;
  }}
  function ask(verb) {{
    try {{
      fetch('http://127.0.0.1:' + PORT + '/' + verb, {{
        method: 'POST', body: '{{}}', headers: {{ 'Content-Type': 'application/json' }}
      }});
    }} catch (e) {{ /* nothing to say that the window can act on */ }}
  }}
  document.addEventListener('mousedown', function (e) {{
    if (e.button !== 0 || (e.detail !== 1 && e.detail !== 2)) return;
    var path = e.composedPath ? e.composedPath() : [];
    if (!dragRegion(path)) return;
    e.preventDefault();
    e.stopPropagation();
    ask(e.detail === 2 ? 'window_maximize_toggle' : 'window_drag');
  }}, true);
}})();"#
    )
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
    eprintln!("khor: window transport on 127.0.0.1:{PORT} ({WORKERS} workers)");
    let server = std::sync::Arc::new(server);
    for worker in 0..WORKERS {
        let server = std::sync::Arc::clone(&server);
        let spawned = std::thread::Builder::new()
            .name(format!("khor-window-{worker}"))
            .spawn(move || serve(server));
        if let Err(error) = spawned {
            eprintln!("khor: a window-transport worker did not start: {error}");
        }
    }
}

/// One worker: its own runtime for the life of the thread, then requests until
/// the server goes away.
///
/// A runtime per worker, not per request: `khor_gui_core::api::answer` takes a
/// runtime for the verbs that dial, and a poll every two seconds should not pay
/// a runtime's construction each time.
fn serve(server: std::sync::Arc<tiny_http::Server>) {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(error) => {
            eprintln!("khor: a window-transport worker has no runtime: {error}");
            return;
        }
    };
    // `recv` hands this thread the next request, or an error when the server is
    // gone; several workers may call it at once, which is the point.
    while let Ok(mut request) = server.recv() {
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
        // Named on stderr when `KHOR_GUI_TRACE` is set, so which commands the
        // page asks for — and which it never asks for — is answerable without
        // a devtools session (whose isolated world cannot see invoke callbacks).
        if std::env::var_os("KHOR_GUI_TRACE").is_some() {
            eprintln!("[transport] {cmd}");
        }
        // The window's own verbs first: they are not the store's to answer.
        if let Some(outcome) = window_verb(&cmd) {
            let _ = request.respond(json(outcome));
            continue;
        }
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
                        khor_gui_core::term::term_drop_stage(&root, &name, request.as_reader())
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
        let _ = request.respond(json(answer(&rt, &cmd, &args)));
    }
}
