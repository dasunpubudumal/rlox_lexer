//
pub fn error(line: u16, message: String) {
    report(line, message);
}

fn report(line: u16, message: String) {
    println!("Error in line [[{}]]: caused by: [[{}]]", line, message);
}
