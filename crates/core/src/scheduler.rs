use core::error::Error;

pub trait Scheduler {
    type Error: Error;
    type TaskHandle<T>;

    fn spawn<T>(&mut self, task: T) -> Result<Self::TaskHandle<T>, Self::Error>;
}
