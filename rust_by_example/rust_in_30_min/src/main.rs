// tuto link: https://fasterthanli.me/articles/a-half-hour-to-learn-rust
// `cargo build` to build the binary
// `cargo run` to run the program


fn main() {

    // VARIABLES
    // let introduces a variable binding
    let x; // declaration
    x = 42; // assignment

    // This can also be written as a single line
    // we may use `_x` eventually, but our code is a work-in-progress
    // and we just wanted to get rid of a compiler warning for now.
    let _x = 5;

    // specify type
    let _y: i32 = 12;
    // there's i8, i16, i32, i64, i128
    // also u8, u16, u32, u64, u128 for unsigned

    // this does *nothing* because 42 is a constant
    let _ = 42;

    // this calls `get_thing` but throws away its result
    fn get_thing(value: i32) -> i32 {
        return value*3;
    }
    let _ = get_thing(x);

    
    
    // TUPLES
    // tuples basically are a "fixed-length collections 
    // of values of different types".
    let pair: (char, i32) = ('a', 17);
    println!("element 1: {:?}, element 2: {:?}", pair.0, pair.1);

    // tuples can be destructured
    let (some_char, some_int) = pair;
    println!("some_char : {:?}, some_int 2: {:?}", some_char, some_int);

    // throw away the first element
    let (_, some_int_2) = pair;
    println!("some_int_2: {:?}", some_int_2);



    // BLOCKS
    // blocks are a way to group statements together
    // a block has its own score
    // a block can return a value
    let x = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z // this is the *tail* - what the whole block will evaluate to
        // NOTE: "omitting the semicolon at the end of a function" 
        // is the same as returning
    };
    println!("x after block: {:?}", x);
    


    // CONDITIONALS and RANDOM
    use rand::Rng;
    // to use a crate, use `use crate::crate_name`
    // and put its reference as a dependency in `Cargo.toml`
    //      [dependencies]
    //      rand = "0.8"

    fn feeling_lucky() -> bool {
        let random_number = rand::thread_rng().gen_range(0..2);
        if random_number == 0 {
            return true;
        } else {
            return false;
        }
    }

    // if-else
    fn fair_dice_roll() -> i32 {
        if feeling_lucky() {
            6
        } else {
            4
        }
    }
    println!("fair_dice_roll: {:?}", fair_dice_roll());

    // match
    fn fair_dice_roll_matching() -> i32 {
        match feeling_lucky() {
            true => 6,
            false => 4,
        }
    }
    println!(
        "fair_dice_roll_matching: {:?}", 
        fair_dice_roll_matching()
    );

    // match has to be exhaustive, at least one arm must match
    // NOTE: use `_` to ignore a value ("catch all" pattern)
    fn print_welcome(message_code: i32) {
        match message_code {
            1 => println!("Welcome to Rust!"),
            2 => println!("Goodbye!"),
            _ => println!("Unknown message code"),
        }
    }
    print_welcome(rand::thread_rng().gen_range(1..4));
    


    // NAMESPACES
    // :: is the scope operator
    // it allows us to access a name from a different scope
    // ex: `crate::crate_name::function_name`
    println!(
        "min of 2 random numbers: {:?}",
        std::cmp::min(
            rand::thread_rng().gen_range(0..1000),
            rand::thread_rng().gen_range(0..1000)
        )
    );

    // use directives can be used to "bring in scope" names from other namespace
    use std::cmp::max;
    println!(
        "max of 2 random numbers: {:?}",
        max(
            rand::thread_rng().gen_range(0..1000),
            rand::thread_rng().gen_range(0..1000)
        )
    );

    // A wildcard (*) lets you import every symbol from a namespace
    //use std::fmt::*;
    println!("{}", format!("---> {:010} ", 42));

    // str is a primitive type, but many non-primitive types 
    // are also in scope by default
    println!("len of string: {}", str::len("a string"));



    // STRUCTS & TRAITS
    // structs are a way to group data together
    // structs are also used to define a new type

    #[derive(Debug)]
    struct Vec2 {
        _x: f64, // 64-bit floating point, aka "double precision"
        _y: f64,
    }
    let v1 = Vec2 { _x: 1.0, _y: 3.0 };
    let v2 = Vec2 { _y: 2.0, _x: 4.0 };
    println!("v1: {:?}, v2: {:?}", v1, v2);

    // shortcut for initializing the rest of the fields from another struct
    let v3 = Vec2 {
        _x: 14.0,
        ..v2
    };
    println!("v3: {:?}", v3);
    
    let v4 = Vec2 {
        ..v1
    };
    println!("v4: {:?}", v4);

    // structs like tuples can be destructured
    let Vec2 { _x, .. } = v1; // throws away the y
    println!("destructured _x: {:?}", _x);

    // let patterns can be used as conditions in if
    #[derive(Debug)]
    struct Number {
        odd: bool,
        value: i32,
    }

    fn print_number_if(n: Number) {
        // dereference the struct n and test the field odd
        if let Number { odd: true, value } = n {
            println!("Odd number: {}", value);
        } else if let Number { odd: false, value } = n {
            println!("Even number: {}", value);
        }
    }

    let one = Number { odd: true, value: 1 };
    let two = Number { odd: false, value: 2 };
    print_number_if(one);
    print_number_if(two);

    // match arms are also patterns, just like if let
    fn print_number_match(n: Number) {
        match n {
            Number { odd: true, value } => println!("Odd number: {}", value),
            Number { odd: false, value } => println!("Even number: {}", value),
        }
    }
    let three = Number { odd: true, value: 3 };
    print_number_match(three);

    // one can declare methods on its own struct/type
    impl Number {
        fn is_strictly_positive(self) -> bool {
            self.value > 0
        }
    }
    let four = Number { odd: false, value: 4 };
    println!("positive? {}", four.is_strictly_positive());

    // traits are like interfaces in other languages
    // traits are implemented on types
    // foreign traits CANNOT be implemented on foreign traits
    trait Signed {
        fn is_strictly_negative(self) -> bool;
    }

    // impl can be used to implement traits
    impl Signed for Number {
        fn is_strictly_negative(self) -> bool {
            self.value < 0
        }
    }
    let five = Number { odd: false, value: 5 };
    println!("negative? {}", five.is_strictly_negative());

    // we can also implement traits on foreign types
    // en even on primitive types !!!
    impl Signed for i32 {
        fn is_strictly_negative(self) -> bool {
            self < 0
        }
    }
    let x: i32 = -44;
    println!("primitive type negative? {}", x.is_strictly_negative());

    // adding a foreign trait to our own struct/type
    impl std::ops::Neg for Number {
        type Output = Number;
    
        fn neg(self) -> Number {
            Number {
                value: -self.value,
                odd: self.odd,
            }        
        }
    }
    let six = Number { odd: false, value: 6 };
    let neg_six = -six;
    println!("neg_six: {:?}", neg_six);


    // MUTABILITY
    // by default, variables are immutable which means they can't be changed
    // mutable variables are declared with the keyword `mut`
    let mut n = Number {
        odd: true,
        value: 17,
    };
    n.value = 19; // reassigned, all good



    // 

}