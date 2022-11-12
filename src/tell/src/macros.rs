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

#[macro_export]
macro_rules! t {

    //($lvl:expr, $($arg:tt)+) => (log!(target: __log_module_path!(), $lvl, $($arg)+));
    ($lvl:expr, $($arg:tt)+)  => {
        tell::Tell::print($lvl, &(format_args!($($arg)+)).to_string());
    };
}

//TODO: this works. copy to other functions
//move files into lib
#[macro_export]
macro_rules! update {
    () => (tell::t!(tell::TellLevel::Update, ""));
    ($($arg:tt)+) => (tell::t!(tell::TellLevel::Update, $($arg)+));
}

#[macro_export]
macro_rules! progress {
    () => (tell::t!(tell::TellLevel::Progress, ""));
    ($($arg:tt)+) => (tell::t!(tell::TellLevel::Progress, $($arg)+));
}

#[macro_export]
macro_rules! error {
    () => (tell::t!(tell::TellLevel::Error, ""));
    ($($arg:tt)+) => (tell::t!(tell::TellLevel::Error, $($arg)+));
}

#[macro_export]
macro_rules! verbose {
    () => (tell::t!(tell::TellLevel::Verbose, ""));
    ($($arg:tt)+) => (tell::t!(tell::TellLevel::Verbose, $($arg)+));
}
