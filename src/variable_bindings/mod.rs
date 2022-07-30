mod basic;
mod freezing;
mod mutability;
mod scope_shadowing;

pub fn variable_examples() {
    basic::basic_examples();
    mutability::mutability_examples();
    scope_shadowing::scope_shadowing_examples();
    freezing::freezing_examples();
}
