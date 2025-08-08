// Week 5: Channels - Message Passing Concurrency
// Learn about mpsc (multiple producer, single consumer) channels

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// TODO 1: Basic channel usage
fn basic_channel() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("Hello from thread!");
        tx.send(val).unwrap();
        // val is moved, can't use it here anymore
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// TODO 2: Multiple messages
fn multiple_messages() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive messages until channel is closed
    for received in rx {
        println!("Got: {}", received);
    }
}

// TODO 3: Multiple producers
fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
            String::from("one"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("thread two"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}

// TODO 4: Synchronous channels (bounded)
fn sync_channel() {
    // Channel with buffer size of 2
    let (tx, rx) = mpsc::sync_channel(2);
    
    let tx_clone = tx.clone();
    thread::spawn(move || {
        for i in 0..5 {
            println!("Sending: {}", i);
            tx_clone.send(i).unwrap();
            println!("Sent: {}", i);
        }
    });
    
    thread::sleep(Duration::from_millis(1000));
    
    for received in rx {
        println!("Received: {}", received);
        thread::sleep(Duration::from_millis(400));
    }
}

// TODO 5: Try operations (non-blocking)
fn try_operations() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("Delayed message").unwrap();
    });
    
    // Try to receive without blocking
    match rx.try_recv() {
        Ok(msg) => println!("Got immediate message: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("No message available yet"),
        Err(mpsc::TryRecvError::Disconnected) => println!("Channel disconnected"),
    }
    
    println!("Waiting for message...");
    thread::sleep(Duration::from_millis(600));
    
    match rx.try_recv() {
        Ok(msg) => println!("Got delayed message: {}", msg),
        Err(e) => println!("Error: {:?}", e),
    }
}

// TODO 6: Timeout operations
fn timeout_operations() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send("Late message").unwrap();
    });
    
    match rx.recv_timeout(Duration::from_millis(500)) {
        Ok(msg) => println!("Got message within timeout: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("Timed out waiting for message"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("Channel disconnected"),
    }
}

// TODO 7: Worker pool pattern
fn worker_pool() {
    enum Job {
        Work(String),
        Terminate,
    }
    
    let (tx, rx) = mpsc::channel();
    let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));
    
    let mut workers = vec![];
    
    // Create worker threads
    for id in 0..3 {
        let rx = rx.clone();
        let worker = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv().unwrap();
                match job {
                    Job::Work(data) => {
                        println!("Worker {} processing: {}", id, data);
                        thread::sleep(Duration::from_millis(100));
                    }
                    Job::Terminate => {
                        println!("Worker {} terminating", id);
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    
    // Send work to pool
    for i in 0..10 {
        tx.send(Job::Work(format!("Task {}", i))).unwrap();
    }
    
    // Terminate workers
    for _ in 0..3 {
        tx.send(Job::Terminate).unwrap();
    }
    
    // Wait for workers to finish
    for worker in workers {
        worker.join().unwrap();
    }
}

// TODO 8: Bidirectional communication
fn bidirectional_communication() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    thread::spawn(move || {
        // Receive request
        let request = rx1.recv().unwrap();
        println!("Worker received request: {}", request);
        
        // Process and send response
        let response = format!("Processed: {}", request);
        tx2.send(response).unwrap();
    });
    
    // Send request
    tx1.send("Do some work".to_string()).unwrap();
    
    // Receive response
    let response = rx2.recv().unwrap();
    println!("Main received response: {}", response);
}

// TODO 9: Fan-out pattern
fn fan_out_pattern() {
    let (tx, rx) = mpsc::channel();
    
    // Multiple receivers (fan-out)
    let receivers = vec![rx];
    
    // Producer
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });
    
    // Process messages
    for rx in receivers {
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                println!("Processor received: {}", msg);
            }
        });
    }
    
    thread::sleep(Duration::from_millis(100));
}

// TODO 10: Pipeline pattern
fn pipeline_pattern() {
    // Stage 1: Generate numbers
    let (tx1, rx1) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..=5 {
            tx1.send(i).unwrap();
        }
    });
    
    // Stage 2: Square numbers
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        for num in rx1 {
            let squared = num * num;
            tx2.send(squared).unwrap();
        }
    });
    
    // Stage 3: Print results
    thread::spawn(move || {
        for result in rx2 {
            println!("Final result: {}", result);
        }
    });
    
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    println!("=== Channels in Rust ===\n");
    
    println!("--- Basic Channel ---");
    basic_channel();
    
    println!("\n--- Multiple Messages ---");
    multiple_messages();
    
    println!("\n--- Multiple Producers ---");
    multiple_producers();
    
    println!("\n--- Synchronous Channel ---");
    sync_channel();
    
    println!("\n--- Try Operations ---");
    try_operations();
    
    println!("\n--- Timeout Operations ---");
    timeout_operations();
    
    println!("\n--- Worker Pool ---");
    worker_pool();
    
    println!("\n--- Bidirectional Communication ---");
    bidirectional_communication();
    
    println!("\n--- Fan-out Pattern ---");
    fan_out_pattern();
    
    println!("\n--- Pipeline Pattern ---");
    pipeline_pattern();
    
    println!("\n=== All channel examples completed! ===");
}
