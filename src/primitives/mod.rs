mod arrays;
mod basic_examples;
mod literals_and_operators;
mod tuples;

pub fn primitives() {
    // scalar primitives: integer, float, character, string, boolean, unit -> ()
    // compound primities: array, tuple
    // NOTE: a char is 4 bytes each
    basic_examples::basic_examples();
    literals_and_operators::lit_and_op_examples();
    tuples::tuple_examples();
    arrays::arrays_example();
}
