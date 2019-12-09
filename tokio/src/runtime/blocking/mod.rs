//! Abstracts out the APIs necessary to `Runtime` for integrating the blocking
//! pool. When the `blocking` feature flag is **not** enabled, these APIs are
//! shells. This isolates the complexity of dealing with conditional
//! compilation.

//cfg_blocking_impl! {
//    mod pool;
//    pub(crate) use pool::{spawn_blocking, BlockingPool, Spawner};
//
//    mod schedule;
//    mod shutdown;
//    mod task;
//
//    use crate::runtime::{self, Builder, io, time};
//
//    pub(crate) fn create_blocking_pool(
//        builder: &Builder,
//        spawner: &runtime::Spawner,
//        io: &io::Handle,
//        time: &time::Handle,
//        clock: &time::Clock,
//    ) -> BlockingPool {
//        BlockingPool::new(
//            builder,
//            spawner,
//            io,
//            time,
//            clock)
//
//    }
//}
#[cfg(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
))]
mod pool;
#[cfg(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
))]
pub(crate) use pool::{spawn_blocking, BlockingPool, Spawner};
#[cfg(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
))]
mod schedule;
#[cfg(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
))]
mod shutdown;
#[cfg(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
))]
mod task;
#[cfg(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
))]
use crate::runtime::{self, io, time, Builder};
#[cfg(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
))]
pub(crate) fn create_blocking_pool(
    builder: &Builder,
    spawner: &runtime::Spawner,
    io: &io::Handle,
    time: &time::Handle,
    clock: &time::Clock,
) -> BlockingPool {
    BlockingPool::new(builder, spawner, io, time, clock)
}

//cfg_not_blocking_impl! {
//    use crate::runtime::{self, io, time, Builder};
//
//    #[derive(Debug, Clone)]
//    pub(crate) struct BlockingPool {}
//
//    pub(crate) use BlockingPool as Spawner;
//
//    pub(crate) fn create_blocking_pool(
//        _builder: &Builder,
//        _spawner: &runtime::Spawner,
//        _io: &io::Handle,
//        _time: &time::Handle,
//        _clock: &time::Clock,
//    ) -> BlockingPool {
//        BlockingPool {}
//    }
//
//    impl BlockingPool {
//        pub(crate) fn spawner(&self) -> &BlockingPool {
//            self
//        }
//
//        pub(crate) fn enter<F, R>(&self, f: F) -> R
//        where
//            F: FnOnce() -> R,
//        {
//            f()
//        }
//    }
//}
#[cfg(not(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
)))]
use crate::runtime::{self, io, time, Builder};
#[cfg(not(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
)))]
#[derive(Debug, Clone)]
pub(crate) struct BlockingPool {}
#[cfg(not(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
)))]
pub(crate) use BlockingPool as Spawner;
#[cfg(not(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
)))]
pub(crate) fn create_blocking_pool(
    _builder: &Builder,
    _spawner: &runtime::Spawner,
    _io: &io::Handle,
    _time: &time::Handle,
    _clock: &time::Clock,
) -> BlockingPool {
    BlockingPool {}
}
#[cfg(not(any(
    feature = "blocking",
    feature = "fs",
    feature = "dns",
    feature = "io-std",
    feature = "rt-threaded",
)))]
impl BlockingPool {
    pub(crate) fn spawner(&self) -> &BlockingPool {
        self
    }

    pub(crate) fn enter<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }
}
