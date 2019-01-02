use proc_macro_hack::proc_macro_hack;
use std::cell::RefCell;
use winapi::shared::{evntprov, guiddef, winerror};

thread_local!(pub static GUID_STACK: RefCell<Vec<guiddef::GUID>> = RefCell::new(Vec::with_capacity(1))); // init when first started so at least one element

/// Add one to an expression.
#[proc_macro_hack]
pub use tracelogging_impl::{register, write, write_start, write_stop, write_tagged};

pub static mut HANDLE: Option<evntprov::REGHANDLE> = None;

pub fn un_register() {
    if let Some(handle) = unsafe { HANDLE } {
        let result = unsafe { evntprov::EventUnregister(handle) };

        if result != winerror::ERROR_SUCCESS {
            println!("un_register failed with '{}'", result);
            return;
        }
    }
}

#[repr(u8)]
pub enum FieldType {
    ANSISTRING = 2,
    I8,
    U8,
    I6,
    U16,
    I32,
    U32,
    I64,
    U64,
}

pub const fn size_of<T>(_s: &T) -> u32 {
    std::mem::size_of::<T>() as u32
}
