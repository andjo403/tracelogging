#![feature(const_str_as_bytes)]
#![feature(const_slice_len)]
#![feature(const_str_len)]
#![feature(const_raw_ptr_deref)]

#[macro_use]
extern crate tracelogging;

use  std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {

    tracelogging_register!("3970f9cf-2c0c-4f11-b1cc-e3a1e9958833", SimpleTraceLoggingProvider);
    let var1 = 42;
    let var2 = "first";
    tracelogging!("myEvent1", var1, var2);
    tracelogging!("myEvent2");

    let var3 = format!("{}", 3);
    tracelogging_start!("myEvent3", var1, var2);
    tracelogging_stop!("myEvent3", var1, var3);

    tracelogging_expr!(
        "myEvent4",
        || {
            tracelogging_tagged!("myEvent5", var1, var2, var3);
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

    tracelogging_un_register!();

    Ok(())
}
