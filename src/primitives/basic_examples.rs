pub fn basic_examples() {
    let logical: bool = true;

    let a_float: f32 = 3.4;
    let an_integer = 5i64;

    let default_float = 5.6; // f64
    let default_integer = 14; // i32

    let mut inferred_type = 12; // defaults to i32 if no further changes
    inferred_type = 4294967296i64; // but now compiler knows it is actually an i64

    // inferred_type = true // this doesn't work, can't change type of variable

    let inferred_type = true; // can overwrite with shadowing though
}
