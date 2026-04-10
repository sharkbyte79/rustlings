// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz"); // Move string2 to make it live as long as string1
    let result; // result is a non-initialized string reference
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
