use async_lapin::LapinAsyncIoExt;
use lapin::ConnectionProperties;

// ConnectionProperties extension

#[deprecated(note = "use async-executor-trait and async-reactor-trait directly instead")]
pub trait LapinAsyncStdExt {
    #[deprecated(note = "use async-executor-trait and async-reactor-trait directly instead")]
    fn with_async_std(self) -> Self
    where
        Self: Sized,
    {
        self.with_async_std_executor().with_async_std_reactor()
    }

    #[deprecated(note = "use async-executor-trait directly instead")]
    fn with_async_std_executor(self) -> Self
    where
        Self: Sized;

    #[deprecated(note = "use async-reactor-trait directly instead")]
    fn with_async_std_reactor(self) -> Self
    where
        Self: Sized;
}

impl LapinAsyncStdExt for ConnectionProperties {
    fn with_async_std_executor(self) -> Self {
        self.with_executor(async_executor_trait::AsyncStd)
    }

    fn with_async_std_reactor(self) -> Self {
        // async-std uses async-io underneath, use async-io reactor until async-std exposes its own API
        self.with_async_io_reactor()
    }
}
