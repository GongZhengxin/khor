// Diagnostic-only: production CSS in a transparent native WKWebView, over
// two controlled native backdrops. No user app, preference or session touched.
import AppKit
import WebKit
import Darwin

@MainActor final class Probe: NSObject, NSApplicationDelegate, WKNavigationDelegate {
    var stage: NSWindow!
    var front: NSWindow!
    var web: WKWebView!
    let output = URL(fileURLWithPath: CommandLine.arguments[2], isDirectory: true)
    var results: [[String: Any]] = []
    func applicationDidFinishLaunching(_ note: Notification) {
        NSApp.setActivationPolicy(.regular)
        let area = NSScreen.main!.visibleFrame
        let rect = NSRect(x: area.minX + 60, y: area.minY + 80, width: 600, height: 420)
        stage = NSWindow(contentRect: rect.insetBy(dx: -30, dy: -30), styleMask: [.borderless], backing: .buffered, defer: false)
        stage.backgroundColor = .magenta
        stage.orderFrontRegardless()
        front = NSWindow(contentRect: rect, styleMask: [.borderless], backing: .buffered, defer: false)
        front.isOpaque = false
        front.backgroundColor = .clear
        front.hasShadow = false
        web = WKWebView(frame: NSRect(origin: .zero, size: rect.size))
        web.setValue(false, forKey: "drawsBackground")
        web.navigationDelegate = self
        front.contentView = web
        front.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
        web.loadFileURL(URL(fileURLWithPath: CommandLine.arguments[1]), allowingReadAccessTo: URL(fileURLWithPath: CommandLine.arguments[1]).deletingLastPathComponent())
    }
    func navigation(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        Task { await run() }
    }
    func pause() async { try? await Task.sleep(nanoseconds: 900_000_000) }
    func js(_ value: String) async -> Any? { try? await web.evaluateJavaScript(value) }
    func image(_ file: URL) -> [String: Any] {
        guard let bitmap = NSBitmapImageRep(contentsOf: file) else { return ["error": "screenshot unavailable"] }
        let scale = Double(bitmap.pixelsWide) / 600
        var samples: [String: [Double]] = [:]
        for (name, point) in [("glass", NSPoint(x: 100, y: 55)), ("selected", NSPoint(x: 300, y: 55)), ("control", NSPoint(x: 100, y: 155)), ("band", NSPoint(x: 100, y: 255)), ("canvas", NSPoint(x: 100, y: 355))] {
            if let c = bitmap.colorAt(x: Int(point.x * scale), y: Int(point.y * scale))?.usingColorSpace(.deviceRGB) {
                samples[name] = [c.redComponent, c.greenComponent, c.blueComponent, c.alphaComponent]
            }
        }
        return ["width": bitmap.pixelsWide, "height": bitmap.pixelsHigh, "samples": samples]
    }
    func capture(_ name: String) -> [String: Any] {
        let file = output.appendingPathComponent(name + ".png")
        let r = front.frame
        let top = NSScreen.screens.map { $0.frame.maxY }.max()! - r.maxY
        let p = Process()
        p.executableURL = URL(fileURLWithPath: "/usr/sbin/screencapture")
        p.arguments = ["-x", "-R\(Int(r.minX)),\(Int(top)),\(Int(r.width)),\(Int(r.height))", file.path]
        do { try p.run(); p.waitUntilExit() } catch { return ["error": String(describing: error)] }
        var value = image(file); value["exit"] = p.terminationStatus
        return value
    }
    func run() async {
        for dark in [false, true] {
            _ = await js("document.documentElement.classList.toggle('dark',\(dark))")
            for fallback in [false, true] {
                _ = await js("document.documentElement.toggleAttribute('data-probe-native-backdrop',\(fallback))")
                for (name, color) in [("magenta", NSColor.magenta), ("cyan", NSColor.cyan)] {
                    stage.backgroundColor = color
                    await pause()
                    let key = "\(dark ? "dark" : "light")-\(fallback ? "off" : "on")-\(name)"
                    let style = await js("JSON.stringify({glass:getComputedStyle(document.querySelector('.glass')).backdropFilter,webkit:getComputedStyle(document.querySelector('.glass')).webkitBackdropFilter,background:getComputedStyle(document.querySelector('.glass')).backgroundColor})") as? String ?? ""
                    results.append(["case": key, "capture": capture(key), "style": style])
                }
            }
        }
        let report: [String: Any] = ["os": ProcessInfo.processInfo.operatingSystemVersionString, "results": results, "limits": ["Native WKWebView and exact released CSS, not the installed Tauri app", "Controlled flat native backdrop; native window frost not simulated"]]
        do {
            try JSONSerialization.data(withJSONObject: report, options: [.prettyPrinted, .sortedKeys]).write(to: output.appendingPathComponent("result.json"))
        } catch { print(error); exit(1) }
        NSApp.terminate(nil)
    }
}
let app = NSApplication.shared
let probe = Probe()
app.delegate = probe
app.run()
