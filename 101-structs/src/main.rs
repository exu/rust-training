#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!(
        "Rect area {}",
        rect_area(Rectangle {
            bottom_right: Point { x: 1.0, y: 2.0 },
            top_left: Point { x: 2.0, y: 5.0 }
        })
    );

    println!("Square {:?}", square(Point { x: 2.0, y: 5.0 }, 10.0));
}

// Activity 1
// Add a function rect_area which calculates the area of a rectangle (try using nested destructuring).
fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        bottom_right: Point { x: x1, y: y1 },
        top_left: Point { x: x2, y: y2 },
    } = rectangle;

    return (x2 - x1) * (y2 - y1);
}

// Activity 2
// Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle
// with its lower left corner on the point, and a width and height corresponding to the f32.
fn square(point: Point, size: f32) -> Rectangle {
    return Rectangle {
        bottom_right: Point {
            x: point.x + size,
            y: point.y,
        },
        top_left: Point {
            x: point.x,
            y: point.y + size,
        },
    };
}
