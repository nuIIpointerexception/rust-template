/// Main binary function, responsible for:
/// * Calling command line parsing logic.
/// * Setting up configuration.
/// * Calling the "Hello World!" function.
/// * Handling the error if the above returns an error.

pub fn main() {
    println!("Hello World! 🌎");
}

#[test]
fn simple_math() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn newspeak_math() {
    assert_eq!(2 + 2, 5);
}
