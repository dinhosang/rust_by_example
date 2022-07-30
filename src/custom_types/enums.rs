// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => {
            println!("page unloaded")
        }
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

#[derive(Debug)]
enum ReallyLongNamedEnumThatRepresentsSomeThings {
    Add,
    Subtract,
}

// can use type functionality to set an alias, i.e. rename, enums to something more useful/easier for where it is being used, or if enum name is too generic
type ShorterName = ReallyLongNamedEnumThatRepresentsSomeThings;

// can also do similar things is in an impl block, using a Self alias, this is the hard-coded alias in rust in such a scenario
impl ReallyLongNamedEnumThatRepresentsSomeThings {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - 7,
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn enum_examples() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let ok = ShorterName::Add;
    println!("the process is: {:?}", ok);
    println!(
        "the result is: {:?}",
        ReallyLongNamedEnumThatRepresentsSomeThings::Add.run(3, 4)
    );
    println!(
        "the result is: {:?}",
        ReallyLongNamedEnumThatRepresentsSomeThings::Subtract.run(3, 4)
    );

    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::custom_types::enums::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::custom_types::enums::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
