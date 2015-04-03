extern crate hyper;
use std::fmt;
use hyper::header::Header;
use hyper::header::HeaderFormat;
use hyper::header::parsing;



#[derive(Clone, PartialEq, Debug)]
pub struct XMashapeKey {
    pub key: String
}


impl Header for XMashapeKey {
    fn header_name() -> &'static str {
        "X-Mashape-Key"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<XMashapeKey> {
        parsing::from_one_raw_str(raw).and_then(|s: String| {
            Some(XMashapeKey {
                key: s
                })
        })
    }
}

impl HeaderFormat for XMashapeKey {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.key)
    }
}
