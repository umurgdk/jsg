extern crate serde_json;

use std::env;
use std::env::Args;
use std::error::Error;
use std::iter::Skip;
use std::process;

const USAGE: &'static str = "USAGE: jsg name=jsg num=25 num_as_str=\"25\" arr=\"$(jsg --arr 1 string 3)\" obj=\"$(jsg field=value)\" bool=false another=null";

fn main() {
    let first_arg = env::args().skip(1).next();
    let res = match first_arg.as_ref().map(String::as_ref) {
        Some("--help") => {
            println!("{}", USAGE);
            process::exit(1);
        }
        Some("--arr") => do_array(env::args().skip(2)),
        _ => do_object(env::args().skip(1)),
    };

    match res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}

fn quote_val(val: String) -> String {
    let first_char = val.chars().next().unwrap();
    let is_num = first_char.is_numeric();
    let is_obj = first_char == '{';
    let is_arr = first_char == '[';
    let is_bool = val == "true" || val == "false";
    let is_null = val == "null";

    if is_num || is_obj || is_arr || is_bool || is_null {
        val
    } else {
        format!("\"{}\"", val)
    }
}

fn do_array(args: Skip<Args>) -> Result<(), Box<Error>> {
    let mut arr: Vec<serde_json::Value> = vec![];
    for arg in args {
        let val = serde_json::from_str(&quote_val(arg))?;
        arr.push(val);
    }

    println!("{}", serde_json::to_string(&arr)?);
    Ok(())
}

fn do_object(args: Skip<Args>) -> Result<(), Box<Error>> {
    let mut obj = serde_json::Value::Object(serde_json::Map::new());
    for arg in args {
        let mut parts = arg.splitn(2, '=').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("each argument needs to be a key=value pair, got: {}", arg).into());
        }

        let val_part = parts.pop().unwrap().to_string();
        if val_part.len() == 0 {
            return Err(format!("field value can't be empty").into());
        }

        let val = serde_json::from_str(&quote_val(val_part)).map_err(|err| {
            Into::<Box<Error>>::into(format!("parse json failed for \"{}\": {}", parts[1], err))
        })?;
        obj[parts[0]] = val;
    }

    println!("{}", serde_json::to_string(&obj).unwrap());
    Ok(())
}
