#[macro_export]
macro_rules! flush {
    () => {{
        use std::io::{stdout, Write};

        stdout()
            .lock()
            .flush()
            .with_context(|| "Failed to flush stdout")
            .unwrap();
    }};
}

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {
        print!($($arg)*);
        flush!();
    }
}
