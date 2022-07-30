pub fn mutability_examples() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! cannot modify immutable variables (variable are immutable by default)
    // _immutable_binding += 1;
}
