/// An example of declaring an Objective-C class from Rust, and using it
/// from either language. In this case, we create a custom class which
/// prints an important message.

#[macro_use]
extern crate objc;

#[macro_use]
mod objc_class;
mod foundation;

use objc::runtime::{Object,Class,YES,BOOL,Sel};
use objc::declare::ClassDecl;
use objc_class::{Id, ObjCClass};
use foundation::NSString;

impl_objc_class!(FruitDetector);
impl FruitDetector {
    pub fn new() -> Self {
        FruitDetector {
            ptr: unsafe { msg_send![class!("FruitDetector"), new] }
        }
    }

    fn inspect(&self, text: &str) {
        let obj = unsafe { &mut *(self.ptr as *mut _ as *mut Object) };
        inspect(obj, sel!(inspect:), NSString::from(text).ptr());
    }
}

fn main() {
    declare_objc_classes();
    inspect_from_rust("pear");
    inspect_from_rust("apple");
    inspect_from_objc("pear");
    inspect_from_objc("apple");
}

fn inspect_from_rust(fruit: &str) {
    println!("from Rust:");
    let detector = FruitDetector::new();
    detector.inspect(fruit);
}

fn inspect_from_objc(fruit: &str) {
    println!("from Objective-C:");
    unsafe {
        let detector: Id = msg_send![class!("FruitDetector"), new];
        let fruit = NSString::from(fruit);
        msg_send![detector, inspect:fruit.ptr()];
    }
}

/// Register a custom class with the Objective-C runtime
fn declare_objc_classes() {
    let mut cls = ClassDecl::new(FruitDetector::class_name(), class!("NSObject")).unwrap();
    unsafe {
        cls.add_method(sel!(inspect:), inspect as extern fn (&Object, Sel, Id));
    }
    cls.register();
}

extern fn inspect(_: &Object, _cmd: Sel, text: Id) {
    if let Some(text) = NSString::from_ptr(text) {
        if text == NSString::from("apple") {
            println!("This is an apple.");
        } else {
            println!("This is not an apple.");
        }
    }
}
