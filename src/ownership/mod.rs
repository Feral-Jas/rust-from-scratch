//? Ownership is the unique feature
//* Enables Rust to make memory safety guarantees without needing a garbage collector
//* memory is managed through a system of ownership with a set of rules that the compiler checks at compile time
//* None of the ownership features slow down your program while it’s running.
//* Keeping track of what parts of code are using what data on the heap,
//* minimizing the amount of duplicate data on the heap,
//* and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
pub mod variable_scope;

pub fn concepts_explained() {
    println!("{}", "------------scope explained");
    variable_scope::scope();
}
