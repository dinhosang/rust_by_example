mod from_into;
mod parse_string;
mod to_string;
mod try_from_try_into;

/*
    Primitive types can be converted to each other through casting.

    Rust addresses conversion between custom types(i.e., struct and enum) by the use of traits.
    The generic conversions will use the From and Into traits.
    However there are more specific ones for the more common cases,
        in particular when converting to and from Strings.
*/

pub fn conversion_examples() {
    from_into::from_into_examples();
    try_from_try_into::try_from_try_into_examples();
    to_string::to_string_examples();
    parse_string::parse_string_examples();
}
