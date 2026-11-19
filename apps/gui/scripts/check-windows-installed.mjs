// Called by the Windows installer check after opening the packaged app
// and creating a local cmd.exe session in an isolated store.
import assert from "node:assert/strict";
import { once } from "node:events";
import { createServer } from "node:http";
import { basename } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { chromium } from "playwright-core";

const [, , port, title, ready, workspace] = process.argv;
const endpoint = `http://127.0.0.1:${port}`;
const origin = "http://tauri.localhost";

async function waitForWindow() {
  const until = Date.now() + 30_000;
  while (Date.now() < until) {
    const response = await fetch(`${endpoint}/json/list`, {signal: AbortSignal.timeout(1000)})
      .catch(() => null);
    if (response?.ok) {
      const targets = await response.json();
      if (targets.some(target => target.type === "page" && new URL(target.url).origin === origin)) return;
    }
    await delay(200);
  }
  throw new Error("The installed App never loaded its bundled frontend in WebView2");
}

function command(page, name, args) {
  return page.evaluate(async ({name, args}) => {
    const transport = window.__KHOR_WINDOW_TRANSPORT__;
    const response = await fetch(`${transport.url}/${name}`, {
      method: "POST",
      headers: {"Content-Type": "application/json", "X-Khor-Window-Token": transport.token},
      body: JSON.stringify(args),
      signal: AbortSignal.timeout(15_000),
    });
    if (!response.ok) throw new Error(`${name} returned ${response.status}: ${await response.text()}`);
    return response.json();
  }, {name, args});
}

async function checkBrowser(page) {
  const fixture = createServer((_request, response) => {
    response.writeHead(200, {"Content-Type": "text/html; charset=utf-8", "Connection": "close"});
    response.end("<!doctype html><title>Khor installer browser</title>");
  });
  const tab = "windows-installer-browser";
  fixture.listen(0, "127.0.0.1");
  await once(fixture, "listening");
  try {
    const url = `http://127.0.0.1:${fixture.address().port}/`;
    let reached = false;
    fixture.on("request", () => { reached = true; });
    const view = await page.evaluate(() => innerHeight);
    const agent = await page.evaluate(() => navigator.userAgent);
    await command(page, "open_page", {tab, machine: "", url, agent, at: {x: 280, y: 75, w: 540, h: 400, view}});
    const until = Date.now() + 15_000;
    let state;
    do {
      state = await command(page, "page_state", {tab});
      if (reached && ["finished", "settled"].includes(state?.phase)) break;
      await delay(200);
    } while (Date.now() < until);
    assert(reached, "The native child WebView must request the fixture URL");
    assert(["finished", "settled"].includes(state?.phase), `The child WebView did not finish: ${JSON.stringify(state)}`);
    await command(page, "close_page", {tab});
    assert.equal(await command(page, "page_state", {tab}), null, "Closing retires the native child WebView");
  } finally {
    fixture.close();
  }
}

await waitForWindow();
const browser = await chromium.connectOverCDP(endpoint, {timeout: 15_000});
try {
  const page = browser.contexts().flatMap(context => context.pages())
    .find(page => new URL(page.url()).origin === origin);
  if (!page) throw new Error("The bundled frontend is missing from the WebView2 session");
  page.setDefaultTimeout(20_000);
  const windowButtons = page.locator("[data-window-controls], [data-window-control]");

  // Drag and drop, navigation progress, and close requests use Tauri's
  // event plugin and do not travel over the app's HTTP command bridge.
  await page.evaluate(async () => {
    const native = window.__TAURI_INTERNALS__;
    const event = "installer:probe";
    const handler = native.transformCallback(() => {});
    const timeout = AbortSignal.timeout(12_000);
    const stopped = new Promise((_, reject) => timeout.addEventListener("abort", () =>
      reject(new Error("WebView2 did not answer the native event subscription"))));
    try {
      const eventId = await Promise.race([
        native.invoke("plugin:event|listen", {event, target: {kind: "Any"}, handler}),
        stopped,
      ]);
      if (!Number.isInteger(eventId)) throw new Error("Native event subscription returned no listener ID");
      await Promise.race([
        native.invoke("plugin:event|unlisten", {event, eventId}),
        stopped,
      ]);
    } finally {
      native.unregisterCallback(handler);
    }
  });

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
  await checkBrowser(page);

  // The installer checker closes the native window after CDP disconnects.
  console.log("Installed Windows App verified a live terminal, workspace file and child browser.");
} finally {
  await browser.close();
}
