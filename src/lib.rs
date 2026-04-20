use std::thread;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn build(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("Size must be > 0");
        }

        Ok(ThreadPool::new(size))
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f);
    }
}