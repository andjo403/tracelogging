use trace_logging;

fn main() {
    trace_logging::register!(
        b"SimpleTraceLoggingProvider",
        "3970F9cf-2c0c-4f11-b1cc-e3a1e9958833"
    );
    let second = "second";
    trace_logging::write!(
        b"myEvent",
        (b"the answer", 41 + 1, trace_logging::FieldType::U32),
        (
            b"the smaller answer",
            42 as u8,
            trace_logging::FieldType::U8
        ),
        (
            b"msg",
            "this is the first log message",
            trace_logging::FieldType::ANSISTRING
        ),
        (b"msg2", second, trace_logging::FieldType::ANSISTRING),
    );
    trace_logging::un_register();
}
