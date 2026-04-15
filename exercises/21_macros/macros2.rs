// TODO: Fix the compiler error by moving the whole definition of this macro.
// Macros must appear before they are invoked. This does not apply to most
// Rust code, but it does for macros
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
