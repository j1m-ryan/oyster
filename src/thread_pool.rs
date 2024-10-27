use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

type Task = Box<dyn FnOnce() + Send + 'static>;
type TaskQueue = (Mutex<VecDeque<Task>>, Condvar);

pub struct ThreadPool {
    workers: Vec<Worker>,
    task_queue: Arc<TaskQueue>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let task_queue = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let queue_clone = Arc::clone(&task_queue);
            workers.push(Worker::new(id, queue_clone));
        }
        ThreadPool {
            workers,
            task_queue,
        }
    }

    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (ref lock, ref cvar) = *self.task_queue;
        let mut queue = lock.lock().unwrap();
        queue.push_back(Box::new(task));
        cvar.notify_one();
    }
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(_id: usize, task_queue: Arc<TaskQueue>) -> Self {
        let thread = std::thread::spawn(move || {
            let (lock, cvar) = &*task_queue;
            loop {
                let task = {
                    let mut queue = lock.lock().unwrap();
                    while queue.is_empty() {
                        queue = cvar.wait(queue).unwrap();
                    }
                    queue.pop_front()
                };

                if let Some(task) = task {
                    task();
                } else {
                    break;
                }
            }
        });

        Worker {
            id: _id,
            thread: Some(thread),
        }
    }
}
