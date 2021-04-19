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
    pub tags:HashMap<String,String>,
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
        static ref RE: Regex = Regex::new(r"^(?P<path>[^;[:space:]]+)(?P<tags>(?:;(?P<tag_name>[^;=\^!]+)=(?P<tag_value>[^;~][^;[:space:]]+))*)(?: (?P<value>\d+))(?: (?P<time>\d+)){0,1}").unwrap();
        static ref TAG: Regex = Regex::new(r";(?P<tag_name>[^;=\^!]+)=(?P<tag_value>[^;~][^;]+)").unwrap();
    }
    match RE.captures(input) {
        Some(cap) => {
            println!("{:?}",cap);
            let path = cap.name("path").unwrap().as_str();
            let value = cap.name("value").unwrap().as_str().parse::<f64>().unwrap();
            let tags = cap.name("tags").unwrap();
            let mut tag_map = HashMap::new();
            println!("Tags: {:?}",tags);
            for tag in TAG.captures_iter(tags.as_str()) {
                tag_map.insert(tag[1].to_string(),tag[2].to_string());
            }

            let time = cap.name("time");
            let timestamp = match time {
                Some(value) => {
                    Some(value.as_str().parse::<u32>().unwrap())
                }
                _ => None
            };
            Ok(GraphiteDataPoint{path,tags:tag_map,value,timestamp})
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
    fn parse_tagged_path() {
        let output = super::parse_graphite("tagged.path;tag=value 1234");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "tagged.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.tags.len(),1);
                assert_eq!(valid.tags["tag"],"value");
            },
            _ => { panic!() },
        }
    }

    #[test]
    fn parse_multi_tagged_path() {
        let output = super::parse_graphite("multi.tagged.path;tag=value;second=tag 1234");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "multi.tagged.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.tags.len(),2);
            },
            _ => { panic!() },
        }
    }

    #[test]
    fn parse_tagged_path_with_ts() {
        let output = super::parse_graphite("simple.path;tag=value 1234 4567");
        match output {
            Ok(valid) => {
                assert_eq!(valid.path, "simple.path");
                assert_eq!(valid.value, 1234.0);
                assert_eq!(valid.tags.len(),1);
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
