use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum TaskKind {
    Cpu,
    Io,
}

#[derive(Debug, Clone)]
pub struct Task {
    id: usize,
    kind: TaskKind,
    arrival_time_ms: u64,
    duration_ms: u64,
    cpu_cost: f64,
}

pub struct TaskQueue {
    queue: VecDeque<Task>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue { queue: VecDeque::new() }
    }
    pub fn push(&mut self, task: Task) {
        self.queue.push_back(task);
    }
    pub fn pop(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

pub struct MonitorSnapshot {
    time_ms: u64,
    cpu_consumption: f64,
    active_workers: usize,
}

pub struct MonitorData {
    snapshots: Vec<MonitorSnapshot>,
}

fn generate_tasks() -> Vec<Task> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut tasks = Vec::new();
    for i in 0..1000 {
        let kind = if rng.gen_bool(0.7) { TaskKind::Io } else { TaskKind::Cpu };
        let cpu_cost = match kind {
            TaskKind::Cpu => 0.35,
            TaskKind::Io => 0.10,
        };
        tasks.push(Task {
            id: i,
            kind,
            cpu_cost,
            arrival_time_ms: (i as u64) * 20,
            duration_ms: 200,
        });
    }
    tasks
}

fn main() {
    
    let queue = Arc::new(Mutex::new(TaskQueue::new()));
    let done = Arc::new(Mutex::new(false));
    let monitor_data = Arc::new(Mutex::new(MonitorData { snapshots: Vec::new() }));

    
    let queue_sender = Arc::clone(&queue);
    let done_sender = Arc::clone(&done);
    let sender_handle = thread::spawn(move || {
    let tasks = generate_tasks();
        for task in tasks {
        thread::sleep(...);
        
        }
      
    });

    
    let mut worker_handles = Vec::new();
    for _ in 0..8 {
        let queue_worker = Arc::clone(&queue);
        let done_worker = Arc::clone(&done);
        let monitor_worker = Arc::clone(&monitor_data);
        let handle = thread::spawn(move || {
            
        });
        worker_handles.push(handle);
    }

    
    let queue_monitor = Arc::clone(&queue);
    let done_monitor = Arc::clone(&done);
    let monitor_data_monitor = Arc::clone(&monitor_data);
    let monitor_handle = thread::spawn(move || {
        
    });
    
    
    sender_handle.join().unwrap();
    
    for h in worker_handles { 
        h.join().unwrap(); 
    }
    
    monitor_handle.join().unwrap();
}