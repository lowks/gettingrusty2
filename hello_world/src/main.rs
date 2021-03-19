mod fizzbuzz;
mod matches;
mod nprimes;
mod raindrops;
mod create_vec;
mod split_string;
mod create_series;
mod closures;
pub use self::fizzbuzz::*;
pub use self::matches::*;
pub use self::nprimes::*;
pub use self::raindrops::*;
pub use self::create_vec::*;
pub use self::split_string::*;
pub use self::create_series::*;
pub use self::closures::*;
use std::collections::HashMap;
use std::env;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn return_red(&self) -> u8 {
        self.red
    }

    fn return_color(&self, color: &str) -> u8 {
        let match_color = color;
        match match_color {
            "red" => self.red,
            "green" => self.green,
            "blue" => self.blue,
            _ => 0,
        }
    }
}

macro_rules! add_as {
    // using a ty token type for macthing datatypes passed to maccro
    ($a:expr,$b:expr,$typ:ty) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! yo {
    ($name:expr, $another_name: expr) => {
        println!("Yo {}  {} !!", $name, $another_name)
    };
}

fn main() {
    // create_vec

    let vec2 = create_vector();

    for e in vec2 {
        println!("{}", e)
    };

    // macros
    println!("Call add_as {}", add_as!(0, 2, u8));
    yo!("Terence", "Piggy");
    // print out positional arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    /* this
     are
    comments */
    println!("Hello, world!");
    let mut x = 45;
    println!("The value of x is {}", x);
    x = 60;
    println!("The value of x is {}", x);
    let y: i64 = 100;
    println!("The value of y is {}", y);

    /* this is the if-else */
    let m = 45;

    if m < 30 {
        println!("The number is less than 30")
    } else {
        println!("It's bigger than 30")
    }

    /* loop */

    let mut z = 1;

    loop {
        z += 1;

        if z == 7 {
            continue;
        }

        if z > 20 {
            break;
        }

        println!("I am in the loop - {}", z);
    }

    /* while loop */

    let mut e = 1;

    while e <= 10 {
        println!("The number is - {}", e);

        e += 1;
    }

    /* for loop */

    for i in 1..100 {
        println!("For loop number - {}", i);
    }

    let animals = vec!["Muiz", "Duck", "Face"];

    for (index, mal) in animals.iter().enumerate() {
        println!("Animal of {} is {}", index, mal)
    }

    /* Enum */

    enum Direction {
        Up,
        Down,
    }

    let player_direction: Direction = Direction::Down;

    match player_direction {
        Direction::Up => println!("going up"),
        Direction::Down => println!("going down"),
    }

    /* constant number */

    const MASIH: &str = "Masih";
    const HOHO: i32 = 23;

    println!("{}", MASIH);
    println!("The number is {}", HOHO);

    /* tuple */

    let tupl = (1, 2, "three", 4, (5, 6));

    println!("The third element in the tuple is {}", tupl.2);
    println!("First inside the tuple in the tuple is {}", (tupl.4).1);
    print_until(100);

    // shadowing

    // let blocker = 10;

    {
        let _blocker = 20;
    }

    let blocker = "a string now";

    println!("block {}, block2 {}", blocker, blocker);

    let blocker = true;

    println!("block {}, block2 {}", blocker, blocker);

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {} and s2 = {}", s1, s2);

    // reference
    let s3 = String::from("hello");
    let s4 = &s3;
    println!("String s4: {}, String s3 {}", s4, s3);

    // mutable borrow
    let mut s5 = String::from("chello");
    let s6 = &mut s5;
    let s7 = s6.replace("ch", "h");
    println!("s6 {}", s6);
    println!("s5 {}", s5);
    println!("s7 {}", s7);
    // s6.push('!'); < === results in error.

    let s7 = String::from("gogo ");
    let s8 = String::from(" uncle roger");
    println!("Appended {}", append_string(s7, s8));
    // println!("s9 {}", s9);

    let mut x = 10;
    // this mut the other also needs to be a mut. Likewise if not mut
    let low = &mut x;
    // cannot just increment. Need to dereference
    *low += 1;
    println!("low is {}", low);

    // struct
    let mut bg = Color {
        red: 255,
        green: 78,
        blue: 15,
    };

    bg.blue = 45;
    // let pick_color = "red";

    println!("This is Color {}", bg.return_red());
    println!(
        "This is the code - {} for the color {}",
        bg.return_color("red"),
        "red"
    );

    println!("The struct {} {} {}", bg.red, bg.green, bg.blue);

    let mut print_line = String::from("Hello World");
    let integer = 10;
    print_line.push_str(" hoho");
    // integer += 1;
    println!("{}", integer);

    let array = create_array(10, 20);

    for i in 0..array.len() {
        println!("Array {}", array[i])
    }

    println!("Array is {:?}", array);
    println!("Length of the array is {}", array.len());

    // fizzbuzz

    for i in 1..100 {
        println!("{}", determine_fizz_buzz(i));
    }

    let add_5 = |var2: i32| -> Vec<i32> { array.iter().map(|x| x + var2).collect() };

    //closure that captures environment

    println!("Closures result {:?}", add_5(10));

    // match

    let country_code = 45;

    println!(
        "Country Code {} with country {}",
        country_code,
        match_country(country_code)
    );
}

#[cfg(test)]
// use std::collections::HashMap;

mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
    }

    #[test]

    fn test_append_string() {
        assert_eq!(
            append_string(String::from("hello"), String::from(" world")),
            "hello world"
        )
    }

    #[test]

    fn test_create_array() {
        assert_eq!(create_array(5, 2), [5, 5])
    }

    #[test]

    fn test_increment_value() {
        let mut x = 2;
        assert_eq!(*increment_value(&mut x), 3)
    }

    #[test]

    fn test_color_struct() {
        let mut color_map = HashMap::new();
        let color_code = 255;
        color_map.insert("red", color_code);
        color_map.insert("blue", color_code);
        color_map.insert("green", color_code);

        let bg = Color {
            red: 255,
            green: 255,
            blue: 255,
        };
        for (key, val) in color_map.iter() {
            assert_eq!(bg.return_color(key), *val)
        }
    }

    #[test]

    fn test_fizz_buzz() {
        assert_eq!(determine_fizz_buzz(4), format!("{}", 4));
        assert_eq!(determine_fizz_buzz(3), "fizz");
        assert_eq!(determine_fizz_buzz(5), "buzz");
        assert_eq!(determine_fizz_buzz(15), "fizz buzz");
    }

    // #[test]

    // fn test_panic() {
    //     let result = panic::catch_unwind(|| {
    //         is_even("stupid");
    //     });
    //     assert!(result.is_err());
    // }

    // #[test]
    // // #[ignore]
    // // #[should_panic]
    // #![feature(try_blocks)]
    // fn test_any_panic() {
    //     // let r = is_even(-1);
    //     let result: Result<(), Error> = try {
    //         is_even(-1)?;
    //     };

    //     if let Err(_err) = result {
    //         println!("Failed to perform necessary steps");
    //     };
    //     // let mut result = match r {
    //     //     true => println!("hoho"),
    //     //     false => println!("hoho2"),
    //     //     Err(e) => e,
    //     // };
    //     // assert_eq!(is_even(-1), error[E0600]);
    //     // assert_eq!(result.is_err());
    //     // dang .... how to assert errors in Rust ??
    // }

    // code blocks
}

// Why this works - https://stackoverflow.com/questions/39917173/why-is-it-not-possible-to-concatenate-two-strings-in-rust-without-taking-a-refer

fn append_string(t: String, u: String) -> String {
    let it = String::from(t);
    let iu = String::from(u);
    it + &iu
}

fn print_until(num: u32) {
    for i in 1..num {
        if is_even(i) {
            println!("The number is ... {}", i);
        } else {
            println!("hoho");
        }
    }
}

fn is_even(num: u32) -> bool {
    num % 2 == 0
}

fn create_array(elements: i32, len: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(elements);
    }
    vec
}

fn increment_value(x: &mut i32) -> &mut i32 {
    *x += 1;
    x
}
