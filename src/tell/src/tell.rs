/*
* Copyright 2022 Mike Chambers
* https://github.com/mikechambers/dcli
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of
* this software and associated documentation files (the "Software"), to deal in
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do
* so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use lazy_static::lazy_static;
use std::cmp;
use std::fmt;
use std::sync::RwLock;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum TellLevel {
    Silent = 0,
    Error,
    Update,   //general update on status
    Progress, //progress update
    Verbose,
}

impl fmt::Display for TellLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            TellLevel::Silent => "Silent",
            TellLevel::Error => "Error",
            TellLevel::Update => "Update",
            TellLevel::Progress => "Progress",
            TellLevel::Verbose => "Verbose",
        };

        write!(f, "{}", out)
    }
}

impl PartialOrd for TellLevel {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let s = *self as usize;
        let o = *other as usize;
        s.partial_cmp(&o)
    }
}

lazy_static! {

    #[allow(non_upper_case_globals)]
    static ref TL: RwLock<TellLevel> = RwLock::new(TellLevel::Update);
}

pub struct Tell;

impl Tell {
    pub fn is_active(level: TellLevel) -> bool {
        let t = TL.write().unwrap();

        level <= *t
    }

    pub fn set_level(level: TellLevel) {
        let mut t = TL.write().unwrap();
        *t = level;
    }

    pub fn init(level: TellLevel) {
        Tell::set_level(level);
    }

    pub fn print(level: TellLevel, msg: &str) {
        if !Tell::is_active(level) || level == TellLevel::Silent {
            return;
        }

        match level {
            TellLevel::Error => {
                eprintln!("{}", msg);
            }
            _ => {
                println!("{}", msg);
            }
        }
    }
}

#[macro_export]
macro_rules! t {

    //($lvl:expr, $($arg:tt)+) => (log!(target: __log_module_path!(), $lvl, $($arg)+));
    ($lvl:expr, $($arg:tt)+)  => {
        tell::tell::Tell::print($lvl, &(format_args!($($arg)+)).to_string());
    };
}

//TODO: this works. copy to other functions
#[macro_export]
macro_rules! foo {
    () => (tell::t!(tell::tell::TellLevel::Update, ""));
    ($($arg:tt)+) => (tell::t!(tell::tell::TellLevel::Update, $($arg)+));
}

//update seems to work. turn this into a macro
#[macro_export]
macro_rules! update {

    () => {{
        tell::tell::Tell::print(tell::tell::TellLevel::Update, "");
    }};

    ($a:expr) => {{

        tell::tell::Tell::print(tell::tell::TellLevel::Update, $a);
    }};

    ($($arg:tt)+) => {{
        tell::tell::Tell::print(tell::tell::TellLevel::Update, &(format_args!($($arg)+)).to_string());
    }}

}

#[macro_export]
macro_rules! progress {

    () => {
        tell::tell::Tell::print(tell::tell::TellLevel::Progress, "\n");
    };

    ($a:expr) => {

        tell::tell::Tell::print(tell::tell::TellLevel::Progress, $a);
    };

    ($($arg:tt)*) => {{
        tell::tell::Tell::print(tell::tell::TellLevel::Progress, &(format_args!($($arg)*)).to_string());
    }}
}

#[macro_export]
macro_rules! error {


    () => {
        tell::tell::Tell::print(tell::tell::TellLevel::Error, "\n");
    };

    ($a:expr) => {

        tell::tell::Tell::print(tell::tell::TellLevel::Error, $a);
    };

    ($($arg:tt)*) => {{
        tell::tell::Tell::print(tell::tell::TellLevel::Error, &(format_args!($($arg)*)).to_string());
    }}

    /*
    ($a:expr) => {
        tell::tell::Tell::print(tell::tell::TellLevel::Error, $a);
    };
    */
}

#[macro_export]
macro_rules! verbose {

    () => {
        tell::tell::Tell::print(tell::tell::TellLevel::Verbose, "\n")
    };

    ($a:expr) => {

        tell::tell::Tell::print(tell::tell::TellLevel::Verbose, $a)
    };

    ($($arg:tt)*) => {{
        tell::tell::Tell::print(tell::tell::TellLevel::Verbose, &(format_args!($($arg)*)).to_string());
    }}

    /*

($($arg:tt)+) => {tell::tell::Tell::print(tell::tell::TellLevel::Verbose, format_args!($($arg)+).to_string())};

    ($a:expr) => {
        tell::tell::Tell::print(tell::tell::TellLevel::Verbose, $a.to_string());
    }
*/
}

/*
//this takes same input as format!
#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
        tell::tell::Tell::print(tell::tell::TellLevel::Error, format_args!($($args)*).to_string());
    };
}
*/
