use std::ffi::CString;
use std::mem;
use winapi::{um::{cguid::GUID_NULL,winnt::{PVOID,ULONGLONG}},shared::{evntprov, guiddef, winerror}};


static mut HANDLE: Option<evntprov::REGHANDLE> = None;

pub fn register() {
    let mut handle: evntprov::REGHANDLE = 0;
    let guid = guiddef::GUID {
        Data1: 0x3970f9cf,
        Data2: 0x2c0c,
        Data3: 0x4f11,
        Data4: [0xb1, 0xcc, 0xe3, 0xa1, 0xe9, 0x95, 0x88, 0x33],
    };

    let mut result = unsafe { evntprov::EventRegister(&guid, None, std::ptr::null_mut(), &mut handle) };

    if result != winerror::ERROR_SUCCESS {
        println!("register failed with '{}'", result);

        return;
    }

    unsafe { HANDLE = Some(handle);}

    #[repr(C, packed)]
    struct EventInformation {
        size: u16, 
        data: [u8;27],
    }
    let msg = CString::new("SimpleTraceLoggingProvider").expect("CString::new failed");
    let mut event_info = EventInformation{
        size: mem::size_of::<EventInformation>() as u16,
        data: unsafe {std::mem::uninitialized()},
    };
    event_info.data.clone_from_slice(msg.as_bytes_with_nul());
    unsafe {
    result = evntprov::EventSetInformation(
    		handle,
    		evntprov::EventProviderSetTraits,
    	    &event_info as *const _ as PVOID,
    		mem::size_of::<EventInformation>() as u32);
    }
    if result != winerror::ERROR_SUCCESS {
        println!("EventSetInformation failed with '{}'", result);

        return;
    }
}

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
            Keyword: 0,};

        #[repr(C, packed)]
        struct StringEvent{
            meta_size: u16,
	        tags: u8,
	        event_name: [u8;8],
            field_name: [u8;4],
	        in_type:InType,
        };

        let event_name = CString::new("myEvent").expect("CString::new failed");
        let field_name = CString::new("msg").expect("CString::new failed");
        let mut event_info = StringEvent{
            meta_size: mem::size_of::<StringEvent>() as u16,
            tags: 0,
            event_name: unsafe {std::mem::uninitialized()},
            field_name: unsafe {std::mem::uninitialized()},
            in_type: InType::ANSISTRING,
        };
        event_info.event_name.clone_from_slice(event_name.as_bytes_with_nul());
        event_info.field_name.clone_from_slice(field_name.as_bytes_with_nul());

        let mut event_data_descriptors: [evntprov::EVENT_DATA_DESCRIPTOR;2] = [
        evntprov::EVENT_DATA_DESCRIPTOR{
            Ptr: &event_info as *const _ as ULONGLONG,
            Size:  mem::size_of::<StringEvent>() as u32,
            u:unsafe{std::mem::zeroed()}},
        evntprov::EVENT_DATA_DESCRIPTOR{
            Ptr: msg.as_ptr() as *const _ as ULONGLONG,
            Size: msg.len() as u32,
            u:unsafe{std::mem::zeroed()}}];

        unsafe {event_data_descriptors[0].u.s_mut().Type = evntprov::EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA;}

	    unsafe {evntprov::EventWriteTransfer(handle, &event_descriptor,&GUID_NULL,&GUID_NULL, 2, event_data_descriptors.as_mut_ptr())};
    }
}
