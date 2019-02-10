use std::ffi::CString;
#[doc(hidden)]
pub use winapi::{
    shared::{
        evntprov::{
            EventActivityIdControl, EventProviderSetTraits, EventRegister, EventSetInformation,
            EventUnregister, EventWrite, EventWriteTransfer, EVENT_ACTIVITY_CTRL_SET_ID,
            EVENT_ACTIVITY_CTRL_CREATE_SET_ID, EVENT_DATA_DESCRIPTOR,
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

impl From<String> for FieldType {
    fn from(value: String) -> Self {
        let cstr = CString::new(value)
            .expect("CString::new failed")
            .into_bytes_with_nul();

        FieldType::ANSISTRING(cstr)
    }
}

impl From<CString> for FieldType {
    fn from(value: CString) -> Self {
        let cstr = value.into_bytes_with_nul();

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
