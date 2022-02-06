use std::convert::TryFrom;
use std::str;
use std::str::{Split, Utf8Error};
use regex::Regex;
use super::method::Method;
use super::validation_result::ValidationResult;

pub struct RawRequest {
    path: String,
    query_params: Option<String>,
    method: Method,
}

impl RawRequest {
    fn get_method(raw_string: &str) -> Method {
        match &raw_string[0..3] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "OPTIONS" => Method::OPTIONS,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "CONNECT" => Method::CONNECT,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            _ => panic!("Unknown method {}", &raw_string[0..3])
        }
    }

    fn get_path(raw_string: &str) -> String {
        let path_params: &str = Self::get_path_and_params(raw_string);

        if path_params.contains("?") {
            let path = path_params
                .split("?")
                .next()
                .expect("Error getting path from path & params");
            String::from(path)
        } else {
            String::from(path_params)
        }
    }

    fn get_params(raw_string: &str) -> Option<String> {
        let path_params: &str = Self::get_path_and_params(raw_string);

        if path_params.contains("?") {
            let params = path_params
                .split("?")
                .nth(1)
                .expect("Error getting params from path & params");
            Option::Some(String::from(params))
        } else {
            Option::None
        }
    }

    fn validate(raw_string: &str) -> ValidationResult {
        // TODO - get proper regex, add further validation and investigate rubbish being send after http message
        let regex = Regex::new(
            "^[A-Z]+ /[a-zA-Z0-9%]*(\\?([a-zA-Z0-9%)]=[a-zA-Z0-9%)]&)*[a-zA-Z0-9%)]=[a-zA-Z0-9%)])? HTTP/1.1")
            .unwrap();

        if regex.is_match(raw_string) {
            ValidationResult::Pass
        } else {
            ValidationResult::Fail(vec!["Request didn't match regex".to_string()])
        }
    }

    fn get_path_and_params(raw_string: &str) -> &str {
        raw_string
            .split(" ")
            .nth(1)
            .expect("Error getting path and params")
    }
}

impl TryFrom<&[u8]> for RawRequest {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let req_str = match str::from_utf8(&value) {
            Ok(str) => str,
            Err(e) => return Result::Err(format!("{}", e))
        };

        match RawRequest::validate(req_str) {
            ValidationResult::Pass => println!("Request passed validation"),
            ValidationResult::Fail(errs) => return Result::Err(errs.iter().fold(String::new(), |mut x, x1| {
                x.push_str(x1);
                x
            })),
        };

        Result::Ok(Self {
            method: RawRequest::get_method(req_str),
            path: RawRequest::get_path(req_str),
            query_params: RawRequest::get_params(req_str),
        })
    }
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS\r\n
[BODY\r\n]
\r\n
 */