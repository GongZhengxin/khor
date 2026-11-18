import assert from 'node:assert/strict';
import path from 'node:path';

export async function checkSessionAttention(browser, { port, report, engine, width }) {
  const context = await browser.newContext({ viewport: { width, height: 900 } });
  const page = await context.newPage(); page.setDefaultTimeout(20000);
  const errors = []; page.on('pageerror', error => errors.push(String(error)));
  const api = (command, body = {}) => page.evaluate(async ({command, body}) => {
    const response = await fetch('/api/'+command, {method:'POST',body:JSON.stringify(body)});
    if (!response.ok) throw Error(await response.text());
    return response.json();
  }, {command, body});
  try {
    await page.goto(`http://127.0.0.1:${port}/app/?scene=list&items=5&machines=3`);
    const list = page.locator('[data-list] [data-side-list]').first();
    await list.locator('[data-side-row]').first().waitFor();
    const ids = () => list.locator('[data-side-row]').evaluateAll(rows => rows.map(row => row.dataset.sideRow));
    const initial = await ids(); assert(initial.length >= 5);
    const [pinned, one, two, three, four] = initial;
    for (const id of initial) await api('__ai_preview/session-state', {id,word:'idle'});
    await api('__ai_preview/session-state', {id:two,word:'busy'});
    await api('__ai_preview/session-state', {id:three,word:'errored'});
    await api('session_draft_edit', {revision:{id:'attention-seed',key:(four.startsWith('wide/')?'khor:':'gui:')+four,text:'unsubmitted draft',at_ms:Date.now()},parents:[]});
    await page.waitForFunction(({three,two}) => {
      const rows=[...document.querySelectorAll('[data-list] [data-side-row]')].map(row=>row.dataset.sideRow);
      return rows.indexOf(three)<rows.indexOf(two);
    },{three,two});
    await list.locator('[data-session-draft]').waitFor();
    assert.match(await list.locator(`[data-side-row="${four}"]`).innerText(), /\[草稿\]unsubmitted draft/);
    assert.equal((await ids())[0],pinned,'A pinned row remains above ordinary attention');
    const before = await ids();
    const target = list.locator(`[data-side-row="${one}"]`), bounds = await target.boundingBox();
    await page.mouse.move(bounds.x+bounds.width/2,bounds.y+bounds.height/2);await page.mouse.down();
    await api('__ai_preview/session-state',{id:one,word:'blocked'});
    await page.waitForTimeout(2600);
    assert.deepEqual(await ids(),before,'An update cannot move a row out from under a held pointer');
    await page.mouse.move(1,899);
    await page.mouse.up();
    await page.waitForFunction(({one}) => document.querySelectorAll('[data-list] [data-side-row]')[1]?.getAttribute('data-side-row')===one,{one});
    await page.screenshot({path:path.join(report,`attention-${engine}-${width}.png`)});
    assert.deepEqual(errors,[]);
    return {name:`${engine} ${width}: draft preview, shared attention and held-pointer stability`,passed:true};
  } finally { await context.close(); }
}
