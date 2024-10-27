use crate::http_manager::HTTPManager;
use crate::thread_pool::ThreadPool;
use std::sync::Arc;
use std::thread;

pub struct MainConfig {
    pub worker_threads: Option<usize>,
}

pub struct Oyster {
    pub http: HTTPManager,
    pub thread_pool: Arc<ThreadPool>,
    pub worker_threads: usize,
}

impl Oyster {
    pub fn new(main_config: MainConfig) -> Self {
        let worker_threads = main_config.worker_threads.unwrap_or_else(|| {
            thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        });

        Oyster {
            http: HTTPManager::new(),
            thread_pool: Arc::new(ThreadPool::new(worker_threads)),
            worker_threads,
        }
    }

    pub fn default() -> Self {
        Self::new(MainConfig {
            worker_threads: None,
        })
    }

    pub fn start(&self) {
        println!("worker_threads is: {}", self.worker_threads);
        self.http.start(Arc::clone(&self.thread_pool));
    }
}
