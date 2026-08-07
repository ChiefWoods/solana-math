//! Host-only overflow diagnostics behind the `debug` feature.

/// Logs `message` with the caller's file/line when the `debug` feature is enabled.
///
/// No-op in the default `no_std` build. Requires `debug` (which enables `std`) for
/// `println!`
#[cfg(feature = "debug")]
#[inline(always)]
#[track_caller]
pub fn log(message: &str) {
    let caller = core::panic::Location::caller();
    println!("{message} at {}:{}", caller.file(), caller.line());
}

#[cfg(not(feature = "debug"))]
#[inline(always)]
pub fn log(_message: &str) {}
