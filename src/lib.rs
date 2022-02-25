use std::ops::Drop;
use std::env::current_exe;
use std::fs::remove_file;

/// Self-destructs when dropped. Doesn't ensure that executable is always deleted, so may not work 100% of the time.
///
/// ### Usage
/// ```rust
/// fn main() {
///     let _mortem = mortem::soft(); // register guard
///
///     // some code
///     println!("Hello!")
///
///     // functions ends, _mortem drops and executable is deleted
/// }
/// ```
pub fn soft() -> Guard {
    Guard {
        ensure: false,
    }
}

/// Self-destructs when dropped. Tries until it succeeds, so may run forever.
///
/// ### Usage
/// ```rust
/// fn main() {
///     let _mortem = mortem::hard(); // register guard
///
///     // some code
///     println!("Hello!")
///
///     // functions ends, _mortem drops and executable is deleted
/// }
/// ```
pub fn hard() -> Guard {
    Guard {
        ensure: true,
    }
}

pub struct Guard {
    /// Ensure deletion of the file, retrying till executable is deleted.
    ensure: bool,
}

impl Drop for Guard {
    fn drop(&mut self) {
        loop {
            match current_exe() {
                Err(_) if self.ensure => continue,
                Err(_) => break,
                Ok(path) => {
                    if remove_file(path).is_err() && self.ensure {
                        continue
                    }
                }
            }
            break
        }
    }
}
