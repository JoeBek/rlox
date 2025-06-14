
pub fn error(line:u32, msg:&str) {
    report(line, "", &msg);
}

fn report(line:u32, loc:&str, msg:&str) {

    eprintln!("[line {line} ] error {loc}: {msg}");
}

