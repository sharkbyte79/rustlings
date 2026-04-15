// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    // Must apply macro_export annotation to make it available
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
