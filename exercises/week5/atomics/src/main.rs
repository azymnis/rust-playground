// Week 5: Atomic Types and Lock-Free Programming
// Learn about atomic operations and memory ordering

use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicI32, AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

// TODO 1: Basic atomic operations
fn basic_atomic_operations() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Atomic counter result: {}", counter.load(Ordering::SeqCst));
}

// TODO 2: Memory ordering examples
fn memory_ordering_examples() {
    let data = Arc::new(AtomicI32::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    
    let data_clone = Arc::clone(&data);
    let flag_clone = Arc::clone(&flag);
    
    // Producer thread
    let producer = thread::spawn(move || {
        // Write data
        data_clone.store(42, Ordering::Relaxed);
        // Set flag with release ordering
        flag_clone.store(true, Ordering::Release);
        println!("Producer: Data written and flag set");
    });
    
    let data_clone = Arc::clone(&data);
    let flag_clone = Arc::clone(&flag);
    
    // Consumer thread
    let consumer = thread::spawn(move || {
        // Wait for flag with acquire ordering
        while !flag_clone.load(Ordering::Acquire) {
            thread::yield_now();
        }
        // Read data - guaranteed to see the write due to acquire-release pair
        let value = data_clone.load(Ordering::Relaxed);
        println!("Consumer: Read value {}", value);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

// TODO 3: Compare and swap operations
fn compare_and_swap() {
    let value = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            loop {
                let current = value.load(Ordering::SeqCst);
                let new_value = current + i + 1;
                
                // Try to update only if value hasn't changed
                match value.compare_exchange_weak(
                    current,
                    new_value,
                    Ordering::SeqCst,
                    Ordering::SeqCst
                ) {
                    Ok(_) => {
                        println!("Thread {}: Successfully updated {} -> {}", i, current, new_value);
                        break;
                    }
                    Err(actual) => {
                        println!("Thread {}: Failed to update, expected {}, found {}", i, current, actual);
                        // Retry with backoff
                        thread::yield_now();
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final value: {}", value.load(Ordering::SeqCst));
}

// TODO 4: Spinlock implementation
struct Spinlock {
    locked: AtomicBool,
}

impl Spinlock {
    fn new() -> Self {
        Spinlock {
            locked: AtomicBool::new(false),
        }
    }
    
    fn lock(&self) {
        while self.locked.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ).is_err() {
            // Spin while locked
            while self.locked.load(Ordering::Relaxed) {
                thread::yield_now();
            }
        }
    }
    
    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn spinlock_example() {
    let spinlock = Arc::new(Spinlock::new());
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let spinlock = Arc::clone(&spinlock);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                spinlock.lock();
                let current = counter.load(Ordering::Relaxed);
                counter.store(current + 1, Ordering::Relaxed);
                spinlock.unlock();
            }
            println!("Thread {} finished", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Spinlock protected counter: {}", counter.load(Ordering::Relaxed));
}

// TODO 5: Lock-free stack
struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
    
    fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: std::ptr::null_mut(),
        }));
        
        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }
            
            if self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
    }
    
    fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            
            let next = unsafe { (*head).next };
            
            if self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed
            ).is_ok() {
                let data = unsafe { Box::from_raw(head).data };
                return Some(data);
            }
        }
    }
}

// Note: This implementation has ABA problem and memory leaks
// In production, use hazard pointers or epoch-based reclamation
fn lock_free_stack_example() {
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];
    
    // Producer threads
    for i in 0..3 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                stack.push(i * 10 + j);
                println!("Pushed: {}", i * 10 + j);
            }
        });
        handles.push(handle);
    }
    
    // Consumer threads
    for i in 0..2 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            for _ in 0..7 {
                if let Some(value) = stack.pop() {
                    println!("Consumer {}: Popped {}", i, value);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// TODO 6: Atomic flags for coordination
fn atomic_flags_coordination() {
    let ready = Arc::new(AtomicBool::new(false));
    let done = Arc::new(AtomicBool::new(false));
    
    let ready_clone = Arc::clone(&ready);
    let done_clone = Arc::clone(&done);
    
    let worker = thread::spawn(move || {
        println!("Worker: Waiting for ready signal...");
        while !ready_clone.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(10));
        }
        
        println!("Worker: Got ready signal, starting work...");
        thread::sleep(Duration::from_millis(200));
        println!("Worker: Work completed!");
        
        done_clone.store(true, Ordering::Release);
    });
    
    thread::sleep(Duration::from_millis(100));
    println!("Main: Sending ready signal...");
    ready.store(true, Ordering::Release);
    
    // Wait for completion
    while !done.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(10));
    }
    
    println!("Main: Worker completed!");
    worker.join().unwrap();
}

// TODO 7: Performance comparison: Atomic vs Mutex
fn performance_comparison() {
    const ITERATIONS: usize = 1_000_000;
    const THREADS: usize = 4;
    
    // Atomic performance test
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS / THREADS {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_time = start.elapsed();
    println!("Atomic counter time: {:?}, result: {}", 
             atomic_time, atomic_counter.load(Ordering::Relaxed));
    
    // Mutex performance test
    let mutex_counter = Arc::new(std::sync::Mutex::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&mutex_counter);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS / THREADS {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_time = start.elapsed();
    println!("Mutex counter time: {:?}, result: {}", 
             mutex_time, *mutex_counter.lock().unwrap());
    
    println!("Atomic was {:.2}x faster", 
             mutex_time.as_nanos() as f64 / atomic_time.as_nanos() as f64);
}

// TODO 8: Atomic reference counting simulation
struct SimpleArc<T> {
    data: *const T,
    ref_count: AtomicUsize,
}

impl<T> SimpleArc<T> {
    fn new(data: T) -> Self {
        let boxed = Box::new(data);
        SimpleArc {
            data: Box::into_raw(boxed),
            ref_count: AtomicUsize::new(1),
        }
    }
    
    fn clone(&self) -> Self {
        self.ref_count.fetch_add(1, Ordering::Relaxed);
        SimpleArc {
            data: self.data,
            ref_count: AtomicUsize::new(0), // This is wrong but simplified for demo
        }
    }
    
    fn get_ref_count(&self) -> usize {
        self.ref_count.load(Ordering::Relaxed)
    }
}

// Note: This is a simplified example - real Arc is much more complex
fn atomic_reference_counting() {
    let data = SimpleArc::new(42);
    println!("Initial ref count: {}", data.get_ref_count());
    
    let data_clone = data.clone();
    println!("After clone ref count: {}", data.get_ref_count());
}

fn main() {
    println!("=== Atomic Types and Lock-Free Programming ===\n");
    
    println!("--- Basic Atomic Operations ---");
    basic_atomic_operations();
    
    println!("\n--- Memory Ordering Examples ---");
    memory_ordering_examples();
    
    println!("\n--- Compare and Swap ---");
    compare_and_swap();
    
    println!("\n--- Spinlock Example ---");
    spinlock_example();
    
    println!("\n--- Lock-Free Stack ---");
    lock_free_stack_example();
    
    println!("\n--- Atomic Flags Coordination ---");
    atomic_flags_coordination();
    
    println!("\n--- Performance Comparison ---");
    performance_comparison();
    
    println!("\n--- Atomic Reference Counting ---");
    atomic_reference_counting();
    
    println!("\n=== All atomic examples completed! ===");
}
