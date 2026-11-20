param(
    [Parameter(Mandatory)][long]$Window,
    [Parameter(Mandatory)][int]$ClientX,
    [Parameter(Mandatory)][int]$ClientY,
    [Parameter(Mandatory)][string]$Path
)

$ErrorActionPreference = 'Stop'
if ($env:GITHUB_ACTIONS -ne 'true') {
    throw 'Pointer automation requires an isolated Windows runner'
}

Add-Type -ReferencedAssemblies System.Windows.Forms.dll -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Windows.Forms;

public static class InstallerFileDrag
{
    [StructLayout(LayoutKind.Sequential)]
    private struct Point { public int X; public int Y; }

    [StructLayout(LayoutKind.Sequential)]
    private struct MouseInput
    {
        public int X;
        public int Y;
        public uint Wheel;
        public uint Flags;
        public uint Time;
        public UIntPtr ExtraInfo;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct Input { public uint Type; public MouseInput Mouse; }

    [DllImport("user32.dll")] private static extern bool ClientToScreen(IntPtr window, ref Point point);
    [DllImport("user32.dll")] private static extern bool SetForegroundWindow(IntPtr window);
    [DllImport("user32.dll")] private static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] private static extern IntPtr WindowFromPoint(Point point);
    [DllImport("user32.dll")] private static extern IntPtr GetAncestor(IntPtr window, uint flags);
    [DllImport("user32.dll")] private static extern IntPtr SetThreadDpiAwarenessContext(IntPtr context);
    [DllImport("user32.dll", SetLastError = true)]
    private static extern uint SendInput(uint count, Input[] input, int size);

    private static void Press(uint flags)
    {
        var mouse = new Input { Mouse = new MouseInput { Flags = flags } };
        if (SendInput(1, new[] { mouse }, Marshal.SizeOf(typeof(Input))) != 1)
            throw new InvalidOperationException("Windows refused the drag pointer input");
    }

    public static void Drop(IntPtr window, int x, int y, string path)
    {
        SetThreadDpiAwarenessContext(new IntPtr(-4)); // Per-monitor aware v2
        SetForegroundWindow(window);
        var point = new Point { X = x, Y = y };
        if (!ClientToScreen(window, ref point) || GetAncestor(WindowFromPoint(point), 2) != window)
            throw new InvalidOperationException("The drag target is outside the installed Khor window");
        using (var source = new Control())
        {
            var data = new DataObject(DataFormats.FileDrop, new[] { path });
            if (!SetCursorPos(point.X - 18, point.Y))
                throw new InvalidOperationException("The Windows runner has no active pointer");
            Press(0x0002); // Left down.
            Exception gestureError = null;
            var gesture = new Thread(() =>
            {
                try
                {
                    Thread.Sleep(400);
                    if (!SetCursorPos(point.X, point.Y))
                        throw new InvalidOperationException("Windows refused to move the drag pointer");
                    Thread.Sleep(400);
                    Press(0x0004); // Left up, which commits the OLE drop.
                }
                catch (Exception error) { gestureError = error; }
            });
            gesture.IsBackground = true;
            gesture.Start();
            DragDropEffects accepted;
            try { accepted = source.DoDragDrop(data, DragDropEffects.Copy); }
            finally
            {
                gesture.Join();
                Press(0x0004);
            }
            if (gestureError != null) throw gestureError;
            if (accepted != DragDropEffects.Copy)
                throw new InvalidOperationException("The installed App did not accept the native file drop");
        }
    }
}
'@

[InstallerFileDrag]::Drop([IntPtr]::new($Window), $ClientX, $ClientY, (Resolve-Path -LiteralPath $Path).Path)
