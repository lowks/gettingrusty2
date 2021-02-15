use std::panic;
// use std::assert_err;

fn main() {
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

        println!("I am in the loop - {}" ,z);
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
        println!("Animal of {} is {}", index,mal)
    }

    /* Enum */

    enum Direction {
        Up,
        Down
    }

    let player_direction:Direction = Direction::Down;
    
    match player_direction {
        Direction::Up => println!("going up"),
        Direction::Down => println!("going down")
    }

    /* constant number */

    const MASIH:&str = "Masih";
    const HOHO:i32 = 23;

    println!("{}", MASIH);
    println!("The number is {}", HOHO);

    /* tuple */

    let tupl = (1, 2, "three", 4, (5,6));

    println!("The third element in the tuple is {}", tupl.2);
    println!("First inside the tuple in the tuple is {}", (tupl.4).1);
    print_until(100);

    // shadowing

    let blocker = 10;

    {
        let blocker = 20;
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
    let s4 = & s3;
    println!("String s4: {}, String s3 {}", s4, s3);

    // mutable borrow
    let mut s5 = String::from("chello");
    let s6 = &mut s5;
    s6.replace("ch", "h");
    println!("s6 {}", s6);
    println!("s5 {}", s5);
    // s6.push('!'); < === results in error.

    let mut s7 = String::from("gogo ");
    let mut s8 = String::from(" uncle roger");
    append_string(&mut s7, &mut s8);
    // println!("s9 {}", s9);
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
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

fn append_string(t: &mut String, u: &mut String) {
    t.push_str(u);
    println!("{}", t);
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
    return num % 2 == 0;
}