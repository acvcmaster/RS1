use std::{error::Error, process};


pub fn handle_result<T: Default, E: Error>(result: Result<T, E>, message: Option<&'static str>) -> T {
    match result {
        Ok(val) => val,
        Err(err) => {
            log_error(message, err);
            T::default()
        }
    }
}

pub fn handle_critical_result<T: Default, E: Error>(result: Result<T, E>, message: Option<&'static str>) -> T {
    match result {
        Ok(val) => val,
        Err(err) => {
            log_error(message, err);
            eprintln!("A critial error has been encountered. The application will exit.");
            process::exit(-1);
        }
    }
}

pub fn log_error<E: Error>(message: Option<&'static str>, err: E) {
    match message {
        None => eprintln!("{}", err),
        Some(msg) => eprintln!("{} {}", msg, err)
    };
}