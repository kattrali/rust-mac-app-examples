import Cocoa

func log(_ data: Data) {
    if let message = NSString(data: data, encoding: String.Encoding.utf8.rawValue) {
        NSLog("%@", message)
    }
}

let task = Process()
let bundle = Bundle.main
let rustBinName = bundle.infoDictionary?["RustBinaryName"] as! String
task.launchPath = bundle.path(forResource: rustBinName, ofType: nil)
task.environment = ["RUST_BACKTRACE": "1"]

let stdOut = Pipe()
let stdErr = Pipe()

stdOut.fileHandleForReading.readabilityHandler = { log($0.availableData) }
stdErr.fileHandleForReading.readabilityHandler = { log($0.availableData) }

task.standardOutput = stdOut
task.standardError = stdErr

task.terminationHandler = { task in
    (task.standardOutput as AnyObject?)?.fileHandleForReading.readabilityHandler = nil
    (task.standardError as AnyObject?)?.fileHandleForReading.readabilityHandler = nil
}

task.launch()
task.waitUntilExit()
