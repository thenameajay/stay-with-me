use std::io;

pub fn gimmi(inp_description: &str) ->String {
    let mut inp_var = String::new();
    println!("{inp_description}: ");
    io::stdin().read_line(&mut inp_var).expect("Failed to get {inp_description}!");
    inp_var.trim().to_string()
}