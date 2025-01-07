use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};

static HIGH_PRIORITY: LazyLock<Runtime> = LazyLock::new(|| {
  Builder::new_multi_thread()
    .worker_threads(2)
    .thread_name("High Priority Runtime")
    .enable_time()
    .build()
    .unwrap()
});
static LOW_PRIORITY: LazyLock<Runtime> = LazyLock::new(|| {
  Builder::new_multi_thread()
    .worker_threads(1)
    .thread_name("Low Priority Runtime")
    .enable_time()
    .build()
    .unwrap()
});
