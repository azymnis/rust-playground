// Week 5: Thread Basics
// Learn the fundamentals of threading in Rust

use std::thread;
use std::time::Duration;

// TODO 1: Basic thread spawning
fn spawn_simple_thread() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..=3 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for the spawned thread to finish
    handle.join().unwrap();
}

// TODO 2: Move closure with threads
fn move_closure_example() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        // v is moved into the thread, can't use it in main thread anymore
    });
    
    handle.join().unwrap();
}

// TODO 3: Returning values from threads
fn thread_return_value() -> i32 {
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        sum // This value is returned from the thread
    });
    
    let result = handle.join().unwrap();
    println!("Sum from thread: {}", result);
    result
}

// TODO 4: Multiple threads
fn multiple_threads() {
    let mut handles = vec![];
    
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("Thread {} with ID {:?} is running", i, thread_id);
            thread::sleep(Duration::from_millis(50));
            i * i // Return the square
        });
        handles.push(handle);
    }
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    println!("Results: {:?}", results);
}

// TODO 5: Thread builder with custom names and stack sizes
fn thread_builder_example() {
    let builder = thread::Builder::new()
        .name("worker-thread".to_string())
        .stack_size(4 * 1024 * 1024); // 4MB stack
    
    let handler = builder.spawn(|| {
        let thread = thread::current();
        println!("Thread name: {:?}", thread.name());
        println!("Thread ID: {:?}", thread.id());
        
        // Simulate some work
        let mut data = vec![0u8; 1_000_000];
        for i in 0..data.len() {
            data[i] = (i % 256) as u8;
        }
        data.len()
    }).unwrap();
    
    let result = handler.join().unwrap();
    println!("Thread processed {} bytes", result);
}

// TODO 6: Panic handling in threads
fn panic_handling() {
    println!("\n--- Panic Handling ---");
    
    // Thread that will panic
    let handle = thread::spawn(|| {
        println!("Thread about to panic!");
        panic!("Oops! Thread panicked!");
    });
    
    // Main thread continues even if spawned thread panics
    match handle.join() {
        Ok(_) => println!("Thread completed successfully"),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
    
    println!("Main thread continues after handling panic");
}

// TODO 7: Thread local storage
thread_local! {
    static THREAD_ID: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
}

fn thread_local_example() {
    let mut handles = vec![];
    
    for i in 0..5 {
        let handle = thread::spawn(move || {
            THREAD_ID.with(|id| {
                *id.borrow_mut() = i;
            });
            
            // Simulate some work
            thread::sleep(Duration::from_millis(50));
            
            THREAD_ID.with(|id| {
                println!("Thread {} has thread-local ID: {}", i, *id.borrow());
            });
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// TODO 8: Scoped threads (requires crossbeam crate in real scenarios)
fn scoped_thread_simulation() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // In real code, you'd use crossbeam::scope for this
    // This is a simulation showing the concept
    {
        let data_ref = &data;
        let handle = thread::spawn(move || {
            println!("Reading data in thread: {:?}", data_ref);
        });
        handle.join().unwrap();
    }
    
    // Can still use data here
    data.push(6);
    println!("Data after thread: {:?}", data);
}

// TODO 9: Thread parking and unparking
fn parking_example() {
    let parked_thread = thread::spawn(|| {
        println!("Thread will park itself");
        thread::park();
        println!("Thread was unparked!");
    });
    
    thread::sleep(Duration::from_millis(100));
    println!("Main thread will unpark the parked thread");
    parked_thread.thread().unpark();
    
    parked_thread.join().unwrap();
}

// TODO 10: Available parallelism
fn check_parallelism() {
    match thread::available_parallelism() {
        Ok(n) => println!("This system can run {} threads in parallel", n),
        Err(e) => println!("Failed to get parallelism info: {}", e),
    }
}

fn main() {
    println!("=== Thread Basics in Rust ===\n");
    
    println!("--- Simple Thread Spawn ---");
    spawn_simple_thread();
    
    println!("\n--- Move Closure ---");
    move_closure_example();
    
    println!("\n--- Thread Return Value ---");
    thread_return_value();
    
    println!("\n--- Multiple Threads ---");
    multiple_threads();
    
    println!("\n--- Thread Builder ---");
    thread_builder_example();
    
    panic_handling();
    
    println!("\n--- Thread Local Storage ---");
    thread_local_example();
    
    println!("\n--- Scoped Thread Simulation ---");
    scoped_thread_simulation();
    
    println!("\n--- Thread Parking ---");
    parking_example();
    
    println!("\n--- System Parallelism ---");
    check_parallelism();
    
    println!("\n=== All examples completed! ===");
}
