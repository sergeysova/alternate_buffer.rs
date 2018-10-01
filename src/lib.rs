//! Alternate screen buffer
//!
//! [Read question on StackOverflow](https://stackoverflow.com/questions/11023929/using-the-alternate-screen-in-a-bash-script)
//!
//! ```rust
//! # extern crate alternate_buffer;
//! # use std;
//!
//! // open alternate buffer
//! alternate_buffer::show();
//! println!("That text in alternate buffer");
//!
//! std::thread::sleep_ms(1000);
//!
//! alternate_buffer::hide();
//! println!("That text in main buffer");
//! ```
//!

use std::io::prelude::*;
use std::io::stdout;

/// Immediatly open alternate screen in current terminal session
///
/// If already opened alternate screen, nothing happened
pub fn show() -> std::io::Result<usize> {
    stdout().write(&[0x1b, 0x5b, 0x3f, 0x31, 0x30, 0x34, 0x39, 0x68])
}


/// Immediatly open main screen in current terminal session
///
/// If already in main screen, nothing happened
pub fn hide() -> std::io::Result<usize> {
    stdout().write(&[0x1b, 0x5b, 0x3f, 0x31, 0x30, 0x34, 0x39, 0x6c])
}

