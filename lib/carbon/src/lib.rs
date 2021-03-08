use regex::Regex;
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;
// The graphite format was determined from:
// https://graphite.readthedocs.io/en/latest/tags.html
// Path SHOULD match ASCII characters. While technically support for UTF-8 it's not been thoroughly
// tested.
pub struct GraphiteDataPoint<'a> {
    pub path:&'a str,
    pub tags:HashMap<&'a str,&'a str>,
    pub value:f64,
    pub timestamp:Option<u32>,
}

impl<'a> GraphiteDataPoint<'a> {
    pub fn from(input:&str) -> Result<GraphiteDataPoint, &'static str> {
        parse_graphite(input)
    }
}

fn parse_graphite(input:&str) -> Result<GraphiteDataPoint, &'static str> {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^(?P<path>[^;[:space:]]+)(?:;(?P<tag_key>[^;!\^=[:space:]]]+)=(?P<tag_value>[^;~[:space:]][^;[:space:]]*))*(?: (?P<value>\d+))(?: (?P<time>\d+)){0,1}").unwrap();
    }
    match RE.captures(input) {
        Some(cap) => {
            println!("{:?}",cap);
            let path = cap.name("path").unwrap().as_str();
            let value = cap.name("value").unwrap().as_str().parse::<f64>().unwrap();
            let time = cap.name("time");
            let ts = match time {
                Some(value) => {
                    Some(value.as_str().parse::<u32>().unwrap())
                }
                _ => None
            };
            Ok(GraphiteDataPoint{path:path,tags:HashMap::new(),value:value,timestamp:ts})
        }
        _ => Err("Input line doesn't match"),

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse_simple_path() {
        let output = super::parse_graphite("simple.path 1234\n");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "simple.path");
                assert_eq!(valid.value, 1234.0);
            },
            _ => { panic!() },
        }
    }

    #[test]
    fn parse_simple_path_with_ts() {
        let output = super::parse_graphite("simple.path 1234 4567");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "simple.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.timestamp, Some(4567));
            },
            _ => { panic!() },
        }
    }

    #[test]
    #[ignore]
    fn parse_tagged_path() {
        let output = super::parse_graphite("simple.path;tag=value 1234");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "simple.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.tags.len(),1);
                assert_eq!(valid.tags["tag"],"value");
                assert_eq!(valid.timestamp, Some(4567));
            },
            _ => { panic!() },
        }
    }

    #[test]
    #[ignore]
    fn parse_multi_tagged_path() {
        let output = super::parse_graphite("simple.path;tag=value;second=tag 1234");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "simple.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.tags.len(),2);
                assert_eq!(valid.timestamp, Some(4567));
            },
            _ => { panic!() },
        }
    }

    #[test]
    #[ignore]
    fn parse_tagged_path_with_ts() {
        let output = super::parse_graphite("simple.path;tag=value 1234 4567");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "simple.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.timestamp, Some(4567));
            },
            _ => { panic!() },
        }
    }

    #[test]
    #[ignore]
    fn parse_bad() {
        let output = super::parse_graphite("simple;.path 1234 4567");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "simple.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.timestamp, Some(4567));
            },
            _ => { panic!() },
        }
    }
}
