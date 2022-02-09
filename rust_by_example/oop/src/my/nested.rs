// this function is public
pub fn function() {
    println!("called `my::nested::function()`");
}

// this function is private
#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}

 // this macro is public
#[macro_export]
macro_rules! my_macro {
    ($text:expr) => {
        format!("called `my::nested::my_macro!({})`", $text)
    }
}


