import path from "node:path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// KaTeX's sheet names three files per font (woff2, woff, ttf) and the
// bundler would ship all three — about 0.8 MB the binary carries and no
// engine this app runs in ever reads. Only the woff2 stays (lane
// md-render, `lib/md/math`).
const katexWoff2Only = {
  name: "katex-woff2-only",
  enforce: "pre" as const,
  transform(code: string, id: string) {
    if (!/katex(\.min)?\.css$/.test(id)) return null;
    return code.replace(/,\s*url\([^)]*\.(?:woff|ttf)\)\s*format\("(?:woff|truetype)"\)/g, "");
  },
};

// 1430, not tauri's usual 1420: mandala's dev server owns 1420 on this
// machine and the two must be able to run side by side.
export default defineConfig({
  plugins: [react(), tailwindcss(), katexWoff2Only],
  resolve: { alias: { "@": path.resolve(import.meta.dirname, "src") } },
  server: { port: 1430, strictPort: true },
  // Browser gates exercise the app. Experimental lab entry points may
  // reference retired APIs and must not restart that app mid-gesture.
  optimizeDeps: process.env.KHOR_FACE_GATE ? { entries: ["index.html"] } : undefined,
  // **One chunk, because a second one cannot be fetched on Windows**
  // (measured 2026-11-16). The desktop window loads its page over Tauri's
  // custom asset protocol, and that protocol stops answering after the startup
  // burst: a `fetch('/assets/<anything>')` from the running page never settles,
  // exactly like the IPC `fetch` it shares a transport with. Code-splitting
  // therefore leaves most of the app unreachable — the terminal panel is lazy
  // (`views/DetailPane` `lazy(() => import("@/views/TerminalPane"))`), so
  // opening a session drew an empty pane and no `term_open` was ever sent,
  // with no error anywhere. Inlining the dynamic imports puts every module the
  // page can ask for in the file it already has, so nothing has to be fetched
  // after the first load.
  //
  // The cost is a larger first load, paid once at launch; the alternative is a
  // face whose second half never arrives. Revisit if Tauri's asset transport is
  // ever fixed upstream.
  build: { rollupOptions: { output: { inlineDynamicImports: true } } },
  clearScreen: false,
});
