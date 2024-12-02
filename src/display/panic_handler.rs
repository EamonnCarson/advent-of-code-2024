use std::panic::PanicInfo;

use console::Alignment;

use crate::display::scroll_writer::ScrollWriter;

pub fn naughty_list_panic_handler(panic_info: &PanicInfo<'_>) {
    let mut scroller = ScrollWriter::new();
    scroller.writeln("");
    scroller.writeln_aligned(include_str!("assets/naughty_list.txt"), Alignment::Center);
    scroller.writeln("");
    scroller.writeln(&panic_info.to_string());

    let backtrace_enabled = std::env::var("RUST_BACKTRACE").is_ok();
    if backtrace_enabled {
        let backtrace = std::backtrace::Backtrace::capture();
        scroller.writeln(&backtrace.to_string());
    } else {
        scroller.writeln(
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace",
        );
    }
    scroller.writeln("");
    scroller.writeln_aligned(include_str!("assets/santa_signature.txt"), Alignment::Right);
    scroller.finish();
}
