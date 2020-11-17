pub fn print_standard(out:String, print:bool) {
    if !print {
        return;
    }

    println!("{}", out);
}

pub fn print_error(out:String, print:bool) {
    if !print {
        return;
    }

    eprintln!("{}", out);
}