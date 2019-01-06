#![feature(const_str_as_bytes)]
#![feature(const_slice_len)]
#![feature(const_str_len)]
#![feature(const_raw_ptr_deref)]

#[doc(hidden)]
pub mod internal {
    use std::cell::RefCell;
    use std::ffi::CString;
    pub use winapi::{
        shared::{
            evntprov::{
                EVENT_DATA_DESCRIPTOR_u, EventActivityIdControl, EventProviderSetTraits,
                EventRegister, EventSetInformation, EventWrite, EventWriteTransfer,
                EVENT_ACTIVITY_CTRL_CREATE_ID, EVENT_DATA_DESCRIPTOR,
                EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA, EVENT_DESCRIPTOR, REGHANDLE,
            },
            guiddef::GUID,
            winerror::ERROR_SUCCESS,
        },
        um::winnt::{PVOID, ULONGLONG},
    };

    pub struct FieldMetaData<T> {
        pub name: T,
        pub field_type: u8,
    }

    pub enum FieldType {
        ANSISTRING(Vec<u8>),
        I8(i8),
        U8(u8),
        I16(i16),
        U16(u16),
        I32(i32),
        U32(u32),
        I64(i64),
        U64(u64),
    }
    impl From<i8> for FieldType {
        fn from(value: i8) -> Self {
            FieldType::I8(value)
        }
    }
    impl From<u8> for FieldType {
        fn from(value: u8) -> Self {
            FieldType::U8(value)
        }
    }
    impl From<i16> for FieldType {
        fn from(value: i16) -> Self {
            FieldType::I16(value)
        }
    }
    impl From<u16> for FieldType {
        fn from(value: u16) -> Self {
            FieldType::U16(value)
        }
    }
    impl From<i32> for FieldType {
        fn from(value: i32) -> Self {
            FieldType::I32(value)
        }
    }
    impl From<u32> for FieldType {
        fn from(value: u32) -> Self {
            FieldType::U32(value)
        }
    }
    impl From<i64> for FieldType {
        fn from(value: i64) -> Self {
            FieldType::I64(value)
        }
    }
    impl From<u64> for FieldType {
        fn from(value: u64) -> Self {
            FieldType::U64(value)
        }
    }

    impl From<&str> for FieldType {
        fn from(value: &str) -> Self {
            let cstr = CString::new(value)
                .expect("CString::new failed")
                .into_bytes_with_nul();

            FieldType::ANSISTRING(cstr)
        }
    }

    impl FieldType {
        pub fn size_of(&self) -> u32 {
            let size = match self {
                FieldType::ANSISTRING(ref cstr) => cstr.len(),
                FieldType::I8(_) => std::mem::size_of::<i8>(),
                FieldType::U8(_) => std::mem::size_of::<u8>(),
                FieldType::I16(_) => std::mem::size_of::<i16>(),
                FieldType::U16(_) => std::mem::size_of::<u16>(),
                FieldType::I32(_) => std::mem::size_of::<i32>(),
                FieldType::U32(_) => std::mem::size_of::<u32>(),
                FieldType::I64(_) => std::mem::size_of::<i64>(),
                FieldType::U64(_) => std::mem::size_of::<u64>(),
            };
            size as u32
        }

        pub fn as_ptr(&self) -> *const u8 {
            match self {
                FieldType::ANSISTRING(ref cstr) => cstr.as_ptr(),
                FieldType::I8(ref u) => u as *const _ as *const u8,
                FieldType::U8(ref u) => u as *const _ as *const u8,
                FieldType::I16(ref u) => u as *const _ as *const u8,
                FieldType::U16(ref u) => u as *const _ as *const u8,
                FieldType::I32(ref u) => u as *const _ as *const u8,
                FieldType::U32(ref u) => u as *const _ as *const u8,
                FieldType::I64(ref u) => u as *const _ as *const u8,
                FieldType::U64(ref u) => u as *const _ as *const u8,
            }
        }

        pub fn as_event_type(&self) -> u8 {
            match self {
                FieldType::ANSISTRING(_) => 2,
                FieldType::I8(_) => 3,
                FieldType::U8(_) => 4,
                FieldType::I16(_) => 5,
                FieldType::U16(_) => 6,
                FieldType::I32(_) => 7,
                FieldType::U32(_) => 8,
                FieldType::I64(_) => 9,
                FieldType::U64(_) => 10,
            }
        }
    }

    thread_local!(pub static GUID_STACK: RefCell<Vec<GUID>> = RefCell::new(Vec::with_capacity(1))); // init when first started so at least one element
    pub static mut HANDLE: Option<REGHANDLE> = None;

}

#[macro_export]
macro_rules! c_string {
    ($id:ident) => {
        concat!(stringify!($id), '\0')
    };
    ($id:expr) => {
        concat!($id, '\0')
    };
}

#[macro_export]
macro_rules! array_type {
    ($id:tt) => {
        [u8; $crate::c_string!($id).len()]
    };
}

#[macro_export]
macro_rules! array_init {
    ($id:tt) => {
        unsafe { *($crate::c_string!($id).as_bytes() as *const _ as *const array_type!($id)) }
    };
}

#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[macro_export]
macro_rules! event_meta_data_macro {
        ( $opcode:tt $funk:ident $event:expr $(,)? $($vars:ident),* $(,)? ) => {
            if let Some(handle) = unsafe { $crate::internal::HANDLE } {
                #[repr(C, packed)]
                struct EventMetaData {
                    meta_size: u16,
                    tags: u8,
                    event_name: array_type!($event),
                    $($vars: $crate::internal::FieldMetaData<array_type!($vars)>),*
                };

                $(let $vars = $crate::internal::FieldType::from($vars);)*

                let event_meta_data = EventMetaData {
                    meta_size: std::mem::size_of::<EventMetaData>() as u16,
                    tags: 0,
                    event_name: array_init!($event),
                    $($vars: $crate::internal::FieldMetaData {
                        name : $crate::array_init!($vars),
                        field_type: $vars.as_event_type()
                    } ),*
                };

                const NUMBER_OF_DESCRIPTORS : usize = <[()]>::len(&[$($crate::replace_expr!($vars ())),*]) + 1;

                let mut event_data_descriptors: [$crate::internal::EVENT_DATA_DESCRIPTOR; NUMBER_OF_DESCRIPTORS] = [
                    $crate::internal::EVENT_DATA_DESCRIPTOR {
                        Ptr: &event_meta_data as *const _ as $crate::internal::ULONGLONG,
                        Size: std::mem::size_of::<EventMetaData>() as u32,
                        u: unsafe { std::mem::transmute(1u32) },
                    },
                    $($crate::internal::EVENT_DATA_DESCRIPTOR {
                        Ptr: $vars.as_ptr() as *const _ as $crate::internal::ULONGLONG,
                        Size: $vars.size_of(),
                        u: unsafe { std::mem::transmute(0u32) },
                    }),*
                ];

                let event_descriptor = $crate::internal::EVENT_DESCRIPTOR {
                    Id: 0,
                    Version: 0,
                    Channel: 0,
                    Level: 0,
                    Opcode: $opcode,
                    Task: 0,
                    Keyword: 0,
                };

                $funk!(handle event_descriptor event_data_descriptors);
            }
        };
    }

#[macro_export]
macro_rules! event_tracelogging_start {
    ($handle:ident $event_descriptor:ident $event_data_descriptors:ident) => {
        $crate::internal::GUID_STACK.with(|s| {
            let mut stack = s.borrow_mut();
            let mut current = unsafe { std::mem::uninitialized::<$crate::internal::GUID>() };
            unsafe {
                $crate::internal::EventActivityIdControl(
                    $crate::internal::EVENT_ACTIVITY_CTRL_CREATE_ID,
                    &mut current,
                );
            }
            stack.push(current);

            unsafe {
                $crate::internal::EventWriteTransfer(
                    $handle,
                    &$event_descriptor,
                    &current,
                    std::ptr::null(),
                    NUMBER_OF_DESCRIPTORS as u32,
                    $event_data_descriptors.as_mut_ptr(),
                )
            };
        });
    };
}

#[macro_export]
macro_rules! event_tracelogging {
    ($handle:ident $event_descriptor:ident $event_data_descriptors:ident) => {
        unsafe {
            $crate::internal::EventWrite(
                $handle,
                &$event_descriptor,
                NUMBER_OF_DESCRIPTORS as u32,
                $event_data_descriptors.as_mut_ptr(),
            )
        }
    };
}

#[macro_export]
macro_rules! event_tracelogging_stop {
    ($handle:ident $event_descriptor:ident $event_data_descriptors:ident) => {
        $crate::internal::GUID_STACK.with(|s| {
            let mut stack = s.borrow_mut();
            let current = stack
                .pop()
                .expect("tracelogging_start needs to done before tracelogging_stop");

            unsafe {
                $crate::internal::EventWriteTransfer(
                    $handle,
                    &$event_descriptor,
                    &current,
                    std::ptr::null(),
                    NUMBER_OF_DESCRIPTORS as u32,
                    $event_data_descriptors.as_mut_ptr(),
                )
            };
        });
    };
}

#[macro_export]
macro_rules! event_tracelogging_tagged {
    ($handle:ident $event_descriptor:ident $event_data_descriptors:ident) => {
        tracelogging::internal::GUID_STACK.with(|s| {
            let stack = s.borrow();
            let current = stack
                .last()
                .expect("tracelogging_start needs to done before tracelogging_stop");

            unsafe {
                tracelogging::internal::EventWriteTransfer(
                    $handle,
                    &$event_descriptor,
                    current,
                    std::ptr::null(),
                    NUMBER_OF_DESCRIPTORS as u32,
                    $event_data_descriptors.as_mut_ptr(),
                )
            };
        });
    };
}

#[macro_export]
macro_rules! tracelogging_register {
    ( $guid:ident , $provider_name:ident ) => {
        let mut handle: $crate::internal::REGHANDLE = 0;

        let mut result = unsafe {
            $crate::internal::EventRegister(&$guid, None, std::ptr::null_mut(), &mut handle)
        };

        if result == $crate::internal::ERROR_SUCCESS {
            #[repr(C, packed)]
            struct ProviderMetaData {
                size: u16,
                data: array_type!($provider_name),
            }

            const EVENT_INFO: ProviderMetaData = ProviderMetaData {
                size: std::mem::size_of::<ProviderMetaData>() as u16,
                data: $crate::array_init!($provider_name),
            };

            unsafe {
                result = $crate::internal::EventSetInformation(
                    handle,
                    $crate::internal::EventProviderSetTraits,
                    &EVENT_INFO as *const _ as $crate::internal::PVOID,
                    std::mem::size_of::<ProviderMetaData>() as u32,
                );
            }
            if result != $crate::internal::ERROR_SUCCESS {
                println!("EventSetInformation failed with '{}'", result);
            }
        } else {
            println!("EventRegister failed with '{}'", result);
        }
        unsafe {
            $crate::internal::HANDLE = Some(handle);
        }
    };
}

#[macro_export]
macro_rules! tracelogging {
    ( $($arg:tt)* ) => {
        $crate::event_meta_data_macro!(0 event_tracelogging $($arg)*)
    };
}

#[macro_export]
macro_rules! tracelogging_start {
    ( $($arg:tt)* ) => {
        $crate::event_meta_data_macro!(1 event_tracelogging_start $($arg)*)
    };
}

#[macro_export]
macro_rules! tracelogging_stop {
    ( $($arg:tt)* ) => {
        $crate::event_meta_data_macro!(2 event_tracelogging_stop $($arg)*)
    };
}

#[macro_export]
macro_rules! tracelogging_tagged {
    ( $($arg:tt)* ) => {
        $crate::event_meta_data_macro!(0 event_tracelogging_tagged $($arg)*)
    };
}

#[macro_export]
macro_rules! tracelogging_expr {
    ( $name:expr , $exp:expr $(,)? $($arg:ident),* $(,)? ) => {
        {
            $crate::tracelogging_start!($name $($arg),*);
            let result = $exp;
            $crate::tracelogging_stop!($name $($arg),*);
            result
        }
    };
}

#[macro_export]
macro_rules! tracelogging_fun {
    ( $name:expr , $exp:expr $(,)? $($arg:ident),* $(,)? ) => {
        {
            $crate::tracelogging_start!($name $($arg),*);
            let result = $exp();
            $crate::tracelogging_stop!($name $($arg),*);
            result
        }
    };
}

/// unregister as an event provider
pub fn un_register() {
    use winapi::shared::{evntprov, winerror};
    if let Some(handle) = unsafe { internal::HANDLE } {
        let result = unsafe { evntprov::EventUnregister(handle) };

        if result != winerror::ERROR_SUCCESS {
            println!("un_register failed with '{}'", result);
            return;
        }
    }
}
