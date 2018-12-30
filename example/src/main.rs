use trace_logging;

fn main() {
    trace_logging::tracelogging_register!(b"SimpleTraceLoggingProvider","3970F9cf-2c0c-4f11-b1cc-e3a1e9958833");
    trace_logging::write_ansi_string("this is the first log message");
    trace_logging::un_register();
}
