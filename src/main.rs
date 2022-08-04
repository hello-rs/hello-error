use std::fs::File;

use anyhow::{Context, Result};
use error::{CustomError, NetworkError};

mod error;

fn get_number_gt_200() -> Result<(), CustomError> {
    Err(CustomError::NumberGt200Error(300))
}
fn get_network_error() -> Result<(), CustomError> {
    Err(CustomError::Network(NetworkError::TimeOut))
}

fn get_io_error(path: String) -> Result<(), CustomError> {
    match File::open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(CustomError::from(e)),
    }
}

fn main() -> Result<()> {
    // match get_number_gt_200() {
    //     Ok(_) => (),
    //     Err(e) => println!("{}", e),
    // }
    match get_network_error() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    let path = "a.txt".to_string();
    get_io_error(path.clone()).with_context(|| format!("找不到此文件,{}", path))?;
    Ok(())
}
