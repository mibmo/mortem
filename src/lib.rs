//! Mortem is a library for that strives to achieve one thing; ensuring the executable is deleted
//! after execution stops and be out of the way while doing so.
//!
//! Mortem requires only one line to function and works completely transparently in the background.
//! It does this by providing a [`Guard`] type.
//! When a [`Guard`] is created, it does nothing.
//! When it gets dropped, however, it begins the process of deleting the host executable.
//! It does this with the best of it's ability, either trying once and exiting successfully upon failure (provided by [`Guard::soft()`]) or trying continually and blocking till it succeeds (provided by [`Guard::hard()`]).
//!
//! This means, for Mortem to do it's work, all that it needs is to be dropped at the end of the
//! main function.
//!
//! # Usage
//! Simply register a guard (either `soft` or `hard`) in the main function, and have it be dropped to delete the binary.
//! ```rust
//! fn main() {
//!     let _mortem = mortem::hard(); // register mortem guard
//!
//!     // some code
//!     println!("Hello!")
//!
//!     // _mortem drops and executable is deleted
//! }
//! ```
//!
//! # Async
//! Using Mortem in an async runtime is functionally the same and no user action is required beyond
//! the usual.
//!
//! #### Tokio
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     let _mortem = mortem::hard(); // register mortem guard
//!
//!     // some code
//!     tokio::spawn(async {
//!         println!("Hello!")
//!     }).await;
//!
//!     // _mortem drops and executable is deleted
//! }
//! ```
//!
//! #### async-std
//! ```rust
//! #[async_std::main]
//! async fn main() {
//!     let _mortem = mortem::hard(); // register mortem guard
//!
//!     // some code
//!     async_std::task::spawn(async {
//!         println!("Hello!")
//!     }).await;
//!
//!     // _mortem drops and executable is deleted
//! }
//! ```

use std::env::current_exe;
use std::fs::remove_file;
use std::ops::Drop;

#[cfg(feature = "tracing")]
use tracing::{debug, error};

/// Create a guard that when dropped tries to delete the host executable.
///
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
#[inline(always)]
pub fn soft() -> Guard {
    Guard::soft()
}

/// Create a guard that when dropped blocks till the host executable is successfully deleted.
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
#[inline(always)]
pub fn hard() -> Guard {
    Guard::hard()
}

/// Executable guard.
pub struct Guard {
    /// Ensure deletion of the file, retrying till executable is deleted.
    ensure: bool,
}

impl Guard {
    fn new(ensure: bool) -> Self {
        #[cfg(feature = "tracing")]
        debug!(?ensure, "creating mortem guard");
        Guard { ensure }
    }

    pub fn soft() -> Self {
        Self::new(false)
    }

    /// Create a guard that blocks till the executable is successfully deleted
    ///
    /// See [`hard`].
    pub fn hard() -> Self {
        Self::new(true)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        #[cfg(feature = "tracing")]
        debug!(ensure = self.ensure, "dropping mortem guard");

        loop {
            match current_exe() {
                Err(_) if self.ensure => continue,
                Err(_) => {
                    #[cfg(feature = "tracing")]
                    error!(ensure = self.ensure, "failed to delete executable");
                    panic!("failed to delete executable")
                }
                Ok(path) => {
                    if remove_file(path).is_err() && self.ensure {
                        #[cfg(feature = "tracing")]
                        error!(
                            ensure = self.ensure,
                            "failed to delete executable; retrying"
                        );
                        continue;
                    }
                }
            }
            break;
        }
    }
}
