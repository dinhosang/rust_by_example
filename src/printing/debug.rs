use std::fmt;

// Below doesn't implement Debug so it can't be printed
#[allow(dead_code)] // this prvents the warning about dead code blocking the run
struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct ToShowOffPrettyPrinting<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct ToShowOffImplementingDisplay {
    age: u8,
    shoe_size: u8,
}

impl fmt::Display for ToShowOffImplementingDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ age: {}, shoe size: {} }}", self.age, self.shoe_size)
    }
}

pub fn basic_debug() {
    println!("this will use the debug version of {:?}", "the variable"); // this causes the passed in string variables to be string wrapped

    println!(
        "this {0:?} use the debug version of {1:?} along with indexing, for example {2:?}",
        "will", "the variable", 9
    );

    println!("The structure will now print: {:?}", DebugPrintable(3));

    println!(
        "The deeper version will also print: {0:?}",
        Deep(DebugPrintable(5))
    );

    let name = "David";
    let person = ToShowOffPrettyPrinting { name, age: 28 };

    // escape braces by wrapping them in braces!
    println!("use {{:#?}} to use pretty printing for debug");
    println!(
        "The is what the person looks like without pretty printing: {:?}",
        person
    );
    println!(
        "The is what the person looks like WITH pretty printing: {:#?}",
        person
    );

    let anotherPerson = ToShowOffImplementingDisplay {
        age: person.age,
        shoe_size: 10,
    };

    println!("using display instead of debug: {}", anotherPerson)
}
