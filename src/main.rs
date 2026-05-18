#![deny(clippy::too_many_arguments)]
#![deny(clippy::to_string_in_format_args)]


fn main() {
    let id = uuid::Uuid::new_v4();
    println!("Generated UUID is {}", id);
    test_lint(1, 1, 1, 1, 1, 1, 1, 1, 1);
}

fn test_lint(_arg_1: u8, _arg_2: u8, _arg_3: u8, _arg_4: u8, _arg_5: u8, _arg_6: u8, _arg_7: u8, _arg_8: u8, _arg_9: u8) {}

#[allow(unused)]
fn test_string() {
    let x = "Hello from rust";
    println!("{}", x)
}