use std::fmt;
use std::mem;

fn main() {
    primitives();
    literals();
    tuples();
    arrays_and_slices();
}

// Scalar Types
// - signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// - unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// - floating point: f32, f64
// - char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// - bool either true or false
// - and the unit type (), whose only possible value is an empty tuple: ()
fn primitives() {
    // Variables can be type annotated.
    let logical: bool = true;

    println!("{:?}", logical);

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`
    println!("a_float, an_integer, default_float, default_integer");
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        a_float, an_integer, default_float, default_integer,
    );

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    println!("{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    println!("{}", mutable);
    mutable = 21;
    println!("{}", mutable);

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
    println!("{}", mutable);
}

// Integers 1, floats 1.2, characters 'a', strings "abc",
// booleans true and the unit type () can be expressed using literals.
// Integers can, alternatively, be expressed using hexadecimal,
// octal or binary notation using these prefixes respectively: 0x, 0o or 0b.
// Underscores can be inserted in numeric literals to improve readability, e.g.
// 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001.
// We need to tell the compiler the type of the literals we use.
// For now, we'll use the u32 suffix to indicate that the literal is an
// unsigned 32-bit integer, and the i32 suffix to indicate that it's a signed 32-bit integer.
// The operators available and their precedence in Rust are similar to other C-like languages.
fn literals() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

// A tuple is a collection of values of different types.
// Tuples are constructed using parentheses (),
// and each tuple itself is a value with type signature (T1, T2, ...),
// where T1, T2 are the types of its members.
// Functions can use tuples to return multiple values,
// as tuples can hold any number of values.

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(pair: Matrix) -> Matrix {
    // `let` can be used to bind the members of a tuple to variables
    return Matrix(pair.0, pair.2, pair.1, pair.3);
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn tuples() {
    // A tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Traspose:\n{}", transpose(matrix));
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn arrays_and_slices() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
}
