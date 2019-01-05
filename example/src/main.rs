fn main() {
    tracelogging::register!(
        b"SimpleTraceLoggingProvider",
        "3970F9cf-2c0c-4f11-b1cc-e3a1e9958833"
    );
    for _ in 0..4 {
        tracelogging::write_start!(
            b"first",
            (b"the answer", 41 + 1, tracelogging::FieldType::U32),
        );

        let second = "second";
        tracelogging::write!(
            b"myEvent",
            (b"the answer", 41 + 1, tracelogging::FieldType::U32),
            (
                b"the smaller answer",
                42u8,
                tracelogging::FieldType::U8
            ),
            (
                b"msg",
                "this is the first log message",
                tracelogging::FieldType::ANSISTRING
            ),
            (b"msg2", second, tracelogging::FieldType::ANSISTRING),
        );

        tracelogging::write_tagged!(
            b"myEvent",
            (b"the answer", 41 + 1, tracelogging::FieldType::U32),
            (
                b"the smaller answer",
                42u8,
                tracelogging::FieldType::U8
            ),
            (
                b"msg",
                "this is the first log message",
                tracelogging::FieldType::ANSISTRING
            ),
            (b"msg2", second, tracelogging::FieldType::ANSISTRING),
        );

        tracelogging::write_stop!(
            b"first",
            (b"the answer", 41 + 1, tracelogging::FieldType::U32),
        );
    }
    tracelogging::un_register();
}
