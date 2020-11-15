//! Minimal wrapper around the [`Remotery`](https://github.com/Celtoys/Remotery) real-time CPU profiler.
//! GPU profiling is not supported.
//!
//! Inspired by (seemingly outdated and abandoned) [`remotery`](https://docs.rs/remotery/0.1.2/remotery/),
//! which this is a slightly updated version of.
//!
//! # How to use
//! 1) Call [`Remotery::initialize`] at application startup.
//! ## Example
//! ```rust
//! use miniremotery::Remotery;
//! use strum::EnumMessage;
//!
//! let profiler = Remotery::initialize().unwrap_or_else(
//!     |error| panic!(
//!         "Remotery initialization error: {}"
//!         ,error.get_detailed_message().unwrap()
//!     )
//! );
//! // Remotery is finalized when `profiler` goes out of scope.
//! ```
//! 2) Add calls to [`Remotery::scope`] to code scopes you want to profile.
//! ## Example
//! ```rust
//! use miniremotery::{ Remotery, rmtSampleFlags };
//!
//! fn does_something_complicated() {
//!     let _scope = Remotery::scope("does_something_complicated", rmtSampleFlags::RMTSF_None);
//!
//!     // Some expensive calculations.
//!
//!     // Remotery gathers timing data when `_scope` goes out of scope.
//! }
//! ```
//! 3) Open `vis/index.html` when the profiled application is running to view real-time profile data.
//!
//! Refer to [`Remotery`](https://github.com/Celtoys/Remotery) repo for more info.
//!
//! [`Remotery::initialize`]: struct.Remotery.html#method.initialize
//! [`Remotery::scope`]: struct.Remotery.html#method.scope

use std::ffi::CString;
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr;

extern crate num_enum;
extern crate strum;

#[macro_use]
extern crate strum_macros;

mod error;

pub use crate::error::rmtError;
pub use strum::EnumMessage;

/// Flags which determine how the timing data for the profiled scopes is aggregated.
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum rmtSampleFlags {
    // Default behaviour
    RMTSF_None = 0,

    // Search parent for same-named samples and merge timing instead of adding a new sample
    RMTSF_Aggregate = 1,

    // Merge sample with parent if it's the same sample
    RMTSF_Recursive = 2,
}

/// Holds the Remotery global instance.
/// Created by [`Remotery::initialize`].
/// Destroys the Remotery global instance when dropped.
///
/// [`Remotery::initialize`]: struct.Remotery.html#method.initialize
pub struct Remotery {
    instance: *mut c_void,
}

/// Calls [`Remotery::end_cpu_sample`] when dropped.
///
/// [`Remotery::end_cpu_sample`]: struct.Remotery.html#method.end_cpu_sample
pub struct RemoteryScope;

impl Remotery {
    /// Initializes the Remotery global instance.
    /// Call once at application startup.
    /// Returned [`Remotery`] object will destroy the Remotery global instance
    /// when dropped.
    ///
    /// [`Remotery`]: struct.Remotery.html
    pub fn initialize() -> Result<Remotery, rmtError> {
        let mut instance = ptr::null_mut();

        let error = unsafe { _rmt_CreateGlobalInstance(&mut instance) };

        if error != rmtError::RMT_ERROR_NONE as u32 {
            Err(error::u32_to_rmtError(error))
        } else {
            Ok(Remotery { instance })
        }
    }

    /// Sets the display name of the calling thread for Remotery UI.
    pub fn set_current_thread_name(name: &str) {
        if let Ok(name) = CString::new(name) {
            unsafe {
                _rmt_SetCurrentThreadName(name.as_ptr());
            }
        }
    }

    /// Calls [`begin_cpu_sample`] and returns an object which calls [`end_cpu_sample`]
    /// when it goes out of scope.
    ///
    /// [`begin_cpu_sample`]: #method.begin_cpu_sample
    /// [`end_cpu_sample`]: #method.end_cpu_sample
    pub fn scope<'n, N: Into<Option<&'n str>>>(name: N, flags: rmtSampleFlags) -> RemoteryScope {
        Remotery::begin_cpu_sample(name.into(), flags);
        RemoteryScope {}
    }

    /// Begins a named CPU profile scope.
    /// Must be paired with a later call to [`end_cpu_sample`].
    ///
    /// [`end_cpu_sample`]: #method.end_cpu_sample
    pub fn begin_cpu_sample<'n, N: Into<Option<&'n str>>>(name: N, flags: rmtSampleFlags) {
        if let Ok(name) = CString::new(name.into().unwrap_or("<unnamed>")) {
            unsafe {
                _rmt_BeginCPUSample(name.as_ptr(), flags as u32, ptr::null_mut());
            }
        }
    }

    /// Ends a named CPU profile scope.
    /// Must be paired with a previous call to [`begin_cpu_sample`].
    ///
    /// [`begin_cpu_sample`]: #method.begin_cpu_sample
    pub fn end_cpu_sample() {
        unsafe {
            _rmt_EndCPUSample();
        }
    }
}

impl Drop for Remotery {
    fn drop(&mut self) {
        if self.instance != ptr::null_mut() {
            unsafe {
                _rmt_DestroyGlobalInstance(self.instance);
            }
        }
    }
}

impl Drop for RemoteryScope {
    fn drop(&mut self) {
        Remotery::end_cpu_sample();
    }
}

extern "C" {
    fn _rmt_CreateGlobalInstance(remotery: *mut *mut c_void) -> c_uint;
    fn _rmt_DestroyGlobalInstance(remotery: *mut c_void);

    fn _rmt_SetCurrentThreadName(thread_name: *const c_char);

    fn _rmt_BeginCPUSample(name: *const c_char, flags: c_uint, hash_cache: *mut c_uint);
    fn _rmt_EndCPUSample();
}
