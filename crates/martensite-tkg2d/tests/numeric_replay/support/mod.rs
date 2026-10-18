macro_rules! replay_print {
    ($output:expr, $($arguments:tt)*) => { $output.emit(format_args!($($arguments)*)) };
}
macro_rules! replay_println {
    ($output:expr,) => { $output.emit(format_args!("\n")) };
    ($output:expr, $($arguments:tt)*) => {{
        $output.emit(format_args!($($arguments)*));
        $output.emit(format_args!("\n"));
    }};
}
pub mod adaptor;
pub mod adaptor_cases;
pub mod bspl;
pub mod common;
pub mod curves;
pub mod eval;
pub mod grid;
pub mod print;
pub mod simple;

pub mod dispatch;
pub mod output;
pub mod reader;
