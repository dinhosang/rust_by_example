use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        // default write! returns a result, using ? means it won't return a result unless it errors
        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

pub fn more_complex_display() {
    let v = List(vec![7, 8, 9]);
    println!("{}", v); // will display: [0: 7, 1: 8, 2: 9]
}
