pub fn basic_printing() {
    println!("Hello, world!");

    println!("{} days", 31);

    println!("Hi Alice, this is Bob. Bob this is Alice");

    println!(
        "Alice jumped over the gate"
    );

    println!("There are {:b} types of people", 2);

    println!("{:>5}", 2);

    println!("{:0>5}", 2);

    println!("{:1>widthToUseOk$}", 2, widthToUseOk = 10);
}
