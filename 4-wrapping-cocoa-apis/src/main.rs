/// An example of wrapping an Objective-C class before interacting with it
/// in Rust. In this case, we wrap NSString (using the `impl_objc_class`
/// macro) and then add a few new functions which we want to use.

#[macro_use]
extern crate objc;
extern crate libc;

#[macro_use]
mod objc_class;

use objc::runtime::{Object,Class,YES,BOOL};
use objc_class::{Id, ObjCClass};

// Link the frameworks used, otherwise classes won't be loaded
#[link(name = "Foundation", kind = "framework")]
extern {}

// Create types for Objective-C values which have different sizes based on
// pointer size
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

// The value of NSUTF8StringEncoding
const UTF8_ENCODING: NSUInteger = 4;

impl_objc_class!(NSString);
impl NSString {

    /// Create a new empty `NSString`
    ///
    /// ## Examples
    ///
    /// ```
    /// let string = NSString::new();
    /// assert_eq!(0, string.len());
    /// ```
    pub fn new() -> Self {
        NSString { ptr: unsafe { msg_send![class!("NSString"), new] } }
    }

    /// Creates an `NSString` from a `str`.
    ///
    /// ## Examples
    ///
    /// ```
    /// let string = NSString::from("hello");
    /// assert_eq!("hello", string.as_str().unwrap());
    /// ```
    pub fn from(content: &str) -> Self {
        let ptr: *mut Object = unsafe {
            let string: *mut Object = msg_send![class!("NSString"), alloc];
            msg_send![string, initWithBytes:content.as_ptr()
                                     length:content.len()
                                   encoding:UTF8_ENCODING]
        };
        NSString { ptr: ptr }
    }

    /// The length of the string as measured in UTF-8 code points
    ///
    /// ## Examples
    ///
    /// ```
    /// assert_eq!(0, NSString::new().len());
    /// assert_eq!(5, NSString::from("hello").len());
    /// ```
    pub fn len(&self) -> usize {
        unsafe { msg_send![self.ptr, lengthOfBytesUsingEncoding:UTF8_ENCODING] }
    }
}

fn main() {
    assert_eq!(NSString::from("hello").len(), 5);
    assert_eq!(NSString::from("hello"), NSString::from("hello"));
    assert_eq!(NSString::new(), NSString::new());
    assert!(NSString::from("bat") != NSString::from("bot"));
    println!("Tests passed!");
}
