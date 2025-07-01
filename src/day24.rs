use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub trait SleighTask: Send {
    fn describe(&self) -> String;
}

#[derive(Default)]
pub struct SantaSleighQueue {
    records: Mutex<VecDeque<Box<dyn SleighTask>>>,
}

impl SantaSleighQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&self, item: Box<dyn SleighTask>) {
        self.records.lock().unwrap().push_back(item);
    }

    pub fn get_task(&self) -> Option<Box<dyn SleighTask>> {
        self.records.lock().unwrap().pop_front()
    }
}

pub struct ElfTask {
    name: String,
    urgency: u32,
}

impl ElfTask {
    pub fn new<S: AsRef<str>>(name: S, urgency: u32) -> Self {
        Self {
            name: name.as_ref().into(),
            urgency,
        }
    }
}

impl SleighTask for ElfTask {
    fn describe(&self) -> String {
        format!("Elf task: {} (urgency {})", self.name, self.urgency)
    }
}

pub struct ReindeerTask {
    name: String,
    weight: u32,
}

impl ReindeerTask {
    pub fn new<S: AsRef<str>>(name: S, weight: u32) -> Self {
        Self {
            name: name.as_ref().into(),
            weight,
        }
    }
}

impl SleighTask for ReindeerTask {
    fn describe(&self) -> String {
        format!("Reindeer task: {} ({} kg)", self.name, self.weight)
    }
}

pub fn main() {
    let queue = Arc::new(SantaSleighQueue::new());

    let producer_queue = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        producer_queue.enqueue(Box::new(ReindeerTask::new("Deliver Toys", 100)));
        producer_queue.enqueue(Box::new(ElfTask::new("Wrap Gifts", 3)));
        producer_queue.enqueue(Box::new(ReindeerTask::new("Collect Reindeer Feed", 50)));
        producer_queue.enqueue(Box::new(ElfTask::new("Decorate Tree", 7)));
    });

    thread::sleep(std::time::Duration::from_millis(10));

    let consumer_queue = Arc::clone(&queue);
    let consumer = thread::spawn(move || {
        while let Some(task) = consumer_queue.get_task() {
            println!("{}", task.describe());
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
