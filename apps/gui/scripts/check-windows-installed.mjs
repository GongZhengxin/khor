// Called by the Windows installer check after opening the packaged app
// and creating a local cmd.exe session in an isolated store.
import { setTimeout as delay } from "node:timers/promises";
import { chromium } from "playwright-core";

const [, , port, title, ready] = process.argv;
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

await waitForWindow();
const browser = await chromium.connectOverCDP(endpoint, {timeout: 15_000});
try {
  const page = browser.contexts().flatMap(context => context.pages())
    .find(page => new URL(page.url()).origin === origin);
  if (!page) throw new Error("The bundled frontend is missing from the WebView2 session");
  page.setDefaultTimeout(20_000);

  await page.locator('[data-landing="sessions"]:visible').first().click();
  await page.locator('[data-pane-filter]:visible').click();
  const conversations = page.locator('[data-filter-option="talk:yes"]');
  await conversations.waitFor();
  if (await conversations.getAttribute("aria-checked") === "true") await conversations.click();
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
  await page.keyboard.type("exit");
  await page.keyboard.press("Enter");

  // The window closing also disconnects CDP. Schedule the original button
  // handler, then let the installer check confirm that the process exited.
  await Promise.all([
    page.waitForEvent("close"),
    page.locator('[data-window-control="close"]:visible').first()
      .evaluate(button => { setTimeout(() => button.click(), 100); }),
  ]);
  console.log("Installed Windows App opened a local session and displayed a live terminal response.");
} finally {
  await browser.close();
}
