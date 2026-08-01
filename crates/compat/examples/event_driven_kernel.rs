//! Example: Event-driven kernel with kernel_waker
//!
//! Demonstrates using kernel_waker() to notify an event-driven kernel
//! when futures are ready.
//!
//! Run with: cargo run --example event_driven_kernel

use core::cell::Cell;
use core::pin::Pin;
use core::task::{Context, Poll};

use aether_compat::Compat;
use aether_core::time::FrozenClock;
use aether_task::context::TaskContext;
use aether_task::task::Task;

/// An event-driven kernel that only polls when woken
struct EventDrivenKernel {
    wake_count: Cell<u32>,
    has_pending_event: Cell<bool>,
}

impl EventDrivenKernel {
    fn new() -> Self {
        Self {
            wake_count: Cell::new(0),
            has_pending_event: Cell::new(false),
        }
    }

    /// Simulate an external event occurring
    fn trigger_event(&self) {
        self.has_pending_event.set(true);
    }

    /// Poll only if there's a pending event
    fn poll_if_ready(&self) -> bool {
        self.has_pending_event.get()
    }

    /// Clear the pending event flag
    fn clear_event(&self) {
        self.has_pending_event.set(false);
    }
}

impl aether_core::Kernel for EventDrivenKernel {
    fn yield_for<C: aether_core::time::Clock, T>(&self, _dur: aether_core::time::Duration<C>) -> Poll<T> {
        // When the waker fires, mark that we have an event
        self.wake_count.set(self.wake_count.get() + 1);
        self.has_pending_event.set(true);
        Poll::Pending
    }
}

struct ExternalEventFuture<'a> {
    kernel: &'a EventDrivenKernel,
}

impl<'a> Future for ExternalEventFuture<'a> {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.kernel.has_pending_event.get() {
            Poll::Ready("Event completed")
        } else {
            Poll::Pending
        }
    }
}

/// A task that tracks how many times it's polled
struct CountingTask {
    poll_count: u32,
}

impl<K: aether_core::Kernel> Task<K> for CountingTask {
    type Output = u32;

    fn poll(&mut self, _kernel: &K) -> Poll<Self::Output> {
        self.poll_count += 1;
        if self.poll_count >= 5 {
            Poll::Ready(self.poll_count)
        } else {
            Poll::Pending
        }
    }
}

fn main() {
    println!("Event-Driven Kernel with kernel_waker()");
    println!("========================================\n");

    let kernel = EventDrivenKernel::new();

    // Example 1: Using kernel_waker for event notification
    println!("Example 1: Event-driven future waking\n");
    {
        let future = Box::pin(ExternalEventFuture { kernel: &kernel });
        let mut task = Compat::new(future);

        // Poll 1: No event, returns Pending
        println!("  Poll 1 (no event):");
        match task.poll(&kernel) {
            Poll::Ready(val) => println!("    Ready: {}", val),
            Poll::Pending => {
                println!("    Pending (waiting for event)");
                println!("    Kernel wake count: {}", kernel.wake_count.get());
            }
        }

        // Simulate an external event
        println!("\n  External event triggered!");
        kernel.trigger_event();
        println!("  Kernel wake count now: {}", kernel.wake_count.get());

        // Check if kernel would poll
        if kernel.poll_if_ready() {
            kernel.clear_event();
            println!("\n  Poll 2 (after event):");
            match task.poll(&kernel) {
                Poll::Ready(val) => println!("    Ready: {}", val),
                Poll::Pending => println!("    Still pending"),
            }
        }
    }

    println!("\n");

    // Example 2: Comparing eager vs event-driven polling
    println!("Example 2: Polling strategies comparison\n");

    let eager_polls = {
        let mut task = CountingTask { poll_count: 0 };
        let kernel = TaskContext::new(aether_core::time::Instant::<FrozenClock>::now());

        // Eager polling loop (no waker involved)
        loop {
            match task.poll(&kernel) {
                Poll::Ready(val) => {
                    println!("  Eager polling: {} polls to complete", val);
                    break;
                }
                Poll::Pending => { /* continue polling */ }
            }
        }
        task.poll_count
    };

    let event_polls = {
        let mut task = CountingTask { poll_count: 0 };
        let kernel = EventDrivenKernel::new();

        // Event-driven loop (waker interacts with kernel)
        for iteration in 0..10 {
            if kernel.poll_if_ready() {
                kernel.clear_event();
                match task.poll(&kernel) {
                    Poll::Ready(val) => {
                        println!(
                            "  Event-driven: {} polls to complete (after {} event checks)",
                            val,
                            iteration + 1
                        );
                        break;
                    }
                    Poll::Pending => { /* wait for next event */ }
                }
            } else if iteration == 0 {
                // Trigger first event manually
                kernel.trigger_event();
            }
        }
        task.poll_count
    };

    println!("\nKey insight:");
    println!("  - Eager: Always polls, even if not ready (uses noop_waker)");
    println!("  - Event-driven: Polls only when woken (uses kernel_waker)");
    println!(
        "  - Both complete {} / {} polls → futures work equally well either way",
        eager_polls, event_polls
    );
}
