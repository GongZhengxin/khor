import { useEffect, useLayoutEffect, useRef, useState, type RefObject } from 'react';

type OrderedRow = { id: string; group: string };
/** Freeze positions, never row data, while a person is operating the list. */
export function useStableListOrder<T extends OrderedRow>(rows: T[], root: RefObject<HTMLDivElement | null>, enabled: boolean, filter: string): T[] {
  const [paused, setPaused] = useState(false);
  const positions = useRef<{ filter: string; rows: { id: string; group: string }[] }>({ filter, rows: [] });
  const anchor = useRef<{ scroller: HTMLElement; id: string; offset: number } | null>(null);
  const remember = () => {
    const box = root.current;
    if (!box) return;
    let scroller: HTMLElement | null = box.parentElement;
    while (scroller && !/auto|scroll/.test(getComputedStyle(scroller).overflowY)) scroller = scroller.parentElement;
    if (!scroller || scroller.scrollTop <= 0) { anchor.current = null; return; }
    const top = scroller.getBoundingClientRect().top;
    const first = [...box.querySelectorAll<HTMLElement>('[data-side-row]')].find(row => row.getBoundingClientRect().bottom > top);
    if (first?.dataset.sideRow) anchor.current = { scroller, id: first.dataset.sideRow, offset: first.getBoundingClientRect().top - top };
  };
  useEffect(() => {
    if (!enabled) return;
    let held = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const inside = (target: EventTarget | null) => target instanceof Node && root.current?.contains(target);
    const resume = () => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        if (held || document.querySelector('[role="menu"][data-state="open"]')) { resume(); return; }
        setPaused(false);
      }, 300);
    };
    const suspend = () => { remember(); setPaused(true); resume(); };
    const scroll = (event: Event) => {
      const target = event.target;
      if (inside(target) || target instanceof Node && target.contains(root.current)) suspend();
    };
    const down = (event: PointerEvent) => { if (inside(event.target)) { held = true; suspend(); } };
    const up = () => { held = false; resume(); };
    const key = (event: KeyboardEvent) => { if (inside(event.target)) suspend(); };
    window.addEventListener('scroll', scroll, true);
    window.addEventListener('pointerdown', down, true);
    window.addEventListener('pointerup', up, true);
    window.addEventListener('pointercancel', up, true);
    window.addEventListener('keydown', key, true);
    return () => {
      clearTimeout(timer);
      window.removeEventListener('scroll', scroll, true);
      window.removeEventListener('pointerdown', down, true);
      window.removeEventListener('pointerup', up, true);
      window.removeEventListener('pointercancel', up, true);
      window.removeEventListener('keydown', key, true);
    };
  }, [enabled, root]);
  let ordered = rows;
  if (enabled && paused && positions.current.filter === filter) {
    const current = new Map(rows.map(row => [row.id, row]));
    ordered = positions.current.rows.flatMap(previous => {
      const row = current.get(previous.id);
      current.delete(previous.id);
      return row ? [{ ...row, group: previous.group }] : [];
    });
    // New rows wait until interaction ends; do not split a frozen group.
  } else positions.current = { filter, rows: rows.map(({ id, group }) => ({ id, group })) };
  const identity = ordered.map(row => `${row.group}\0${row.id}`).join('\n');
  const previous = useRef(identity);
  useLayoutEffect(() => {
    if (enabled && previous.current !== identity && anchor.current) {
      const { scroller, id, offset } = anchor.current;
      const row = [...(root.current?.querySelectorAll<HTMLElement>('[data-side-row]') ?? [])].find(row => row.dataset.sideRow === id);
      if (row) scroller.scrollTop += row.getBoundingClientRect().top - scroller.getBoundingClientRect().top - offset;
    }
    previous.current = identity;
    remember();
  });
  return ordered;
}
