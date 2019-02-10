#![feature(const_str_as_bytes)]
#![feature(const_slice_len)]
#![feature(const_str_len)]
#![feature(const_raw_ptr_deref)]

#[doc(hidden)]
pub mod internal;

#[doc(hidden)]
pub mod internal_macros;

#[macro_export]
macro_rules! tracelogging {
    ( $handle:expr, $($arg:tt)* ) => {
        $crate::event_meta_data_macro!($handle, event_tracelogging_tagged $($arg)*)
    };
}

#[macro_export]
macro_rules! tracelogging_start {
    ( $handle:expr, $($arg:tt)* ) => {
        $crate::event_meta_data_macro!($handle, event_tracelogging_start $($arg)*)
    };
}

#[macro_export]
macro_rules! tracelogging_stop {
    ( $handle:expr, $related_guid:ident, $($arg:tt)* ) => {{
        $crate::event_meta_data_macro!($handle, event_tracelogging_stop $($arg)*);
        unsafe {
            $crate::internal::EventActivityIdControl(
                $crate::internal::EVENT_ACTIVITY_CTRL_SET_ID,
                &$related_guid as *const $crate::internal::GUID as *mut $crate::internal::GUID,
            );
        }
    }};
}

#[macro_export]
macro_rules! tracelogging_tagged {
    ( $handle:expr, $($arg:tt)* ) => {
        $crate::event_meta_data_macro!($handle, event_tracelogging_tagged $($arg)*)
    };
}

#[macro_export]
macro_rules! tracelogging_expr {
    ( $handle:expr, $name:expr , $exp:expr $(,)? $($arg:ident),* $(,)? ) => {
        {
            let activity = $crate::tracelogging_start!($handle, $name $($arg),*);
            let result = $exp;
            $crate::tracelogging_stop!($handle, activity, $name);
            result
        }
    };
}

#[macro_export]
macro_rules! tracelogging_fun {
    ( $handle:expr, $name:expr , $exp:expr $(,)? $($arg:ident),* $(,)? ) => {
        {
            let activity = $crate::tracelogging_start!($handle, $name $($arg),*);
            let result = $exp();
            $crate::tracelogging_stop!($handle, activity, $name);
            result
        }
    };
}

/// register as an event provider
/// guid in the form "3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"
/// name of the provider
#[macro_export]
macro_rules! tracelogging_register {
    ( $guid:literal , $provider_name:ident ) => {{
        let mut handle: $crate::internal::REGHANDLE = 0;
        // parse guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833
        let guid = $crate::internal::GUID {
            Data1: u32::from_str_radix(&$guid[0..8], 16)
                .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
            Data2: u16::from_str_radix(&$guid[9..13], 16)
                .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
            Data3: u16::from_str_radix(&$guid[14..18], 16)
                .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
            Data4: [
                u8::from_str_radix(&$guid[19..21], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
                u8::from_str_radix(&$guid[21..23], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
                u8::from_str_radix(&$guid[24..26], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
                u8::from_str_radix(&$guid[26..28], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
                u8::from_str_radix(&$guid[28..30], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
                u8::from_str_radix(&$guid[30..32], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
                u8::from_str_radix(&$guid[32..34], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
                u8::from_str_radix(&$guid[34..36], 16)
                    .expect("guid of the form 3970f9cf-2c0c-4f11-b1cc-e3a1e9958833"),
            ],
        };

        let mut result = unsafe {
            $crate::internal::EventRegister(&guid, None, core::ptr::null_mut(), &mut handle)
        };

        assert_eq!(
            result,
            $crate::internal::ERROR_SUCCESS,
            "call to EventRegister failed"
        );
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
        assert_eq!(
            result,
            $crate::internal::ERROR_SUCCESS,
            "call to EventSetInformation failed"
        );
        handle
    }};
}

/// unregister as an event provider
#[macro_export]
macro_rules! tracelogging_un_register {
    ($handle:expr) => {{
        let result = unsafe { $crate::internal::EventUnregister($handle) };

        assert_eq!(
            result,
            $crate::internal::ERROR_SUCCESS,
            "call to EventUnregister failed"
        );
    }};
}
