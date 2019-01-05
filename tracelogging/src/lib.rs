use proc_macro_hack::proc_macro_hack;
use winapi::shared::{evntprov, winerror};

/// register as an event provider
#[proc_macro_hack]
pub use tracelogging_impl::register;

/// log an event
/// not associated with any activity.
#[proc_macro_hack]
pub use tracelogging_impl::write;

/// Starts an ETW activity and logs the start event.
/// The activity is thread local and must be stoped
/// in the same thread
#[proc_macro_hack]
pub use tracelogging_impl::write_start;

/// Stops an ETW activity and logs the stop event.
#[proc_macro_hack]
pub use tracelogging_impl::write_stop;

/// Logs an event with an associated ETW activity.
/// before use an activity must have been started for this thread,
/// and must not have been stopped yet.
#[proc_macro_hack]
pub use tracelogging_impl::write_tagged;


#[doc(hidden)]
pub mod internal {
    pub use winapi::{
        shared::{
            evntprov::{
                EventActivityIdControl, EventProviderSetTraits, EventRegister, EventSetInformation,
                EventWrite, EventWriteTransfer, EVENT_ACTIVITY_CTRL_CREATE_ID,
                EVENT_DATA_DESCRIPTOR, EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA, EVENT_DESCRIPTOR,EVENT_DATA_DESCRIPTOR_u,
                REGHANDLE,
            },
            guiddef::GUID,
            winerror::ERROR_SUCCESS,
        },
        um::winnt::{PVOID, ULONGLONG},
    };
    use std::cell::RefCell;
    thread_local!(pub static GUID_STACK: RefCell<Vec<GUID>> = RefCell::new(Vec::with_capacity(1))); // init when first started so at least one element
    pub static mut HANDLE: Option<REGHANDLE> = None;
    pub const fn size_of<T>(_s: &T) -> u32 {
        std::mem::size_of::<T>() as u32
    }
}

/// unregister as an event provider
pub fn un_register() {
    if let Some(handle) = unsafe { internal::HANDLE } {
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
