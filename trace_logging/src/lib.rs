use proc_macro_hack::proc_macro_hack;
use winapi::{
    shared::{evntprov, winerror}
};

/// Add one to an expression.
#[proc_macro_hack]
pub use tracelogging_impl::{register, write};

pub static mut HANDLE: Option<evntprov::REGHANDLE> = None;

pub fn un_register() {
    if let Some(handle) = unsafe { HANDLE } {
        println!("un_register {}", handle);
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

pub fn size_of<T>(_s: T) -> u32 {
    std::mem::size_of::<T>() as u32
}
