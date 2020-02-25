#![allow(unreachable_code)]
#![allow(unused_labels)]

fn main() {
    if_else();
    loop_example();
    nesting_and_labels();
    returning_from_loops();
    while_example();
    for_range();
    for_and_iterators();
    match_example();
}

fn if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        n / 2
        // TODO ^ Try suppressing this expression with a semicolon.
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}

fn loop_example() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

fn nesting_and_labels() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn returning_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn print_fizz_buzz(n: i32) {
    if n % 15 == 0 {
        print!("fizzbuzz ");
    } else if n % 3 == 0 {
        print!("fizz ");
    } else if n % 5 == 0 {
        print!("buzz ");
    } else {
        print!("{} ", n);
    }
}

fn while_example() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        print_fizz_buzz(n);
        // Increment counter
        n += 1;
    }
}

fn for_range() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        print_fizz_buzz(n);
    }

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        print_fizz_buzz(n);
    }
}

fn for_and_iterators() {
    // iter - This borrows each element of the collection through each iteration.
    // Thus leaving the collection untouched and available for reuse after the loop.

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter - This consumes the collection so that on each iteration the exact
    // data is provided. Once the collection has been consumed it is no longer available
    // for reuse as it has been 'moved' within the loop.

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // iter_mut - This mutably borrows each element of the collection,
    // allowing for the collection to be modified in place.

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn match_example() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
