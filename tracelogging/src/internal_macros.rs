#[doc(hidden)]
#[macro_export]
macro_rules! c_string {
    ($id:ident) => {
        concat!(stringify!($id), '\0')
    };
    ($id:expr) => {
        concat!($id, '\0')
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! array_type {
    ($id:tt) => {
        [u8; $crate::c_string!($id).len()]
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! array_init {
    ($id:tt) => {
        unsafe { *($crate::c_string!($id).as_bytes() as *const _ as *const array_type!($id)) }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! event_meta_data_macro {
    ( $handle:ident $funk:ident $event:expr $(,)? $($vars:ident),* $(,)? ) => {{
            #[repr(C, packed)]
            struct EventMetaData {
                meta_size: u16,
                tags: u8,
                event_name: array_type!($event),
                $($vars: $crate::internal::FieldMetaData<array_type!($vars)>),*
            };

            $(let $vars = $crate::internal::FieldType::from($vars.clone());)*

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

            $funk!($handle event_data_descriptors)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! event_tracelogging_start {
    ($handle:ident $event_data_descriptors:ident) => {{
        let event_descriptor = $crate::internal::EVENT_DESCRIPTOR {
            Id: 0,
            Version: 0,
            Channel: 0,
            Level: 0,
            Opcode: 1,
            Task: 0,
            Keyword: 0,
        };

        let mut related_guid = unsafe { std::mem::uninitialized::<$crate::internal::GUID>() };
        unsafe {
            $crate::internal::EventActivityIdControl(
                $crate::internal::EVENT_ACTIVITY_CTRL_CREATE_SET_ID,
                &mut related_guid,
            );
        }

        unsafe {
            $crate::internal::EventWriteTransfer(
                $handle,
                &event_descriptor,
                core::ptr::null(),
                &related_guid,
                NUMBER_OF_DESCRIPTORS as u32,
                $event_data_descriptors.as_mut_ptr(),
            )
        };
        related_guid
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! event_tracelogging_stop {
    ($handle:ident $event_data_descriptors:ident) => {{
        let event_descriptor = $crate::internal::EVENT_DESCRIPTOR {
            Id: 0,
            Version: 0,
            Channel: 0,
            Level: 0,
            Opcode: 2,
            Task: 0,
            Keyword: 0,
        };

        unsafe {
            $crate::internal::EventWrite(
                $handle,
                &event_descriptor,
                NUMBER_OF_DESCRIPTORS as u32,
                $event_data_descriptors.as_mut_ptr(),
            )
        };
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! event_tracelogging_tagged {
    ($handle:ident $event_data_descriptors:ident) => {{
        let event_descriptor = $crate::internal::EVENT_DESCRIPTOR {
            Id: 0,
            Version: 0,
            Channel: 0,
            Level: 0,
            Opcode: 0,
            Task: 0,
            Keyword: 0,
        };

        unsafe {
            tracelogging::internal::EventWrite(
                $handle,
                &event_descriptor,
                NUMBER_OF_DESCRIPTORS as u32,
                $event_data_descriptors.as_mut_ptr(),
            )
        };
    }};
}
