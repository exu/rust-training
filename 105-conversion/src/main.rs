use std::fmt;

use std::convert::From;

use std::convert::TryFrom;
use std::convert::TryInto;

fn main() {
    from_into();
    try_from_try_into();
    convert_to_string();
    parsing_a_string();
}

#[derive(Debug)]
struct Number {
    value: i32,
    desc: f32,
}

// The From trait allows for a type to define how to create itself from another type,
// hence providing a very simple mechanism for converting between several types.
// There are numerous implementations of this trait within the standard library for
// conversion of primitive and common types.
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {
            value: item,
            desc: 0.0,
        }
    }
}

fn from_into() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;

    // The Into trait is simply the reciprocal of the From trait. That is,
    // if you have implemented the From trait for your type you get the `Into`
    // implementation  *for free*.
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn try_from_try_into() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

//Converting to String
// To convert any type to a String is as simple as implementing the ToString
// trait for the type. Rather than doing so directly, you should implement the
// fmt::Display trait which automagically provides ToString and also allows
// printing the type as discussed in the section on print!.

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius <r={}>", self.radius)
    }
}

fn convert_to_string() {
    let circle = Circle { radius: 6 };
    println!("{}", circle);
}

// One of the more common types to convert a string into is a number.
// The idiomatic approach to this is to use the parse function and either
// to arrange for type inference
// or to specify the type to parse using the 'turbofish' syntax.
// Both alternatives are shown in the following example.
// This will convert the string into the type specified so long as the FromStr
// trait is implemented for that type. This is implemented for numerous types
// within the standard library. To obtain this functionality on a user defined type
// simply implement the FromStr trait for that type.

fn parsing_a_string() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let f: f32 = "4.534534".parse().unwrap();
    println!("Float parse {:?}", f);
}
