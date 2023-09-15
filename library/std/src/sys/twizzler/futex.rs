use crate::sync::atomic::AtomicU32;
use crate::time::Duration;

/// Wait for a futex_wake operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    // No need to wait if the value already changed.
    if futex.load(core::sync::atomic::Ordering::Relaxed) != expected {
        return true;
    }

    let runtime = twizzler_runtime_api::get_runtime();
    runtime.futex_wait(futex, expected, timeout)
}

/// Wake up one thread that's blocked on futex_wait on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
///
/// On some platforms, this always returns false.
pub fn futex_wake(futex: &AtomicU32) -> bool {
    let runtime = twizzler_runtime_api::get_runtime();
    runtime.futex_wake(futex)
}

/// Wake up all threads that are waiting on futex_wait on this futex.
pub fn futex_wake_all(futex: &AtomicU32) {
    let runtime = twizzler_runtime_api::get_runtime();
    runtime.futex_wake_all(futex)
}
