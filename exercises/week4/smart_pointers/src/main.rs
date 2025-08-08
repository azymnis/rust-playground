// Week 4: Smart Pointers
// Learn about Box, Rc, RefCell, and other smart pointer types

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

// TODO 1: Box<T> for heap allocation
fn box_examples() {
    println!("--- Box<T> Examples ---");
    
    // Basic box usage
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Box enables recursive types
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list: {:?}", list);
    
    // Box for large data on heap
    let large_array = Box::new([0; 1000]);
    println!("Large array on heap, first element: {}", large_array[0]);
}

// TODO 2: Implementing our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn mybox_examples() {
    println!("\n--- MyBox Examples ---");
    
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref coercion
    
    println!("x = {}, *y = {}", x, *y);
    
    // Deref coercion with functions
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> &String -> &str
}

// TODO 3: Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_examples() {
    println!("\n--- Drop Trait Examples ---");
    
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    
    println!("CustomSmartPointers created.");
    
    // Manual drop
    let early_drop = CustomSmartPointer {
        data: String::from("early drop"),
    };
    
    drop(early_drop);
    println!("CustomSmartPointer dropped early");
    
    // c and d will be dropped automatically at end of function
}

// TODO 4: Rc<T> for reference counting
fn rc_examples() {
    println!("\n--- Rc<T> Examples ---");
    
    #[derive(Debug)]
    enum List2 {
        Cons(i32, Rc<List2>),
        Nil,
    }
    
    use List2::{Cons, Nil};
    
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
        println!("c = {:?}", c);
    }
    
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}

// TODO 5: RefCell<T> for interior mutability
fn refcell_examples() {
    println!("\n--- RefCell<T> Examples ---");
    
    // Basic RefCell usage
    let data = RefCell::new(5);
    
    println!("Original value: {:?}", data.borrow());
    
    *data.borrow_mut() += 10;
    
    println!("After mutation: {:?}", data.borrow());
    
    // RefCell with trait objects
    trait Messenger {
        fn send(&self, msg: &str);
    }
    
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    
    let mock_messenger = MockMessenger::new();
    mock_messenger.send("Hello");
    mock_messenger.send("World");
    
    println!("Messages sent: {:?}", mock_messenger.sent_messages.borrow());
}

// TODO 6: Combining Rc and RefCell
fn rc_refcell_examples() {
    println!("\n--- Rc<RefCell<T>> Examples ---");
    
    #[derive(Debug)]
    enum List3 {
        Cons(Rc<RefCell<i32>>, Rc<List3>),
        Nil,
    }
    
    use List3::{Cons, Nil};
    
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;
    
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// TODO 7: Weak references to prevent cycles
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_references() {
    println!("\n--- Weak References ---");
    
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", 
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("branch strong = {}, weak = {}", 
                 Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", 
                 Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", 
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

// TODO 8: Custom smart pointer with multiple ownership
#[derive(Debug)]
struct SharedData<T> {
    data: T,
    ref_count: RefCell<usize>,
}

struct SharedPtr<T> {
    inner: Rc<SharedData<T>>,
}

impl<T> SharedPtr<T> {
    fn new(data: T) -> Self {
        SharedPtr {
            inner: Rc::new(SharedData {
                data,
                ref_count: RefCell::new(1),
            }),
        }
    }
    
    fn get_ref_count(&self) -> usize {
        *self.inner.ref_count.borrow()
    }
}

impl<T> Clone for SharedPtr<T> {
    fn clone(&self) -> Self {
        *self.inner.ref_count.borrow_mut() += 1;
        SharedPtr {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> Deref for SharedPtr<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.inner.data
    }
}

impl<T> Drop for SharedPtr<T> {
    fn drop(&mut self) {
        *self.inner.ref_count.borrow_mut() -= 1;
        let count = *self.inner.ref_count.borrow();
        if count == 0 {
            println!("Last SharedPtr dropped!");
        }
    }
}

// TODO 9: Cell<T> for Copy types
use std::cell::Cell;

fn cell_examples() {
    println!("\n--- Cell<T> Examples ---");
    
    let data = Cell::new(5);
    println!("Original: {}", data.get());
    
    data.set(10);
    println!("After set: {}", data.get());
    
    // Cell with struct
    #[derive(Clone, Copy, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point_cell = Cell::new(Point { x: 0, y: 0 });
    println!("Original point: {:?}", point_cell.get());
    
    point_cell.set(Point { x: 5, y: 10 });
    println!("Updated point: {:?}", point_cell.get());
}

// TODO 10: Comparison of different pointer types
fn pointer_comparison() {
    println!("\n--- Pointer Type Comparison ---");
    
    println!("Box<T>:");
    println!("- Single ownership");
    println!("- Heap allocation");
    println!("- Move semantics");
    
    println!("\nRc<T>:");
    println!("- Multiple ownership");
    println!("- Reference counting");
    println!("- Immutable data");
    
    println!("\nRefCell<T>:");
    println!("- Interior mutability");
    println!("- Runtime borrow checking");
    println!("- Single-threaded only");
    
    println!("\nCell<T>:");
    println!("- Interior mutability for Copy types");
    println!("- No borrowing, only get/set");
    println!("- Single-threaded only");
    
    // Usage examples
    let boxed = Box::new(42);
    let shared = Rc::new(42);
    let mutable = RefCell::new(42);
    let cell = Cell::new(42);
    
    println!("\nValues: Box: {}, Rc: {}, RefCell: {}, Cell: {}", 
             boxed, shared, *mutable.borrow(), cell.get());
}

fn main() {
    println!("=== Smart Pointers in Rust ===\n");
    
    box_examples();
    mybox_examples();
    drop_examples();
    rc_examples();
    refcell_examples();
    rc_refcell_examples();
    weak_references();
    
    // TODO 11: Custom smart pointer usage
    println!("\n--- Custom Smart Pointer ---");
    let ptr1 = SharedPtr::new(String::from("Hello"));
    println!("ptr1 ref count: {}", ptr1.get_ref_count());
    
    {
        let ptr2 = ptr1.clone();
        println!("After cloning: {}", ptr1.get_ref_count());
        println!("ptr2 value: {}", *ptr2);
    }
    
    println!("After ptr2 dropped: {}", ptr1.get_ref_count());
    
    cell_examples();
    pointer_comparison();
    
    println!("\n=== All smart pointer examples completed! ===");
}
