#![feature(const_str_as_bytes)]
#![feature(const_slice_len)]
#![feature(const_str_len)]
#![feature(const_raw_ptr_deref)]

#[macro_use]
extern crate tracelogging;

fn main() {
    let guid = tracelogging::internal::GUID {
        Data1: 0x3970_f9cf,
        Data2: 0x2c0c,
        Data3: 0x4f11,
        Data4: [0xb1, 0xcc, 0xe3, 0xa1, 0xe9, 0x95, 0x88, 0x33],
    }; //3970f9cf-2c0c-4f11-b1cc-e3a1e9958833

    tracelogging_register!(guid, SimpleTraceLoggingProvider);
    let var1 = 42;
    let var2 = "first";
    tracelogging!("myEvent1", var1, var2);
    tracelogging!("myEvent2");

    tracelogging_start!("myEvent3", var1, var2);
    tracelogging_stop!("myEvent3", var1, var2);

    tracelogging_expr!(
        "myEvent4",
        || {
            tracelogging_tagged!("myEvent5", var1, var2);
        },
        var1,
        var2
    );

    assert_eq!(
        3,
        tracelogging_expr!("myEvent6", {
            tracelogging_tagged!("myEvent7", var1, var2);
            2 + 1
        })
    );

    assert_eq!(
        3,
        tracelogging_fun!("myEvent6", || {
            tracelogging_tagged!("myEvent7", var1, var2);
            2 + 1
        })
    );

    tracelogging::un_register();
}
