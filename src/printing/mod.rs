mod basic;
mod complicated_display;
mod debug;
mod formatting;

pub fn printing_examples() {
    println!("\nSTART: Basic printing");
    basic::basic_printing();
    println!("FINISH: Basic printing\n\n");

    println!("START: Basic debug");
    debug::basic_debug();
    println!("FINISH: Basic debug\n\n");

    println!("START: Slightly more complex display");
    complicated_display::more_complex_display();
    println!("FINISH: Slightly more complex display\n\n");

    println!("START: formatting");
    formatting::some_formatting();
    println!("FINISH: formatting\n\n");
}
