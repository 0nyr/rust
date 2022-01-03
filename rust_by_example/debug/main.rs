// https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}



// Define a structure where the fields are nameable for comparison.
use std::fmt; // Import `fmt`

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}



fn main() {
    /*
    fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
    Rust also provides "pretty printing" with {:#?}.
    */

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);

    // {1:?} here 1 is index of print, :? means debug print
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // Pretty print
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter); // normal print
    println!("{:#?}", peter); // pretty print

    // custom print
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("{:?}", point);
}