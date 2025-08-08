// Week 5: Shared State Concurrency
// Learn about Mutex, Arc, and RwLock for safe shared state

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// TODO 1: Basic Mutex usage
fn basic_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter result: {}", *counter.lock().unwrap());
}

// TODO 2: Handling poisoned mutexes
fn poisoned_mutex() {
    let data = Arc::new(Mutex::new(0));
    let data_clone = Arc::clone(&data);
    
    let handle = thread::spawn(move || {
        let mut num = data_clone.lock().unwrap();
        *num += 1;
        panic!("Thread panicked while holding lock!");
    });
    
    // Thread panics, mutex becomes poisoned
    let _ = handle.join();
    
    // Try to access poisoned mutex
    match data.lock() {
        Ok(guard) => println!("Got lock: {}", *guard),
        Err(poisoned) => {
            println!("Mutex was poisoned!");
            let guard = poisoned.into_inner();
            println!("Recovered data: {}", *guard);
        }
    }
}

// TODO 3: Multiple readers with RwLock
fn rwlock_example() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Multiple reader threads
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data.read().unwrap();
            println!("Reader {} sees: {:?}", i, *reader);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // Writer thread
    let data = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut writer = data.write().unwrap();
        writer.push(6);
        println!("Writer added element 6");
    });
    handles.push(writer_handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final data: {:?}", *data.read().unwrap());
}

// TODO 4: Shared data structure
struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            value: Mutex::new(0),
        }
    }
    
    fn increment(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }
    
    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

fn shared_data_structure() {
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.increment();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Shared counter result: {}", counter.get());
}

// TODO 5: Bank account simulation
#[derive(Debug)]
struct BankAccount {
    balance: Mutex<f64>,
    account_number: String,
}

impl BankAccount {
    fn new(account_number: String, initial_balance: f64) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
            account_number,
        }
    }
    
    fn deposit(&self, amount: f64) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
        println!("Account {}: Deposited ${:.2}, new balance: ${:.2}", 
                self.account_number, amount, *balance);
    }
    
    fn withdraw(&self, amount: f64) -> bool {
        let mut balance = self.balance.lock().unwrap();
        if *balance >= amount {
            *balance -= amount;
            println!("Account {}: Withdrew ${:.2}, new balance: ${:.2}", 
                    self.account_number, amount, *balance);
            true
        } else {
            println!("Account {}: Insufficient funds for ${:.2} withdrawal", 
                    self.account_number, amount);
            false
        }
    }
    
    fn get_balance(&self) -> f64 {
        *self.balance.lock().unwrap()
    }
}

fn bank_account_simulation() {
    let account = Arc::new(BankAccount::new("12345".to_string(), 1000.0));
    let mut handles = vec![];
    
    // Multiple threads accessing the same account
    for i in 0..5 {
        let account = Arc::clone(&account);
        let handle = thread::spawn(move || {
            account.deposit(50.0);
            account.withdraw(30.0);
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final balance: ${:.2}", account.get_balance());
}

// TODO 6: Deadlock demonstration (commented out to prevent actual deadlock)
fn deadlock_example() {
    let resource1 = Arc::new(Mutex::new(1));
    let resource2 = Arc::new(Mutex::new(2));
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    // Thread 1: locks resource1 then resource2
    let handle1 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap();
        println!("Thread 1: locked resource 1");
        thread::sleep(Duration::from_millis(100));
        
        // This would cause deadlock if thread 2 runs simultaneously
        // let _guard2 = r2.lock().unwrap();
        // println!("Thread 1: locked resource 2");
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    // Thread 2: locks resource2 then resource1
    let handle2 = thread::spawn(move || {
        let _guard2 = r2.lock().unwrap();
        println!("Thread 2: locked resource 2");
        thread::sleep(Duration::from_millis(100));
        
        // This would cause deadlock
        // let _guard1 = r1.lock().unwrap();
        // println!("Thread 2: locked resource 1");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("Deadlock example completed (deadlock prevented by commenting out conflicting locks)");
}

// TODO 7: Cache implementation with RwLock
struct Cache {
    data: RwLock<HashMap<String, String>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: RwLock::new(HashMap::new()),
        }
    }
    
    fn get(&self, key: &str) -> Option<String> {
        let reader = self.data.read().unwrap();
        reader.get(key).cloned()
    }
    
    fn insert(&self, key: String, value: String) {
        let mut writer = self.data.write().unwrap();
        writer.insert(key, value);
    }
    
    fn len(&self) -> usize {
        let reader = self.data.read().unwrap();
        reader.len()
    }
}

fn cache_example() {
    let cache = Arc::new(Cache::new());
    let mut handles = vec![];
    
    // Writer threads
    for i in 0..3 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            cache.insert(format!("key{}", i), format!("value{}", i));
            println!("Writer {} inserted key{}", i, i);
        });
        handles.push(handle);
    }
    
    // Reader threads
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            if let Some(value) = cache.get(&format!("key{}", i % 3)) {
                println!("Reader {} found: {}", i, value);
            } else {
                println!("Reader {} found nothing for key{}", i, i % 3);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Cache size: {}", cache.len());
}

// TODO 8: Try lock example
fn try_lock_example() {
    let data = Arc::new(Mutex::new(0));
    let data_clone = Arc::clone(&data);
    
    let handle = thread::spawn(move || {
        let _guard = data_clone.lock().unwrap();
        println!("Thread 1: acquired lock");
        thread::sleep(Duration::from_millis(200));
        println!("Thread 1: releasing lock");
    });
    
    thread::sleep(Duration::from_millis(50));
    
    // Try to acquire lock without blocking
    match data.try_lock() {
        Ok(mut guard) => {
            println!("Main thread: acquired lock");
            *guard += 1;
        }
        Err(_) => {
            println!("Main thread: could not acquire lock (busy)");
        }
    }
    
    handle.join().unwrap();
    
    // Now we can acquire it
    match data.try_lock() {
        Ok(guard) => {
            println!("Main thread: acquired lock after thread finished, value: {}", *guard);
        }
        Err(_) => {
            println!("Main thread: still could not acquire lock");
        }
    }
}

fn main() {
    println!("=== Shared State Concurrency ===\n");
    
    println!("--- Basic Mutex ---");
    basic_mutex();
    
    println!("\n--- Poisoned Mutex ---");
    poisoned_mutex();
    
    println!("\n--- RwLock Example ---");
    rwlock_example();
    
    println!("\n--- Shared Data Structure ---");
    shared_data_structure();
    
    println!("\n--- Bank Account Simulation ---");
    bank_account_simulation();
    
    println!("\n--- Deadlock Example ---");
    deadlock_example();
    
    println!("\n--- Cache Example ---");
    cache_example();
    
    println!("\n--- Try Lock Example ---");
    try_lock_example();
    
    println!("\n=== All shared state examples completed! ===");
}
