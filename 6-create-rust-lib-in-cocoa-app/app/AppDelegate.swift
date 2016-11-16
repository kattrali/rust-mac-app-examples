import Cocoa

@NSApplicationMain
class AppDelegate: NSObject, NSApplicationDelegate, NSTextFieldDelegate {

    @IBOutlet var window: NSWindow!
    @IBOutlet weak var textField: NSTextField!

    @IBOutlet weak var cubeLabel: NSTextField!
    @IBOutlet weak var squareLabel: NSTextField!

    func applicationDidFinishLaunching(_ notification: Notification) {
    }

    private func applicationShouldTerminate(afterLastWindowClosed sender: NSApplication) -> Bool {
        return true
    }

    override func controlTextDidChange(_ obj: Notification) {
        cubeLabel.stringValue = String(format: "%d", cube(Int32(textField.integerValue)))
        squareLabel.stringValue = String(format: "%d", square(Int32(textField.integerValue)))
    }
}
