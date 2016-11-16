# Using Rust in Cocoa apps

Examples for working with the Objective-C runtime from Rust.

Each example is annotated and can be run using `cargo run` or using the
specialized instructions below, if any.

## Examples

1. [Creating Objective-C objects from Rust](1-creating-objc-objects):
   Creates an object and sends it a message

2. [Displaying a Cocoa Window](2-displaying-cocoa-window):
   Creates and presents a Cocoa app with a single window, leveraging the `cocoa`
   crate

3. [Packaging a macOS app](3-packaging-a-mac-app):
   Compiles and runs a Rust binary as a part of a macOS app. The Rust binary is
   launched as the primary process and handles presenting the user interface.
   Run the application by opening the Xcode project in the `app/` directory of
   the example and using the Run button.

4. [Wrapping Cocoa APIs](4-wrapping-cocoa-apis):
   Creates a custom interface in Rust to accessing Cocoa classes (somewhat)
   safely, or at least without `unsafe` blocks

5. [Declaring a new Objective-C class from Rust](5-declaring-new-objc-class):
   Creates and registers an Objective-C class with the runtime from Rust, as
   well as sending messages to an instance from both Rust and Objective-C

6. [Include a Rust library in a Cocoa app](6-create-rust-lib-in-cocoa-app):
   Create a static library in Rust, and bundles/links it with a mac app. The mac
   app target in Xcode depends on an external target which is the Rust library,
   and the C header for the Rust library is used to invoke Rust functions from
   Swift. Run the application by opening the Xcode project and using the Run
   button.
