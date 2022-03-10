use rand::prelude::*;
use ansi_term::{Color};
use std::sync::{Arc, Mutex};

// custom type representing thread ID
pub type ThreadId = u16;

#[derive(Debug)]
pub struct Counter {
    last_value: u32,
    // list of threads that have updated the counter
    updating_threads: Vec<ThreadId>, 
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            last_value: 0,
            updating_threads: Vec::new(),
        }
    }

    pub fn add(&mut self, new_value: u32, updating_thread: ThreadId) {
        self.last_value = new_value;
        self.updating_threads.push(updating_thread);
    }

    pub fn get_last(&self) -> u32 {
        self.last_value
    }

    pub fn get_values(&self) -> &Vec<ThreadId> {
        &self.updating_threads
    }
}

fn main() {
    println!("{}", Color::Green.paint(
        "Launching the program..."
    ));

    // create a new counter
    let mut counter = Counter {
        last_value: 0,
        updating_threads: Vec::new(),
    };

    // init the counter with some values
    counter.add(100);
    println!("{}", Color::Yellow.paint(
        format!("initial counter: {:?}", counter)
    ));

    let mut handles = vec![];

    // since we will be concurrently accessing the counter, 
    // we need to wrap it in an Arc<Mutex> so as to be
    // able to share it between threads safely
    // NOTE: Arc<Mutex<T>> is a smart pointer to a mutex
    // NOTE: Arc<T> provides reference counting via atomic 
    //  operations. It is thread safe.
    // NOTE: Rc<T> provides reference counting via normal  
    //  reads/writes. It is not thread safe.
    let data = Arc::new(Mutex::new(counter));

    for _ in 0..3 {
        // create a copy of the Arc pointer
        // for each thread, since we will be moving
        // the ownership of the copy of the Arc pointer
        // to the new thread
        let data_copy = Arc::clone(&data);

        // create a new thread
        // use closure to pass (move) the ownership 
        // of the Arc pointer to the new thread
        let handle = std::thread::spawn(move || {
            thread_operation(data_copy)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", (*data).lock().unwrap());
}

// -> std::thread::JoinHandle<()>
fn thread_operation(data: Arc<Mutex<Counter>>) {
    for _ in 0..10 {
        // keep ownership of the data
        // pass it by reference (borrow)
        update_counter(&data);
    }
}

// concurrently update the counter by solving a random problem
fn update_counter(data: &Arc<Mutex<Counter>>) {
    let mut counter = (*data).lock().unwrap();
    let mut rng = rand::thread_rng(); // get the seed from the OS
    let current_max = counter.get_last() + 10;

    // generate a random number between the last value and the current max
    loop {
        let value: u32 = rng.gen_range(0..current_max);
        if value > counter.last_value {
            counter.add(value);
            break;
        }
    }
}