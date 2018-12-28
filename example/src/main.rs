use trace_logging;

fn main() {
    trace_logging::register();
    trace_logging::write_ansi_string("this is the first log message");
    trace_logging::un_register();
}
