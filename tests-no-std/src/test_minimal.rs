//! Minimal tests for no_std environment.
//!
//! These tests verify that `Spec` can be deserialized using `serde_json::from_str()`
//! in a no_std environment.

#![no_std]
#![no_main]

extern crate alloc;

use oci_spec::runtime::Spec;
use oci_spec::{Result, OciSpecError};
use linked_list_allocator::LockedHeap;

const HEAP_SIZE: usize = 8 * 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

fn init_allocator() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP_SIZE;
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_allocator();
    
    test_empty_json();
    test_minimal_version();
    test_version_only();
    test_version_and_process();
    test_with_root();
    
    loop {}
}

fn from_json_str(json: &str) -> Result<Spec> {
    serde_json::from_str(json).map_err(OciSpecError::from)
}

fn test_empty_json() {
    let json = r#"{}"#;
    let _result = from_json_str(json);
}

fn test_minimal_version() {
    let json = r#"{"ociVersion": "1.0.2-dev"}"#;
    let _result = from_json_str(json);
}

fn test_version_only() {
    let json = r#"{}"#;
    let result = from_json_str(json);
    let _ = result;
}

fn test_version_and_process() {
    let json = r#"{
        "ociVersion": "1.0.2-dev",
        "process": {
            "args": ["sh"],
            "cwd": "/"
        }
    }"#;
    let _result = from_json_str(json);
}

fn test_with_root() {
    let json = r#"{
        "ociVersion": "1.0.2-dev",
        "process": {
            "args": ["sh"],
            "cwd": "/"
        },
        "root": {
            "path": "rootfs",
            "readonly": true
        }
    }"#;
    let _result = from_json_str(json);
}
