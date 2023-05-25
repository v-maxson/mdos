
/// Prints to the screen with a newline.
/// Formatting not supported.
#[macro_export]
macro_rules! println {
    () => {{
        use core::fmt::Write;
        use $crate::utility::FRAME_BUFFER_WRITER;

        let writer = unsafe { FRAME_BUFFER_WRITER.get_mut_unchecked() };

        writer.newline();
    }};

    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use $crate::utility::FRAME_BUFFER_WRITER;

        let writer = unsafe { FRAME_BUFFER_WRITER.get_mut_unchecked() };

        _ = writer.write_fmt(format_args!($($arg)*));
        writer.newline();
    }};
}

/// Prints to the screen. 
/// Formatting not supported.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use $crate::utility::FRAME_BUFFER_WRITER;

        let writer = unsafe { FRAME_BUFFER_WRITER.get_mut_unchecked() };

        _ = writer.write_fmt(format_args!($($arg)*));
    }};
}

/// Clears the screen.
#[macro_export]
macro_rules! cls {
    () => {{
        use $crate::utility::FRAME_BUFFER_WRITER;

        let writer = unsafe { FRAME_BUFFER_WRITER.get_mut_unchecked() };

        writer.clear();
    }};
}

