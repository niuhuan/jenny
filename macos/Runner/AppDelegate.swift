import Cocoa
import FlutterMacOS

@NSApplicationMain
class AppDelegate: FlutterAppDelegate {
    
    override init() {
        super.init()
        let folder = NSHomeDirectory()+"/Library/Application Support/jenny"
        let chars = folder.cString(using: String.Encoding.utf8)
        init_ffi(chars!)
    }
    
    override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
    
    override func applicationWillFinishLaunching(_ notification: Notification)   {
        super.applicationWillFinishLaunching(notification)
        let width = load_int_property("window_width", 600);
        let height = load_int_property("window_height", 800);
        mainFlutterWindow?.setContentSize(NSSize.init(width: CGFloat(max(width,50)), height: CGFloat(max(height,50))));
    }
    
}
