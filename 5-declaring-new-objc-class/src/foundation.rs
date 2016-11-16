extern crate libc;

use objc::runtime::{Object,Class,YES,BOOL};
use objc_class::{Id, ObjCClass};

// Create types for Objective-C values which have different sizes based on
// pointer size
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

// Link the frameworks used, otherwise classes won't be loaded
#[link(name = "Foundation", kind = "framework")]
extern {}

// The value of NSUTF8StringEncoding
const UTF8_ENCODING: NSUInteger = 4;

impl_objc_class!(NSString);
impl NSString {

    /// Creates an `NSString` from a `str`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use foundation::NSString;
    ///
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
}
