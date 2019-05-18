use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

struct Worker<T> {
    closure: Box<Fn(Sender<T>) + Send + 'static>,
}

impl<T> Worker<T> {
    fn new(closure: impl Fn(Sender<T>) + Send + 'static) -> Self {
        Self {
            closure: Box::new(closure),
        }
    }
}

pub struct Pool<T: Send + 'static> {
    queue: VecDeque<Worker<T>>,
    slots: usize,
    pending: usize,
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Pool<T>
where
    T: Send + 'static,
{
    pub fn new(sender: Sender<T>, receiver: Receiver<T>) -> Self {
        Self {
            queue: VecDeque::new(),
            slots: 8,
            pending: 0,
            sender,
            receiver,
        }
    }

    pub fn send(&mut self, closure: impl Fn(Sender<T>) + Send + 'static) {
        if (self.slots - self.pending) > 0 {
            let tx = self.sender.clone();
            thread::spawn(move || closure(tx));
            self.pending += 1;
        } else {
            self.queue.push_back(Worker::new(closure));
        }
    }

    pub fn receive(&mut self) -> Option<T> {
        if let Ok(result) = self.receiver.try_recv() {
            self.pending -= 1;

            for _ in 0..(self.slots - self.pending) {
                if let Some(worker) = self.queue.pop_back() {
                    let tx = self.sender.clone();
                    thread::spawn(move || (worker.closure)(tx));
                    self.pending += 1;
                }
            }

            Some(result)
        } else {
            None
        }
    }

    pub fn get_pending(&self) -> usize {
        self.pending
    }

    pub fn get_waiting(&self) -> usize {
        self.queue.len()
    }

    pub fn is_finished(&self) -> bool {
        self.pending == 0 && self.queue.is_empty()
    }

    pub fn is_running(&self) -> bool {
        !self.is_finished()
    }
}
