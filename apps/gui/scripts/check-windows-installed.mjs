// Called by the Windows installer check after opening the packaged app
// and creating a local cmd.exe session in an isolated store.
import assert from "node:assert/strict";
import { once } from "node:events";
import { createServer } from "node:http";
import { basename } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { chromium } from "playwright-core";

const [, , port, title, ready, workspace, exitMachine] = process.argv;
const endpoint = `http://127.0.0.1:${port}`;
function isAppPage(url) {
  try {
    const parsed = new URL(url);
    return ["http:", "https:"].includes(parsed.protocol) && parsed.hostname === "tauri.localhost";
  } catch {
    return false;
  }
}

async function waitForWindow() {
  const until = Date.now() + 60_000;
  let last = "no response";
  while (Date.now() < until) {
    try {
      const response = await fetch(`${endpoint}/json/list`, {signal: AbortSignal.timeout(1000)});
      if (response.ok) {
        const targets = await response.json();
        if (targets.some(target => target.type === "page" && isAppPage(target.url))) return;
        last = targets.map(({type, url}) => ({type, url: url?.split(/[?#]/, 1)[0]}));
      } else {
        last = `HTTP ${response.status}`;
      }
    } catch (error) {
      last = error.cause?.code ?? error.message;
    }
    await delay(200);
  }
  throw new Error(`WebView2 did not report the bundled App page on port ${port}: ${JSON.stringify(last)}`);
}

function command(page, name, args, timeout = 15_000) {
  return page.evaluate(async ({name, args, timeout}) => {
    const transport = window.__KHOR_WINDOW_TRANSPORT__;
    const response = await fetch(`${transport.url}/${name}`, {
      method: "POST",
      headers: {"Content-Type": "application/json", "X-Khor-Window-Token": transport.token},
      body: JSON.stringify(args),
      signal: AbortSignal.timeout(timeout),
    });
    if (!response.ok) throw new Error(`${name} returned ${response.status}: ${await response.text()}`);
    return response.json();
  }, {name, args, timeout});
}

async function checkBrowser(page, navigation) {
  const requests = new Map();
  const fixture = createServer((request, response) => {
    requests.set(request.url, request.headers.connection);
    response.writeHead(200, {"Content-Type": "text/html; charset=utf-8", "Connection": "close"});
    response.end("<!doctype html><title>Khor installer browser</title>");
  });
  fixture.listen(0, "127.0.0.1");
  await once(fixture, "listening");
  try {
    const base = `http://127.0.0.1:${fixture.address().port}`;
    const view = await page.evaluate(() => innerHeight);
    const agent = await page.evaluate(() => navigator.userAgent);
    const at = {x: 280, y: 75, w: 540, h: 400, view};
    const tabs = [
      {tab: "windows-installer-direct", machine: "", path: "/direct"},
      {tab: "windows-installer-borrow", machine: exitMachine, path: "/borrow"},
    ];
    for (const {tab, machine, path} of tabs) {
      await command(page, "open_page", {tab, machine, url: base + path, agent, at}, 90_000);
      const until = Date.now() + 15_000;
      let state;
      do {
        state = await command(page, "page_state", {tab});
        if (requests.has(path) && ["finished", "settled"].includes(state?.phase)
            && navigation.some(event => event.tab === tab)) break;
        await delay(200);
      } while (Date.now() < until);
      assert(requests.has(path), `The native WebView must request ${path}`);
      assert(["finished", "settled"].includes(state?.phase), `The ${path} WebView did not finish: ${JSON.stringify(state)}`);
      assert.equal(state.proxied, !!machine);
      assert(navigation.some(event => event.tab === tab), `The main WebView missed navigation events for ${tab}`);
    }
    // Both nodes are on the runner. A direct WebView would reach the same
    // address, but only the Khor tunnel's HTTP proxy rewrites Connection.
    assert.equal(requests.get("/direct"), "keep-alive");
    assert.equal(requests.get("/borrow"), "close");
    for (const {tab} of tabs) {
      await command(page, "close_page", {tab});
      assert.equal(await command(page, "page_state", {tab}), null, "Closing retires the native child WebView");
    }
  } finally {
    fixture.close();
  }
}

async function findWindowPage(browser) {
  const until = Date.now() + 15_000;
  let pages = [];
  do {
    pages = browser.contexts().flatMap(context => context.pages());
    const main = pages.find(page => isAppPage(page.url()));
    if (main) return main;
    await delay(200);
  } while (Date.now() < until);
  const targets = pages.map(page => page.url().split(/[?#]/, 1)[0]);
  throw new Error(`WebView2 connected but did not attach the App page: ${JSON.stringify(targets)}`);
}

await waitForWindow();
const browser = await chromium.connectOverCDP(endpoint, {timeout: 15_000});
try {
  const page = await findWindowPage(browser);
  page.setDefaultTimeout(20_000);
  const windowButtons = page.locator("[data-window-controls], [data-window-control]");

  const navigation = [];
  await page.exposeFunction("recordInstallerNavigation", (event) => navigation.push(event));
  const callback = await page.evaluate(() => window.__TAURI_INTERNALS__.transformCallback(
    event => { void window.recordInstallerNavigation(event.payload); },
  ));
  const eventId = await command(page, "window_event_listen", {
    event: "page:navigation", target: {kind: "Any"}, handler: callback,
  });
  assert(Number.isInteger(eventId), "The Windows event bridge must return its listener ID");
  assert(Array.isArray(await command(page, "ssh_config_hosts")), "The Windows App must answer the SSH host picker");

  await page.locator('[data-landing="sessions"]:visible').first().click();
  await page.locator('[data-pane-filter]:visible').click();
  const conversations = page.locator('[data-filter-option="talk:yes"]');
  await conversations.waitFor();
  assert.equal(await conversations.getAttribute("aria-checked"), "false");
  assert.equal(await page.locator('[data-arrange-option="recent"]').getAttribute("aria-checked"), "true");
  await page.keyboard.press("Escape");

  const row = page.locator(`[data-list] [data-kind="shell"][data-title=${JSON.stringify(title)}]`);
  await row.locator('[data-row-open]:visible').click();
  const terminal = page.locator("[data-terminal]:visible");
  await terminal.waitFor();

  const readTerminal = marker => page.waitForFunction(value => {
    const terminal = document.querySelector("[data-terminal]");
    const buffer = terminal?.__xterm?.buffer.active;
    if (!buffer) return false;
    for (let index = Math.max(0, buffer.length - 48); index < buffer.length; index++) {
      if (buffer.getLine(index)?.translateToString(true).trim() === value) return true;
    }
    return false;
  }, marker);
  await readTerminal(ready);
  await terminal.evaluate(element => element.__xterm.focus());
  await page.keyboard.type("echo KHOR_TERMINAL_INPUT_^OK");
  await page.keyboard.press("Enter");
  await readTerminal("KHOR_TERMINAL_INPUT_OK");
  assert.equal(await windowButtons.count(), 0, "Session panes inherit the native caption");
  await page.keyboard.type("exit");
  await page.keyboard.press("Enter");

  await command(page, "ws_add", {machine: "", path: workspace});
  await page.locator('[data-landing="workspaces"]:visible').first().click();
  await page.locator("[data-ws-row]:visible").filter({hasText: basename(workspace)}).click();
  await page.locator('[data-file-tree] button[data-file-path]:visible').filter({hasText: "readme.md"}).click();
  await page.locator('[data-file-face-toggle="source"]:visible').click();
  const preview = page.getByText("Khor installation workspace", {exact: true});
  await preview.waitFor();
  assert.equal(await windowButtons.count(), 0, "Workspace panes inherit the native caption");
  await checkBrowser(page, navigation);
  await command(page, "window_event_unlisten", {eventId});
  await page.evaluate(id => window.__TAURI_INTERNALS__.unregisterCallback(id), callback);

  // The installer checker closes the native window after CDP disconnects.
  console.log("Installed Windows App verified a live terminal, workspace file and child browser.");
} finally {
  await browser.close();
}
