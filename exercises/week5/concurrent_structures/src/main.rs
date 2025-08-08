// Week 5: Concurrent Data Structures
// Build thread-safe data structures and implement common concurrency patterns

use std::sync::{Arc, Mutex, Condvar, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

// TODO 1: Thread-safe counter with multiple operations
struct ThreadSafeCounter {
    value: Mutex<i64>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            value: Mutex::new(0),
        }
    }
    
    fn increment(&self) -> i64 {
        let mut val = self.value.lock().unwrap();
        *val += 1;
        *val
    }
    
    fn decrement(&self) -> i64 {
        let mut val = self.value.lock().unwrap();
        *val -= 1;
        *val
    }
    
    fn add(&self, amount: i64) -> i64 {
        let mut val = self.value.lock().unwrap();
        *val += amount;
        *val
    }
    
    fn get(&self) -> i64 {
        *self.value.lock().unwrap()
    }
    
    fn reset(&self) -> i64 {
        let mut val = self.value.lock().unwrap();
        let old = *val;
        *val = 0;
        old
    }
}

fn thread_safe_counter_example() {
    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];
    
    // Multiple threads performing different operations
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            match i % 3 {
                0 => {
                    for _ in 0..100 {
                        counter.increment();
                    }
                    println!("Thread {}: Incremented 100 times", i);
                }
                1 => {
                    for _ in 0..50 {
                        counter.decrement();
                    }
                    println!("Thread {}: Decremented 50 times", i);
                }
                2 => {
                    counter.add(25);
                    println!("Thread {}: Added 25", i);
                }
                _ => unreachable!(),
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", counter.get());
}

// TODO 2: Producer-Consumer with bounded buffer
struct BoundedBuffer<T> {
    buffer: Mutex<VecDeque<T>>,
    not_full: Condvar,
    not_empty: Condvar,
    capacity: usize,
}

impl<T> BoundedBuffer<T> {
    fn new(capacity: usize) -> Self {
        BoundedBuffer {
            buffer: Mutex::new(VecDeque::with_capacity(capacity)),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
            capacity,
        }
    }
    
    fn put(&self, item: T) {
        let mut buffer = self.buffer.lock().unwrap();
        
        // Wait while buffer is full
        while buffer.len() == self.capacity {
            buffer = self.not_full.wait(buffer).unwrap();
        }
        
        let was_empty = buffer.is_empty();
        buffer.push_back(item);
        
        // Notify consumers if buffer was empty
        if was_empty {
            self.not_empty.notify_one();
        }
    }
    
    fn take(&self) -> T {
        let mut buffer = self.buffer.lock().unwrap();
        
        // Wait while buffer is empty
        while buffer.is_empty() {
            buffer = self.not_empty.wait(buffer).unwrap();
        }
        
        let was_full = buffer.len() == self.capacity;
        let item = buffer.pop_front().unwrap();
        
        // Notify producers if buffer was full
        if was_full {
            self.not_full.notify_one();
        }
        
        item
    }
    
    fn len(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }
}

fn producer_consumer_example() {
    let buffer = Arc::new(BoundedBuffer::new(5));
    let mut handles = vec![];
    
    // Producer threads
    for i in 0..3 {
        let buffer = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let item = i * 100 + j;
                buffer.put(item);
                println!("Producer {}: Put {}", i, item);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }
    
    // Consumer threads
    for i in 0..2 {
        let buffer = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            for _ in 0..15 {
                let item = buffer.take();
                println!("Consumer {}: Got {}", i, item);
                thread::sleep(Duration::from_millis(80));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Buffer final size: {}", buffer.len());
}

// TODO 3: Thread-safe priority queue
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct ThreadSafePriorityQueue<T: Ord> {
    heap: Mutex<BinaryHeap<T>>,
    not_empty: Condvar,
}

impl<T: Ord> ThreadSafePriorityQueue<T> {
    fn new() -> Self {
        ThreadSafePriorityQueue {
            heap: Mutex::new(BinaryHeap::new()),
            not_empty: Condvar::new(),
        }
    }
    
    fn push(&self, item: T) {
        let mut heap = self.heap.lock().unwrap();
        let was_empty = heap.is_empty();
        heap.push(item);
        
        if was_empty {
            self.not_empty.notify_one();
        }
    }
    
    fn pop(&self) -> Option<T> {
        let mut heap = self.heap.lock().unwrap();
        heap.pop()
    }
    
    fn pop_blocking(&self) -> T {
        let mut heap = self.heap.lock().unwrap();
        
        while heap.is_empty() {
            heap = self.not_empty.wait(heap).unwrap();
        }
        
        heap.pop().unwrap()
    }
    
    fn len(&self) -> usize {
        self.heap.lock().unwrap().len()
    }
}

fn priority_queue_example() {
    let pq = Arc::new(ThreadSafePriorityQueue::new());
    let mut handles = vec![];
    
    // Producer threads with different priorities
    for i in 0..3 {
        let pq = Arc::clone(&pq);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let priority = (i + j) % 10;
                pq.push(priority);
                println!("Producer {}: Added priority {}", i, priority);
                thread::sleep(Duration::from_millis(30));
            }
        });
        handles.push(handle);
    }
    
    // Consumer thread
    let pq_clone = Arc::clone(&pq);
    let consumer = thread::spawn(move || {
        for _ in 0..15 {
            let item = pq_clone.pop_blocking();
            println!("Consumer: Got priority {}", item);
        }
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    consumer.join().unwrap();
}

// TODO 4: Work-stealing queue simulation
struct WorkStealingQueue<T> {
    local_queue: Mutex<VecDeque<T>>,
    steal_queue: Mutex<VecDeque<T>>,
}

impl<T> WorkStealingQueue<T> {
    fn new() -> Self {
        WorkStealingQueue {
            local_queue: Mutex::new(VecDeque::new()),
            steal_queue: Mutex::new(VecDeque::new()),
        }
    }
    
    fn push_local(&self, item: T) {
        self.local_queue.lock().unwrap().push_back(item);
    }
    
    fn pop_local(&self) -> Option<T> {
        self.local_queue.lock().unwrap().pop_back()
    }
    
    fn steal(&self) -> Option<T> {
        // Try to steal from the front (oldest items)
        self.local_queue.lock().unwrap().pop_front()
    }
    
    fn len(&self) -> usize {
        self.local_queue.lock().unwrap().len()
    }
}

fn work_stealing_example() {
    let num_workers = 4;
    let queues: Vec<Arc<WorkStealingQueue<i32>>> = (0..num_workers)
        .map(|_| Arc::new(WorkStealingQueue::new()))
        .collect();
    
    let mut handles = vec![];
    
    // Worker threads
    for worker_id in 0..num_workers {
        let queues = queues.clone();
        let handle = thread::spawn(move || {
            let my_queue = &queues[worker_id];
            
            // Add some work to own queue
            for i in 0..5 {
                my_queue.push_local(worker_id * 100 + i);
            }
            
            // Work loop
            for _ in 0..10 {
                // Try to get work from own queue first
                if let Some(work) = my_queue.pop_local() {
                    println!("Worker {} processing own work: {}", worker_id, work);
                } else {
                    // Try to steal work from other workers
                    let mut found_work = false;
                    for other_id in 0..num_workers {
                        if other_id != worker_id {
                            if let Some(work) = queues[other_id].steal() {
                                println!("Worker {} stole work {} from worker {}", 
                                        worker_id, work, other_id);
                                found_work = true;
                                break;
                            }
                        }
                    }
                    if !found_work {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
                thread::sleep(Duration::from_millis(20));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Show remaining work
    for (i, queue) in queues.iter().enumerate() {
        println!("Queue {}: {} items remaining", i, queue.len());
    }
}

// TODO 5: Rate limiter
struct RateLimiter {
    tokens: Mutex<f64>,
    capacity: f64,
    refill_rate: f64, // tokens per second
    last_refill: Mutex<Instant>,
}

impl RateLimiter {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        RateLimiter {
            tokens: Mutex::new(capacity),
            capacity,
            refill_rate,
            last_refill: Mutex::new(Instant::now()),
        }
    }
    
    fn try_acquire(&self, tokens: f64) -> bool {
        let mut token_count = self.tokens.lock().unwrap();
        let mut last_refill = self.last_refill.lock().unwrap();
        
        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;
        
        *token_count = (*token_count + new_tokens).min(self.capacity);
        *last_refill = now;
        
        // Try to consume tokens
        if *token_count >= tokens {
            *token_count -= tokens;
            true
        } else {
            false
        }
    }
    
    fn get_available_tokens(&self) -> f64 {
        let token_count = self.tokens.lock().unwrap();
        *token_count
    }
}

fn rate_limiter_example() {
    let limiter = Arc::new(RateLimiter::new(10.0, 2.0)); // 10 capacity, 2 tokens/sec
    let mut handles = vec![];
    
    for i in 0..5 {
        let limiter = Arc::clone(&limiter);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                if limiter.try_acquire(1.0) {
                    println!("Thread {} - Request {} ALLOWED", i, j);
                } else {
                    println!("Thread {} - Request {} DENIED (rate limited)", i, j);
                }
                thread::sleep(Duration::from_millis(200));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// TODO 6: Multi-producer, multi-consumer queue
struct MPMCQueue<T> {
    queue: Mutex<VecDeque<T>>,
    not_empty: Condvar,
}

impl<T> MPMCQueue<T> {
    fn new() -> Self {
        MPMCQueue {
            queue: Mutex::new(VecDeque::new()),
            not_empty: Condvar::new(),
        }
    }
    
    fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        let was_empty = queue.is_empty();
        queue.push_back(item);
        
        if was_empty {
            self.not_empty.notify_all(); // Notify all waiting consumers
        }
    }
    
    fn dequeue(&self) -> Option<T> {
        self.queue.lock().unwrap().pop_front()
    }
    
    fn dequeue_blocking(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        
        queue.pop_front().unwrap()
    }
}

fn mpmc_queue_example() {
    let queue = Arc::new(MPMCQueue::new());
    let mut handles = vec![];
    
    // Multiple producers
    for i in 0..3 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let item = format!("P{}-Item{}", i, j);
                queue.enqueue(item.clone());
                println!("Producer {}: Enqueued {}", i, item);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }
    
    // Multiple consumers
    for i in 0..2 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for _ in 0..7 {
                let item = queue.dequeue_blocking();
                println!("Consumer {}: Dequeued {}", i, item);
                thread::sleep(Duration::from_millis(150));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("=== Concurrent Data Structures ===\n");
    
    println!("--- Thread-Safe Counter ---");
    thread_safe_counter_example();
    
    println!("\n--- Producer-Consumer with Bounded Buffer ---");
    producer_consumer_example();
    
    println!("\n--- Thread-Safe Priority Queue ---");
    priority_queue_example();
    
    println!("\n--- Work Stealing Queue ---");
    work_stealing_example();
    
    println!("\n--- Rate Limiter ---");
    rate_limiter_example();
    
    println!("\n--- MPMC Queue ---");
    mpmc_queue_example();
    
    println!("\n=== All concurrent structure examples completed! ===");
}
