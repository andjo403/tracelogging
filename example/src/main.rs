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

    tracelogging::register!(guid, SimpleTraceLoggingProvider);
    let var1 = 42;
    let var2 = "first";
    tracelogging::write!("myEvent1", var1, var2);
    tracelogging::write!("myEvent2");

    tracelogging::write_start!("myEvent3", var1, var2);
    tracelogging::write_stop!("myEvent3", var1, var2);

    tracelogging::write_expr!(
        "myEvent4",
        tracelogging::write_tagged!("myEvent5", var1, var2),
        var1,
        var2
    );

    tracelogging::write_expr!(
        "myEvent6",
        tracelogging::write_tagged!("myEvent7", var1, var2),
    );

    tracelogging::un_register();
}
