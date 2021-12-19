use std::io::{stdin, stdout, Write};

fn validate(dia: i32, min: i32, max: i32) -> bool {
    (min..=max).contains(&dia)
}

pub fn get_number_from_user(min: i32, max: i32, error_message: &str) -> Option<i32> {
    loop {
        let mut user_input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut user_input).expect(error_message);
        if user_input.trim() == "q" {
            break None;
        }
        if let Ok(number_from_input) = user_input.trim().parse::<i32>() {
            if validate(number_from_input, min, max) {
                break Some(number_from_input);
            }
        }
        println!("{}", error_message);
    }
}
