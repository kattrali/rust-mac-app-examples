/// Example of using Objective-C from Rust. Creates a NSNumber instance and
/// prints its description.

#[macro_use]
extern crate objc;
extern crate libc;

use std::{str,slice};
use objc::runtime::{Object, Class};


const UTF8_ENCODING: libc::c_uint = 4;

// Link the frameworks used, otherwise classes won't be loaded
#[link(name = "Foundation", kind = "framework")]
extern {}

fn main() {
    unsafe {
        let cls = Class::get("NSNumber").unwrap();
        let obj: *mut Object = msg_send![cls, numberWithInteger:34];
        describe(obj);
        msg_send![obj, release];
    }
}

/// Get and print an objects description
unsafe fn describe(obj: *mut Object) {
    let description: *mut Object = msg_send![obj, description];
    if let Some(desc_str) = to_s(description) {
        println!("Object description: {}", desc_str);
    }
}

/// Convert an NSString to a String
fn to_s<'a>(nsstring_obj: *mut Object) -> Option<&'a str> {
    let bytes = unsafe {
        let length = msg_send![nsstring_obj, lengthOfBytesUsingEncoding:UTF8_ENCODING];
        let utf8_str: *const u8 = msg_send![nsstring_obj, UTF8String];
        slice::from_raw_parts(utf8_str, length)
    };
    str::from_utf8(bytes).ok()
}
