use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Unsafe utility function to set the wake flag to true.
///
/// # Safety
/// This function is marked as unsafe because it reconstructs an `Arc` from
/// a raw pointer. It must only be used within the context of `RawWaker`,
/// where the pointer was created by `Arc::into_raw`.
unsafe fn wake_flag(flag_ptr: *const ()) {
    // Reconstructs an Arc<Mutex<bool>> from the raw pointer.
    let flag_arc: Arc<Mutex<bool>> = Arc::from_raw(flag_ptr as *const Mutex<bool>);
    // Sets the flag to true to signal that the task should be woken up.
    *flag_arc.lock().unwrap() = true;
    // Converts the Arc back into a raw pointer without dropping it (to avoid decreasing the ref count).
    // The actual drop of the Arc will be handled either by `wake_raw_flag`
    // (in the case of a consuming wake) or by `drop_raw_flag` later.
    // Not storing the returned value (here, `let _ = ...`) ensures that
    // the destructor of the Arc is not called.
    let _ = Arc::into_raw(flag_arc);
}

/// Function called by the runtime to clone the *waker*.
/// It increments the reference count of the `Arc` for sharing.
unsafe fn clone_raw_flag(flag_ptr: *const ()) -> RawWaker {
    // Reconstructs the original Arc from the pointer.
    let flag_arc: Arc<Mutex<bool>> = Arc::from_raw(flag_ptr as *const Mutex<bool>);
    // Clones the Arc to increase the reference count.
    let flag_clone = Arc::clone(&flag_arc);
    // Prevents the original Arc from being freed by "forgetting" it
    // (we still own `flag_arc` inside `flag_clone`).
    std::mem::forget(flag_arc);
    // Creates a new RawWaker with the cloned pointer and the same function table.
    RawWaker::new(Arc::into_raw(flag_clone) as *const (), &VTABLE)
}

/// Function called when `waker.wake()` is invoked (consumes the waker).
/// It wakes up the task by calling `wake_flag` and then frees the Arc.
unsafe fn wake_raw_flag(flag_ptr: *const ()) {
    // Marks the flag as woken up.
    wake_flag(flag_ptr);
    // Frees the Arc since wake() consumes the waker.
    drop(Arc::from_raw(flag_ptr as *const Mutex<bool>));
}

/// Function called when `waker.wake_by_ref()` is invoked (without consuming the waker).
/// It wakes up the task without freeing the Arc (the waker remains usable).
unsafe fn wake_by_ref_raw_flag(flag_ptr: *const ()) {
    // Simply marks the flag as woken up.
    wake_flag(flag_ptr);
    // The Arc is not freed here because the waker is not consumed by wake_by_ref().
}

/// Function called to release the waker's resources when it is dropped.
unsafe fn drop_raw_flag(flag_ptr: *const ()) {
    // Releases the Arc by decrementing the reference count.
    drop(Arc::from_raw(flag_ptr as *const Mutex<bool>));
}

/// Function table for our custom RawWaker.
/// It associates the above functions with waker management operations.
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_raw_flag,
    wake_raw_flag,
    wake_by_ref_raw_flag,
    drop_raw_flag,
);

/// Convenience function to create a `Waker` from an `Arc<Mutex<bool>>`.
fn create_waker(flag: Arc<Mutex<bool>>) -> Waker {
    // Converts the Arc into a raw pointer and creates a RawWaker with the defined VTABLE.
    let raw_waker = RawWaker::new(Arc::into_raw(flag) as *const (), &VTABLE);
    // Converts the RawWaker into a usable Rust Waker.
    unsafe { Waker::from_raw(raw_waker) }
}

/// Simple Future that completes when the shared flag turns true.
struct FlagFuture {
    shared_flag: Arc<Mutex<bool>>,
}

impl Future for FlagFuture {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Checks the current value of the flag.
        let flag_value = *self.shared_flag.lock().unwrap();
        if flag_value {
            // If the flag is true, the task is woken up and the future can complete.
            Poll::Ready("Task woken up!")
        } else {
            // If the flag is false, we indicate to the runtime that the task is not ready.
            // (Normally, we would store the Waker here to be woken up later.
            // In this example, our custom waker modifies the flag.)
            println!("The task is not yet woken up, waiting...");
            Poll::Pending
        }
    }
}

#[test]
fn main() {
    // Initializes the shared wake-up flag to false.
    let shared_flag = Arc::new(Mutex::new(false));

    // Creates a custom Waker associated with this flag.
    let waker = create_waker(Arc::clone(&shared_flag));

    // Creates a task context from our Waker.

    // [Option1] Clone waker for context to avoid borrowing conflict
    let waker_for_context = waker.clone();
    let mut context = Context::from_waker(&waker_for_context);
    // [Option2] If wake_by_ref is acceptable (Waker not consumed by waker.wake())
    // let mut context = Context::from_waker(&waker);

    // Instantiates the future that will use the same wake-up flag.
    let mut future = FlagFuture {
        shared_flag: Arc::clone(&shared_flag),
    };

    // First poll call: the task is not yet completed, we expect Poll::Pending.
    match Pin::new(&mut future).poll(&mut context) {
        Poll::Pending => println!("First poll: not ready yet, task is pending."),
        Poll::Ready(msg) => println!("First poll: {}", msg),
    }

    // Simulating an external event after some time...
    println!("An external event occurs and calls waker.wake() to wake up the task.");
    // Wake up the task by explicitly calling wake() on the Waker.
    // [Option1]
    waker.wake();
    // [Option2]
    // waker.wake_by_ref();

    // After calling wake(), we can check that the shared flag has turned true.
    let flag_after = *shared_flag.lock().unwrap();
    println!("Flag after waker.wake() = {}", flag_after);

    // Second poll call: now that the flag is true, the future should complete (Poll::Ready).
    match Pin::new(&mut future).poll(&mut context) {
        Poll::Pending => {
            println!("Second poll: still pending (this should not happen).")
        }
        Poll::Ready(msg) => println!("Second poll: {}", msg),
    }
}
