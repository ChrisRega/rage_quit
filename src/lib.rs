#![crate_name = "rage_quit"]
//! # Printing flipping tables and catching tables before panic / catch_unwind
//! This crate allows to use the macros `rage_quit` and `catch_table`
//! Example for panic:
//! ```no_run
//! use rage_quit::rage_quit;
//! rage_quit!("Use me instead of panic!");
//! ```
//!
//! Example for catching a panic:
//! ```no_run
//! use rage_quit::{catch_table, rage_quit};
//! assert!(catch_table!(|| {
//!             rage_quit!("Le wild rage quit appeared!");
//!         })
//!         .is_err());
//! ```
//!

#[macro_export]
macro_rules! rage_quit {
    ( $x:expr ) => {
        println!("Le wild rage quit appears!");
        println!("(ノಠ益ಠ)ノ彡┻━┻");
        panic!($x);
    };
}

#[macro_export]
macro_rules! catch_table {
    ( $x:expr ) => {{
        let result = std::panic::catch_unwind($x);
        if result.is_err() {
            println!("Le wild table was caught!");
            println!("┳━┳ ヽ(ಠل͜ಠ)ﾉ");
        }
        result
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn simple() {
        rage_quit!("Le wild rage quit appeared!");
    }

    #[test]
    fn can_catch() {
        assert!(catch_table!(|| {
            rage_quit!("Le wild rage quit appeared!");
        })
        .is_err());
    }
}
