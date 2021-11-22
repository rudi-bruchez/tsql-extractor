use std::env;
use std::fs;
use regex::Regex;
use std::borrow::Cow;
use sqlformat::format;
// use reqwest::Client;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result = extract_tsql(contents);
    // format_with_sqlformat (result);

    println!("QUERY:\n{}", result);
}

// extract sql from sp_executesql, and injecting variable values
fn extract_tsql (content: String) -> String {
    let re = Regex::new(r"(@\w*)=(\w*)").unwrap();

    // assert!(re.is_match(&contents));
    // println!("{}", re.is_match(&contents));

    let mut result = Cow::from(&content);
    
    for cap in re.captures_iter(&content) {
        // println!("Param: {} value: {} ", &cap[1], &cap[2]);
        result = result.replace(&cap[1], &cap[2]).into();
    }
    result = result.replace("exec sp_executesql N'", "\n").into();
    // result = sqlformat::format(result);

    // end of the execute_sql
    let re = Regex::new(r"',N'.*$").unwrap();
    // result = re.replace_all(result, "");

    return result.replace("''", "'").into();

}

// fn format_with_sqlformat (content: String) -> String {
//     let resp = reqwest::blocking::post("https://sqlformat.org/api/v1/format")
//         .body("sql={}", content)
//         .send()
    
//     println!("{}", resp);

// }