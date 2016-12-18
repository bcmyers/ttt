use std::io;

pub fn get_input() -> usize {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    match buffer.trim().parse::<usize>() {
        Ok(val) => val,
        Err(err) => {
            println!("Invalid input. Please try again.");
            get_input()
        }
    }
}
