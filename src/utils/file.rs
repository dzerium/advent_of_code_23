use std::fs;
use crate::utils::errors::*;

pub fn read_file (file: &str) -> Result<String, AppError>{
    println!("opening {}", file);
    let fp = fs::read_to_string(file);

    match fp {
        Ok(contents) => {
            Ok(contents)
        },
        Err(_e) => {
            Err(AppError::new(
                ErrorCode::InvalidInput,
                AreaCode::Utils,
                String::from("Unable to read file"),
            ))
        }
    }
}
