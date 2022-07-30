mod for_loops;
mod if_else;
mod if_let;
mod loops;
mod match_basics;
mod match_binding;
mod match_destructuring;
mod match_guards;
mod while_let;
mod while_loop;

pub fn control_flow_examples() {
    if_else::if_else_examples();
    loops::loops_examples();
    while_loop::while_examples();
    for_loops::for_loop_examples();
    match_basics::match_examples();
    match_destructuring::destructuring_examples();
    match_guards::match_guards_examples();
    match_binding::match_binding_examples();
    if_let::if_let_examples();
    while_let::while_let_examples();
}
