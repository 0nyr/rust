// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;
mod client;

fn function() {
    println!("called `function() fn from main.rs`");
}

fn main() {

    // module my containing functions
    my::function();
    function();
    my::indirect_access();
    my::nested::function();

    // module Client containing type Client
    // module::Type::function()
    let client0 = client::Client::new(
        123456, "Gallé".to_string(), 
        "romain.galle@insa-lyon.fr".to_string(), 
        "123456789lolilol".to_string()
    );
    println!("{}", client0);
}