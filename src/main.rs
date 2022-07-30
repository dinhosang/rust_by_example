// An attribute to hide warnings for unused code.
#![allow(dead_code)]

mod control_flow;
mod conversion;
mod custom_types;
mod expressions;
mod functions;
mod primitives;
mod printing;
mod types;
mod variable_bindings;

fn main() {
    println!("\n\n-----\n\n");
    printing::printing_examples();
    println!("\n\n-----\n\n");
    primitives::primitives();
    println!("\n\n-----\n\n");
    custom_types::custom_types_examples();
    println!("\n\n-----\n\n");
    variable_bindings::variable_examples();
    println!("\n\n-----\n\n");
    types::types_examples();
    println!("\n\n-----\n\n");
    conversion::conversion_examples();
    println!("\n\n-----\n\n");
    expressions::expressions_examples();
    println!("\n\n-----\n\n");
    control_flow::control_flow_examples();
    println!("\n\n-----\n\n");
    functions::functions_examples();
    println!("\n\n-----\n\n");
}
