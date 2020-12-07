use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum DateTimeFormat {
    RFC2822,
    RFC3339,
}

impl FromStr for DateTimeFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "rfc2822" => Ok(DateTimeFormat::RFC2822),
            "rfc3339" => Ok(DateTimeFormat::RFC3339),
            
            _ => Err("Unknown DateTimeFormat type"),
        }
    }
}

impl fmt::Display for DateTimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            DateTimeFormat::RFC2822 => " RFC 2822",
            DateTimeFormat::RFC3339 => "RFC 3339",
        };

        write!(f, "{}", out)
    }
}