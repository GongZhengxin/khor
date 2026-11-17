// End-to-end close check through the road the owner actually presses:
//   1. start the app
//   2. find the page's own close control (data-window-control="close")
//   3. press it through CDP
//   4. require the process to exit and its ports to be free
//
// Run with: node scripts/windows-close-cli-check.js <path-to-khor-gui.exe>
import http from 'node:http';
import { spawn, execFileSync } from 'node:child_process';

const EXE = process.argv[2] || 'F:\\Temp\\khor-target\\release\\khor-gui.exe';
const PORT = 9223;

const sleep = ms => new Promise(r => setTimeout(r, ms));

function listTargets() {
  return new Promise((resolve, reject) => {
    const req = http.get(`http://127.0.0.1:${PORT}/json/list`, r => {
      let d = '';
      r.on('data', c => (d += c));
      r.on('end', () => {
        try {
          resolve(JSON.parse(d));
        } catch (e) {
          reject(e);
        }
      });
    });
    req.on('error', reject);
    req.setTimeout(4000, () => req.destroy(new Error('cdp list timeout')));
  });
}

function portsBusy() {
  const out = execFileSync('powershell.exe', [
    '-NoProfile',
    '-Command',
    "(Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object { $_.LocalPort -in 1432,5467 } | Measure-Object).Count",
  ]).toString().trim();
  return Number(out) || 0;
}

function alive(pid) {
  // **`execFileSync` succeeding says the shell ran, not that the process
  // lives** — the answer has to be read from its output. (The first version
  // of this check returned true unconditionally and "proved" that the close
  // button was broken after the fix.)
  const out = execFileSync('powershell.exe', [
    '-NoProfile',
    '-Command',
    `if (Get-Process -Id ${pid} -ErrorAction SilentlyContinue) { 'yes' } else { 'no' }`,
  ]).toString().trim();
  return out === 'yes';
}

async function main() {
  // A clean slate: no other khor may hold the ports.
  try {
    execFileSync('powershell.exe', ['-NoProfile', '-Command',
      "Get-Process khor-gui,khor -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue"]);
  } catch {}
  await sleep(2000);
  const before = portsBusy();
  console.log(`ports busy before start: ${before}`);

  const app = spawn(EXE, [], {
    detached: true,
    stdio: 'ignore',
    env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${PORT}` },
  });
  app.unref();
  const pid = app.pid;
  console.log(`started pid ${pid}`);

  // Wait for the page to exist and to have drawn its window controls.
  let page = null;
  for (let i = 0; i < 60; i++) {
    await sleep(1000);
    try {
      const targets = await listTargets();
      page = targets.find(t => t.type === 'page' && (t.url || '').includes('tauri.localhost'));
      if (page) break;
    } catch {}
  }
  if (!page) throw new Error('the app never exposed its page');
  console.log(`page: ${page.url}`);

  const ws = new WebSocket(page.webSocketDebuggerUrl);
  let id = 0;
  const waiting = new Map();
  const send = (method, params) => {
    const mine = ++id;
    ws.send(JSON.stringify({ id: mine, method, params }));
    return new Promise((res, rej) => {
      waiting.set(mine, res);
      setTimeout(() => rej(new Error('timeout ' + method)), 15000);
    });
  };
  ws.addEventListener('message', ev => {
    const msg = JSON.parse(ev.data);
    if (msg.id && waiting.has(msg.id)) {
      waiting.get(msg.id)(msg);
      waiting.delete(msg.id);
    }
  });
  await new Promise((res, rej) => {
    ws.addEventListener('open', res);
    ws.addEventListener('error', rej);
  });
  const evaluate = async expression => {
    const r = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
    return r.result?.result?.value;
  };

  // The controls are drawn after the first frame; wait for one to exist.
  let close = null;
  for (let i = 0; i < 30; i++) {
    const found = await evaluate(`
      (() => {
        const el = document.querySelector('[data-window-control="close"]');
        if (!el) return '';
        const r = el.getBoundingClientRect();
        if (!r.width || !r.height) return '';
        return JSON.stringify({ x: Math.round(r.x + r.width / 2), y: Math.round(r.y + r.height / 2) });
      })()
    `);
    if (found) {
      close = JSON.parse(found);
      break;
    }
    await sleep(700);
  }
  if (!close) throw new Error('the page never drew a close control');
  console.log(`close control at ${close.x},${close.y}`);

  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: close.x, y: close.y, button: 'none' });
  await sleep(120);
  await send('Input.dispatchMouseEvent', {
    type: 'mousePressed', x: close.x, y: close.y, button: 'left', buttons: 1, clickCount: 1,
  });
  await sleep(90);
  await send('Input.dispatchMouseEvent', {
    type: 'mouseReleased', x: close.x, y: close.y, button: 'left', buttons: 0, clickCount: 1,
  });
  console.log('pressed the close control');

  let gone = false;
  for (let i = 0; i < 25; i++) {
    await sleep(500);
    if (!alive(pid)) {
      gone = true;
      break;
    }
  }
  const after = portsBusy();
  console.log(`process gone: ${gone}`);
  console.log(`ports busy after close: ${after}`);

  try { ws.close(); } catch {}
  if (!gone) throw new Error(`the close control did not end pid ${pid} (left running for inspection)`);
  if (after !== 0) throw new Error(`the process exited but ${after} port(s) are still held`);
  console.log('PASS: the close control ends the app and frees its ports');
}

main().catch(e => {
  console.error('FAIL:', e.message);
  process.exit(1);
});
