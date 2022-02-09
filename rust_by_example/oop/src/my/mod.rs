// mod.rs is main file of the module my

// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible; // import inaccessible.rs
#[macro_use] pub mod nested; // import nested.rs with macros

pub fn function() {
    // double nested macro
    // use my_macro!("hello") to call my_macro!
    println!(
        "{}", // WARN: Don't forget the string formatting
        my_macro!("called `my::function()`")
    );
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}