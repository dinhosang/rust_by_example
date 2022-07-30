#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct - apparently useful for Generics
struct Unit;

// A tuple struct - a named tuple essentially
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right:
            Point {
                x: bottom_right_x,
                y: bottom_right_y,
            },
    } = rectangle;

    let length = top_left_x * bottom_right_y;
    let width = top_left_y * bottom_right_x;
    return length * width;
}

fn square(point: Point, length: f32) -> Rectangle {
    let Point {
        x: bottom_left_x,
        y: bottom_left_y,
    } = point;

    return Rectangle {
        top_left: Point {
            x: bottom_left_x,
            y: bottom_left_y + length,
        },
        bottom_right: Point {
            x: bottom_left_x + length,
            y: bottom_left_y,
        },
    };
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

pub fn structs_example() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
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
        x: left_edge,
        y: top_edge,
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
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area of rectangle is: {:>.2?}", rect_area(_rectangle));
    println!(
        "square made from point and length: {:?}",
        square(Point { x: 3.0, y: 5.0 }, 5.0)
    )
}
