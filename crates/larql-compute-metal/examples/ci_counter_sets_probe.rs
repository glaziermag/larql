//! Fork-only CI probe: does `[MTLDevice counterSets]` return nil on this
//! runner? (`Device::counter_sets()` in `metal` 0.29 dereferences the
//! array without a nil check, so it cannot be asked safely.)

use metal::foreign_types::ForeignType;
use objc::{msg_send, sel, sel_impl};

fn main() {
    let Some(device) = metal::Device::system_default() else {
        eprintln!("no Metal device");
        std::process::exit(2);
    };
    println!("device: {}", device.name());
    println!(
        "MetalBackend::new(): {}",
        if larql_compute_metal::MetalBackend::new().is_some() {
            "Some (test skip-guards do not fire)"
        } else {
            "None"
        }
    );
    let raw: *mut objc::runtime::Object = device.as_ptr() as *mut _;
    // SAFETY: `counterSets` exists on MTLDevice and may return nil; the
    // pointer is inspected, never dereferenced when nil.
    let cs: *mut objc::runtime::Object = unsafe { msg_send![raw, counterSets] };
    if cs.is_null() {
        println!("[device counterSets] -> nil");
    } else {
        let n: u64 = unsafe { msg_send![cs, count] };
        println!("[device counterSets] -> {n} counter sets:");
        for i in 0..n {
            let set: *mut objc::runtime::Object = unsafe { msg_send![cs, objectAtIndex: i] };
            let name: *mut objc::runtime::Object = unsafe { msg_send![set, name] };
            let utf8: *const std::os::raw::c_char = unsafe { msg_send![name, UTF8String] };
            let s = unsafe { std::ffi::CStr::from_ptr(utf8) }.to_string_lossy();
            println!("  [{i}] {s}");
        }
    }
}
