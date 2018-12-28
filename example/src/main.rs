use trace_logging;

fn main() {
    println!("Hello, world!  1");
    trace_logging::register();
    println!("Hello, world!  2");
    trace_logging::write_ansi_string("this is the first log message");
    println!("Hello, world!  3");
    trace_logging::un_register();
    println!("Hello, world!  4");
}
