pub mod control_flow;
pub mod data_type;
pub mod function;
pub mod variable_mutability;

pub fn concepts_explained() {
    variable_mutability::mutable();
    variable_mutability::constant();
    variable_mutability::shadow();
    data_type::integers();
    data_type::floats();
    data_type::numeric_operation();
    data_type::boolean();
    data_type::character();
    data_type::tuple();
    data_type::array();
    function::expression();
    function::fn_get_return();
    control_flow::if_else();
    control_flow::if_in_let();
    control_flow::loop_fn();
    control_flow::while_fn();
    control_flow::for_fn();
}
