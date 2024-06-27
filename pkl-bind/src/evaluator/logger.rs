/// Provides logging support for the pkl library
/// dependent on the environment variable `PKL_DEBUG`.
/// If `PKL_DEBUG=1` then more verbose logging is enabled.
/// without, we still print trace data.
#[macro_export]
macro_rules! log {
    // NOTE: this uses a variadic argument system
    // copied from the std::fmt eprintln! macro
    // see the appropriate documentation
    ($l:expr, $($arg:tt)*) => {{
        let key = "PKL_DEBUG";
        match std::env::var(key) {
            Ok(val) => {
                match val.parse::<i32>() {
                    Ok(v) => {
                        if v >= $l {
                            eprintln!($($arg)*);
                        }
                    },
                    Err(_) => {
                        if $l == 0 {
                            eprintln!($($arg)*);
                        }
                    }
                }
            },
            Err(_) => {
                if $l == 0 {
                    eprintln!($($arg)*);
                }
            },
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {
        log!(1, "this is an unprinted {}", "test");
    }

    #[test]
    fn test_env_set() {
        std::env::set_var("PKL_DEBUG", "1");

        log!(1, "this is a {} test", "printed");
    }

    #[test]
    fn test_env_set_no_print() {
        std::env::set_var("PKL_DEBUG", "0");

        log!(1, "this is still not to print");
    }

    #[test]
    fn test_printed_log() {
        log!(0, "This should print");
    }
}
