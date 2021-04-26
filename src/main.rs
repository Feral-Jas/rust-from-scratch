mod data_type;
mod variable_mutability;
fn main() {
    variable_mutability::mutable();
    variable_mutability::constant();
    variable_mutability::shadow();
    data_type::integers();
    data_type::floats();
}
