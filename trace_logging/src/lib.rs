use proc_macro_hack::proc_macro_hack;
use std::ffi::CString;
use std::mem;
use winapi::{
    shared::{evntprov, winerror},
    um::{
        cguid::GUID_NULL,
        winnt::ULONGLONG,
    },
};

/// Add one to an expression.
#[proc_macro_hack]
pub use tracelogging_impl::tracelogging_register;

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
enum InType {
    ANSISTRING = 2,
}

pub fn write_ansi_string(msg: &str) {
    if let Some(handle) = unsafe { HANDLE } {
        let c_string = CString::new(msg).expect("CString::new failed");
        let msg = c_string.as_bytes_with_nul();

        let event_descriptor = evntprov::EVENT_DESCRIPTOR {
            Id: 0,
            Version: 0,
            Channel: 0,
            Level: 0,
            Opcode: 0,
            Task: 0,
            Keyword: 0,
        };

        #[repr(C, packed)]
        struct StringEvent {
            meta_size: u16,
            tags: u8,
            event_name: [u8; 8],
            field_name: [u8; 4],
            in_type: InType,
        };

        let event_info = StringEvent {
            meta_size: mem::size_of::<StringEvent>() as u16,
            tags: 0,
            event_name: [b'm', b'y', b'E', b'v', b'e', b'n', b't', b'\0'],
            field_name: [b'm', b's', b'g', b'\0'],
            in_type: InType::ANSISTRING,
        };

        let mut event_data_descriptors: [evntprov::EVENT_DATA_DESCRIPTOR; 2] = [
            evntprov::EVENT_DATA_DESCRIPTOR {
                Ptr: &event_info as *const _ as ULONGLONG,
                Size: mem::size_of::<StringEvent>() as u32,
                u: unsafe { std::mem::zeroed() },
            },
            evntprov::EVENT_DATA_DESCRIPTOR {
                Ptr: msg.as_ptr() as *const _ as ULONGLONG,
                Size: msg.len() as u32,
                u: unsafe { std::mem::zeroed() },
            },
        ];

        unsafe {
            event_data_descriptors[0].u.s_mut().Type =
                evntprov::EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA;
        }

        unsafe {
            evntprov::EventWriteTransfer(
                handle,
                &event_descriptor,
                &GUID_NULL,
                &GUID_NULL,
                2,
                event_data_descriptors.as_mut_ptr(),
            )
        };
    }
}
