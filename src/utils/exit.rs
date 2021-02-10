use exitcode::ExitCode;

pub fn exit_with_code(code: ExitCode, message: Option<&str>) -> ! {
    if let Some(message) = message {
        println!("\n{}", message);
    }

    #[cfg(not(test))]
    std::process::exit(code);
    #[cfg(test)]
    panic!(code);
}
