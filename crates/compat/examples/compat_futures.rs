//! Example demonstrating futures compatibility with Aether tasks.
//!
//! Run with: cargo run --example compat_futures --features aether-task/std

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

use aether_compat::Compat;
use aether_core::Task;
use aether_core::clock::Clock;
use aether_platform::clock::FrozenClock;
use aether_task::context::TaskContext;

// Example 1: Wrap a standard async future as a Task
async fn async_function() -> i32 {
    42
}

// Example 2: Create a simple task
struct SimpleTask {
    count: u32,
}

impl Task<TaskContext<FrozenClock>> for SimpleTask {
    type Output = u32;

    fn poll(&mut self, _kernel: &TaskContext<FrozenClock>) -> core::task::Poll<Self::Output> {
        self.count += 1;
        if self.count >= 3 {
            core::task::Poll::Ready(self.count)
        } else {
            core::task::Poll::Pending
        }
    }
}

fn main() {
    println!("Aether Futures Compatibility Example");
    println!("====================================\n");

    // Example 1: Use a Rust async future as a Task
    println!("Example 1: Wrapping async future as Task");
    let future = Box::pin(async_function());
    let mut future_task = Compat::new(future);
    let clock = FrozenClock;
    let kernel = TaskContext::new(clock, clock.now());

    // Poll the wrapped future as a task
    match future_task.poll(&kernel) {
        core::task::Poll::Ready(value) => println!("  Future completed with value: {}\n", value),
        core::task::Poll::Pending => println!("  Future is still pending\n"),
    }

    // Example 2: Use a Task as a Future
    println!("Example 2: Wrapping Task as Future");
    let task = SimpleTask { count: 0 };
    let kernel = TaskContext::new(clock, clock.now());
    let mut task_future = Compat::with_kernel(task, kernel);

    // Poll the wrapped task as a future
    let dummy_waker = Arc::new(DummyWaker);
    let waker = Waker::from(dummy_waker);
    let mut cx = Context::from_waker(&waker);

    match Pin::new(&mut task_future).poll(&mut cx) {
        Poll::Ready(value) => println!("  Task completed with value: {}\n", value),
        Poll::Pending => println!("  Task is still pending\n"),
    }

    println!("Compatibility wrappers working correctly!");
}

/// Dummy waker for demonstration purposes
struct DummyWaker;

#[allow(clippy::manual_noop_waker)]
impl Wake for DummyWaker {
    fn wake(self: Arc<Self>) {}
}
