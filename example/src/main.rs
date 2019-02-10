#![feature(const_str_as_bytes)]
#![feature(const_slice_len)]
#![feature(const_str_len)]
#![feature(const_raw_ptr_deref)]

use tracelogging::*;

fn main() {
    let handle = tracelogging_register!(
        "3970f9cf-2c0c-4f11-b1cc-e3a1e9958833",
        SimpleTraceLoggingProvider
    );
    let activity1 = tracelogging_start!(handle, "main");
    let var1 = 42;
    let var2 = "first";
    tracelogging!(handle, "myEvent1", var1, var2);
    tracelogging!(handle, "myEvent2");

    let var3 = format!("{}", 3);
    let activity2 = tracelogging_start!(handle, "myEvent3", var1, var2);
    tracelogging_stop!(handle, activity2, "myEvent3", var1, var3);

    tracelogging_expr!(
        handle,
        "myEvent4",
        || {
            tracelogging_tagged!(handle, "myEvent5", var1, var2, var3);
        },
        var1,
        var2
    );

    assert_eq!(
        3,
        tracelogging_expr!(handle, "myEvent6", {
            tracelogging_tagged!(handle, "myEvent7", var1, var2);
            2 + 1
        })
    );

    assert_eq!(
        3,
        tracelogging_fun!(handle, "myEvent6", || {
            tracelogging_tagged!(handle, "myEvent7", var1, var2);
            2 + 1
        })
    );

    tracelogging_stop!(handle, activity1, "main", var1, var3);

    tracelogging_un_register!(handle);
}
