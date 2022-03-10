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
        self.last_value += new_value;
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
    let counter = Counter {
        last_value: 100,
        updating_threads: Vec::new(),
    };
    println!("{}", Color::Yellow.paint(
        format!("initial counter: {:?}", counter)
    ));

    // since we will be concurrently accessing the counter, 
    // we need to wrap it in an Arc<Mutex> so as to be
    // able to share it between threads safely
    // NOTE: Arc<Mutex<T>> is a smart pointer to a mutex
    // NOTE: Arc<T> provides reference counting via atomic 
    //  operations. It is thread safe.
    // NOTE: Rc<T> provides reference counting via normal  
    //  reads/writes. It is not thread safe.
    let data = Arc::new(Mutex::new(counter));

    let mut handles = vec![];

    for i in 0..3 {
        // create a copy of the Arc pointer
        // for each thread, since we will be moving
        // the ownership of the copy of the Arc pointer
        // to the new thread
        let data_copy = Arc::clone(&data);

        // create a new thread
        // use closure to pass (move) the ownership 
        // of the Arc pointer to the new thread
        let handle = std::thread::spawn(move || {
            thread_operation(data_copy, i)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", (*data).lock().unwrap());
}

// -> std::thread::JoinHandle<()>
// concurrently update the counter by solving a random problem
fn thread_operation(data: Arc<Mutex<Counter>>, thread_id: ThreadId) {
    // keep ownership of the Arc pointer
    // so pass it by borrowing

    for _ in 0..10 {
        let mut rng = rand::thread_rng(); // get the seed from the OS
        let last_value = get_locked_last_value(&data);
        let current_max = last_value + 10;
    
        // generate a random number between the last value and the current max
        loop {
            let value: u32 = rng.gen_range(0..current_max);
            if value > last_value {
                // use lock at this scope to update the counter
                update_counter(&data, thread_id);
                break;
            }
        }
    }
}

// use lock in separate score to avoid deadlock
fn get_locked_last_value(data: &Arc<Mutex<Counter>>) -> u32 {
    let counter = (*data).lock().unwrap();
    counter.get_last()
}

fn update_counter(data: &Arc<Mutex<Counter>>, thread_id: ThreadId) {
    let mut guarder_counter = (*data).lock().unwrap();
    guarder_counter.add(10, thread_id);
}