macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    // Calls to macros should end with !
    my_macro!();
}
