// Week 5: Testing Concurrency
// Learn strategies and techniques for testing multithreaded code

use std::sync::{Arc, Mutex, Barrier, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Example concurrent data structure to test
struct ConcurrentCounter {
    value: Mutex<i32>,
}

impl ConcurrentCounter {
    fn new() -> Self {
        ConcurrentCounter {
            value: Mutex::new(0),
        }
    }
    
    fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
    }
    
    fn decrement(&self) {
        let mut val = self.value.lock().unwrap();
        *val -= 1;
    }
    
    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
    
    fn add(&self, amount: i32) {
        let mut val = self.value.lock().unwrap();
        *val += amount;
    }
}

// TODO 1: Basic stress testing
fn stress_test_counter() {
    println!("--- Stress Test: Counter ---");
    let counter = Arc::new(ConcurrentCounter::new());
    let mut handles = vec![];
    
    let num_threads = 10;
    let increments_per_thread = 1000;
    
    for i in 0..num_threads {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                if i % 2 == 0 {
                    counter.increment();
                } else {
                    counter.decrement();
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_value = counter.get();
    let expected = 0; // Equal increments and decrements
    
    println!("Final counter value: {}", final_value);
    println!("Expected value: {}", expected);
    println!("Test {}", if final_value == expected { "PASSED" } else { "FAILED" });
}

// TODO 2: Deterministic testing with barriers
fn deterministic_test_with_barriers() {
    println!("\n--- Deterministic Test with Barriers ---");
    let counter = Arc::new(ConcurrentCounter::new());
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    // Phase 1: All threads start together
    for i in 0..3 {
        let counter = Arc::clone(&counter);
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();
            
            // Perform operations in lockstep
            counter.add(i + 1);
            
            // Synchronize again
            barrier.wait();
            
            // Check intermediate state
            let value = counter.get();
            println!("Thread {} sees value: {}", i, value);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final value: {} (expected: 6)", counter.get());
}

// TODO 3: Race condition detection
struct UnsafeCounter {
    value: i32, // No mutex - intentionally unsafe!
}

impl UnsafeCounter {
    fn new() -> Self {
        UnsafeCounter { value: 0 }
    }
    
    fn increment(&mut self) {
        let temp = self.value;
        thread::sleep(Duration::from_nanos(1)); // Increase chance of race
        self.value = temp + 1;
    }
    
    fn get(&self) -> i32 {
        self.value
    }
}

fn detect_race_conditions() {
    println!("\n--- Race Condition Detection ---");
    
    // Run multiple iterations to catch race conditions
    let mut race_detected = false;
    
    for iteration in 0..10 {
        let counter = Arc::new(Mutex::new(UnsafeCounter::new()));
        let mut handles = vec![];
        
        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..20 {
                    counter.lock().unwrap().increment();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_value = counter.lock().unwrap().get();
        let expected = 100;
        
        if final_value != expected {
            println!("Iteration {}: Race detected! Got {}, expected {}", 
                    iteration, final_value, expected);
            race_detected = true;
        }
    }
    
    if !race_detected {
        println!("No races detected in this run (try running multiple times)");
    }
}

// TODO 4: Testing for deadlocks with timeouts
fn test_for_deadlocks() {
    println!("\n--- Deadlock Detection Test ---");
    
    let resource1 = Arc::new(Mutex::new(1));
    let resource2 = Arc::new(Mutex::new(2));
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        
        // Try to acquire second resource with timeout
        match r2.try_lock() {
            Ok(_guard2) => println!("Thread 1: Acquired both locks"),
            Err(_) => println!("Thread 1: Could not acquire second lock"),
        }
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        let _guard2 = r2.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        
        // Try to acquire first resource with timeout
        match r1.try_lock() {
            Ok(_guard1) => println!("Thread 2: Acquired both locks"),
            Err(_) => println!("Thread 2: Could not acquire second lock"),
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("Deadlock test completed (no deadlock occurred)");
}

// TODO 5: Property-based testing for concurrency
fn property_based_testing() {
    println!("\n--- Property-Based Testing ---");
    
    // Property: Total operations should equal final state change
    let counter = Arc::new(ConcurrentCounter::new());
    let operations = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for thread_id in 0..8 {
        let counter = Arc::clone(&counter);
        let operations = Arc::clone(&operations);
        let handle = thread::spawn(move || {
            for i in 0..50 {
                if (thread_id + i) % 3 == 0 {
                    counter.increment();
                    operations.fetch_add(1, Ordering::SeqCst);
                } else if (thread_id + i) % 3 == 1 {
                    counter.decrement();
                    operations.fetch_add(1, Ordering::SeqCst);
                } else {
                    counter.add(2);
                    operations.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_operations = operations.load(Ordering::SeqCst);
    println!("Total operations performed: {}", total_operations);
    println!("Final counter value: {}", counter.get());
    println!("Property verified: All operations completed atomically");
}

// TODO 6: Performance testing under load
fn performance_testing() {
    println!("\n--- Performance Testing ---");
    
    let counter = Arc::new(ConcurrentCounter::new());
    let iterations = 100_000;
    let thread_counts = vec![1, 2, 4, 8];
    
    for num_threads in thread_counts {
        let counter = Arc::clone(&counter);
        let start_time = Instant::now();
        let mut handles = vec![];
        
        for _ in 0..num_threads {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..iterations / num_threads {
                    counter.increment();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start_time.elapsed();
        let ops_per_second = iterations as f64 / duration.as_secs_f64();
        
        println!("Threads: {}, Time: {:?}, Ops/sec: {:.0}", 
                num_threads, duration, ops_per_second);
        
        // Reset counter for next test
        let final_value = counter.get();
        for _ in 0..final_value {
            counter.decrement();
        }
    }
}

// TODO 7: Memory ordering testing
use std::sync::atomic::AtomicI32;

struct DataWithFlag {
    data: AtomicI32,
    flag: std::sync::atomic::AtomicBool,
}

fn memory_ordering_test() {
    println!("\n--- Memory Ordering Test ---");
    
    let shared = Arc::new(DataWithFlag {
        data: AtomicI32::new(0),
        flag: std::sync::atomic::AtomicBool::new(false),
    });
    
    let mut handles = vec![];
    let successful_reads = Arc::new(AtomicUsize::new(0));
    
    // Writer thread
    let shared_writer = Arc::clone(&shared);
    let writer = thread::spawn(move || {
        for i in 1..=1000 {
            shared_writer.data.store(i, Ordering::Relaxed);
            shared_writer.flag.store(true, Ordering::Release);
            thread::sleep(Duration::from_micros(10));
            shared_writer.flag.store(false, Ordering::Relaxed);
        }
    });
    
    // Reader threads
    for _ in 0..3 {
        let shared_reader = Arc::clone(&shared);
        let successful_reads = Arc::clone(&successful_reads);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                if shared_reader.flag.load(Ordering::Acquire) {
                    let value = shared_reader.data.load(Ordering::Relaxed);
                    if value > 0 {
                        successful_reads.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    writer.join().unwrap();
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Successful reads with proper ordering: {}", 
            successful_reads.load(Ordering::Relaxed));
}

// TODO 8: Testing thread safety invariants
struct BankAccount {
    balance: Mutex<i32>,
    transaction_count: AtomicUsize,
}

impl BankAccount {
    fn new(initial_balance: i32) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
            transaction_count: AtomicUsize::new(0),
        }
    }
    
    fn transfer(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
        self.transaction_count.fetch_add(1, Ordering::Relaxed);
    }
    
    fn get_balance(&self) -> i32 {
        *self.balance.lock().unwrap()
    }
    
    fn get_transaction_count(&self) -> usize {
        self.transaction_count.load(Ordering::Relaxed)
    }
}

fn test_thread_safety_invariants() {
    println!("\n--- Thread Safety Invariants Test ---");
    
    let account = Arc::new(BankAccount::new(1000));
    let mut handles = vec![];
    
    // Multiple threads making transfers
    for i in 0..10 {
        let account = Arc::clone(&account);
        let handle = thread::spawn(move || {
            for j in 0..100 {
                let amount = if (i + j) % 2 == 0 { 1 } else { -1 };
                account.transfer(amount);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_balance = account.get_balance();
    let transaction_count = account.get_transaction_count();
    
    println!("Final balance: {} (should be 1000)", final_balance);
    println!("Total transactions: {} (should be 1000)", transaction_count);
    
    // Invariant checks
    assert_eq!(final_balance, 1000, "Balance invariant violated!");
    assert_eq!(transaction_count, 1000, "Transaction count invariant violated!");
    println!("All invariants maintained!");
}

// TODO 9: Stress testing with random delays
fn chaos_testing() {
    println!("\n--- Chaos Testing (Random Delays) ---");
    
    let counter = Arc::new(ConcurrentCounter::new());
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for j in 0..20 {
                // Random delays to create different interleavings
                let delay_ms = (i * 7 + j * 3) % 10;
                thread::sleep(Duration::from_millis(delay_ms as u64));
                
                counter.increment();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_value = counter.get();
    println!("Final counter value: {} (expected: 100)", final_value);
    
    if final_value == 100 {
        println!("Chaos test PASSED");
    } else {
        println!("Chaos test FAILED - race condition detected!");
    }
}

fn main() {
    println!("=== Testing Concurrency in Rust ===\n");
    
    stress_test_counter();
    deterministic_test_with_barriers();
    detect_race_conditions();
    test_for_deadlocks();
    property_based_testing();
    performance_testing();
    memory_ordering_test();
    test_thread_safety_invariants();
    chaos_testing();
    
    println!("\n=== Concurrency testing examples completed! ===");
    println!("\nTesting Tips:");
    println!("1. Run tests multiple times to catch intermittent race conditions");
    println!("2. Use tools like 'cargo test --release' for realistic performance tests");
    println!("3. Consider using loom (https://crates.io/crates/loom) for model checking");
    println!("4. Use thread sanitizer: RUSTFLAGS=\"-Z sanitizer=thread\" cargo run");
    println!("5. Add randomness and delays to expose timing-dependent bugs");
}
