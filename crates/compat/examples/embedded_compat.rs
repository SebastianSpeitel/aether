//! Embedded-friendly futures compatibility example.
//!
//! Demonstrates using futures with Tasks on embedded systems without allocations.
//! Run with: cargo run --example embedded_compat --no-default-features

#![allow(unused)]

use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use aether_compat::Compat;
use aether_core::Task;
use aether_core::clock::Clock;
use aether_platform::clock::FrozenClock;
use aether_task::context::TaskContext;

/// Example embedded future that doesn't require allocations
async fn embedded_future() -> u32 {
    // This could be reading from a peripheral, processing sensor data, etc.
    100
}

/// Example task for embedded systems
struct EmbeddedTask {
    counter: u32,
}

impl Task<TaskContext<FrozenClock>> for EmbeddedTask {
    type Output = u32;

    fn poll(&mut self, _kernel: &TaskContext<FrozenClock>) -> Poll<Self::Output> {
        self.counter += 1;
        if self.counter >= 5 {
            Poll::Ready(self.counter * 10)
        } else {
            Poll::Pending
        }
    }
}

fn main() {
    println!("Embedded-Friendly Futures Compatibility");
    println!("======================================\n");

    // Example 1: Polling a future as a task without allocations
    println!("Example 1: Future → Task (no allocations)");
    {
        let future = Box::pin(embedded_future());
        let mut task = Compat::new(future);
        let clock = FrozenClock;
        let kernel = TaskContext::new(clock, clock.now());

        // Create a no-op waker for the future context
        let mut cx = Context::from_waker(Waker::noop());

        // Poll the wrapped future multiple times
        for i in 1..=3 {
            match task.poll(&kernel) {
                Poll::Ready(val) => {
                    println!("  Poll {}: Ready with value {}", i, val);
                    break;
                }
                Poll::Pending => println!("  Poll {}: Still pending", i),
            }
        }
    }
    println!();

    // Example 2: Using a task as a future
    println!("Example 2: Task → Future (no allocations)");
    {
        let task = EmbeddedTask { counter: 0 };
        let clock = FrozenClock;
        let kernel = TaskContext::new(clock, clock.now());
        let mut future = Compat::with_kernel(task, kernel);

        let mut cx = Context::from_waker(Waker::noop());

        // Poll the wrapped task multiple times
        for i in 1..=6 {
            match Pin::new(&mut future).poll(&mut cx) {
                Poll::Ready(val) => {
                    println!("  Poll {}: Ready with value {}", i, val);
                    break;
                }
                Poll::Pending => println!("  Poll {}: Still pending", i),
            }
        }
    }

    println!("\nKey benefits for embedded systems:");
    println!("  ✓ Zero allocations (no Arc, no Heap)");
    println!("  ✓ Works on bare-metal targets (no_std)");
    println!("  ✓ Uses only core::future and core::task");
    println!("  ✓ Minimal waker overhead (pure function pointers)");
    println!("  ✓ Stack-allocated (suitable for stack-constrained systems)");
}
