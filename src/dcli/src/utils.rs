pub fn print_standard(out:&str, print:bool) {
    if !print {
        return;
    }

    println!("{}", out);
}

pub fn print_error(out:&str, print:bool) {
    if !print {
        return;
    }

    eprintln!("{}", out);
}