use flate2::write::GzEncoder;
use flate2::Compression;
use regex::Regex;
// use std::fs::File;
use std::io::prelude::*;
// use std::io::BufReader;

// fn main() -> std::io::Result<()> {
//     let test_files = 4;

//     for i in 1..=test_files {
//         let target_num: String;
//         match i {
//             _ if i < 10 => target_num = "0".to_owned() + &i.to_owned().to_string(),
//             _ => target_num = i.to_owned().to_string(),
//         }
//         println!(
//             "================== Test: {}.mws ==================",
//             target_num
//         );

//         let file = format!("./test/{}.mws", target_num);
//         let f = File::open(file)?;
//         let reader = BufReader::new(f);

//         for line in reader.lines() {
//             let line = line?;
//             parse_line(&line);
//         }
//     }

//     Ok(())
// }


//? Split a string using regex. Keep the matched string.
fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(r) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

pub fn parse_line(line: &str) -> String {
    let split_re = Regex::new(r"([ ,:;]+)").expect("Invalid regex");
    let tokens = split_keep(&split_re, line)
        .into_iter()
        .map(|n| n.trim())
        .filter(|n| !n.is_empty())
        .collect::<Vec<_>>();
    let mut tokenized_line: Vec<String> = vec![];
    let mut minimized_line: Vec<String> = vec![];
    for token in tokens {
        let token = tokenizer(token);
        println!("{} = \"{}\"", token.token_type, token.value);
        if token.token_type == "unknown" {
            println!("================== FAILED! ==================");
            println!("Unknown token: {}", token.value);
            println!("================== FAILED! ==================");
        } else {
            tokenized_line.push(token.token_type);
            minimized_line.push(token.value);
        }
    }

    return minimized_line.join(" ");

}

struct Token {
    token_type: String,
    value: String,
}

fn tokenizer(token: &str) -> Token {
    let token_type;

    //? Regex:
    let operators_re = Regex::new(r"=|\+|-|\\").unwrap();
    let keywords_re = Regex::new(r"let|pub").unwrap();
    let identifier_re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    let punctuator_re = Regex::new(r"\(|\)|\{|\}|;|:").unwrap();
    // literals
    let string_re = Regex::new(r#""[^"]*""#).unwrap();
    let number_re = Regex::new(r"^\d+$").unwrap();
    let bool_re = Regex::new(r"true|false").unwrap();
    let null_re = Regex::new(r"null").unwrap();
    let float_re = Regex::new(r"^\d+\.\d+$").unwrap();
    // types
    let float_type_re = Regex::new(r"float").unwrap();
    let int_type_re = Regex::new(r"int").unwrap();
    let string_type_re = Regex::new(r"string").unwrap();
    let bool_type_re = Regex::new(r"bool").unwrap();
    let null_type_re = Regex::new(r"null").unwrap();
    let object_type_re = Regex::new(r"object").unwrap();
    let array_type_re = Regex::new(r"array").unwrap();
    //whitespace
    let whitespace_re = Regex::new(r"^\s+$").unwrap();
    //comment
    let comment_re = Regex::new(r"//.*").unwrap();
    let comment_block_re = Regex::new(r"/\*.*\*/").unwrap();
    //? End of regex

    match token {
        _ if keywords_re.is_match(token) => token_type = "keyword",
        _ if operators_re.is_match(token) => token_type = "operator",
        _ if punctuator_re.is_match(token) => token_type = "punctuator",

        _ if number_re.is_match(token) => token_type = "number",
        _ if float_re.is_match(token) => token_type = "float",
        _ if bool_re.is_match(token) => token_type = "bool",
        _ if null_re.is_match(token) => token_type = "null",
        _ if string_re.is_match(token) => token_type = "string",

        _ if float_type_re.is_match(token) => token_type = "type",
        _ if int_type_re.is_match(token) => token_type = "type",
        _ if string_type_re.is_match(token) => token_type = "type",
        _ if bool_type_re.is_match(token) => token_type = "type",
        _ if null_type_re.is_match(token) => token_type = "type",
        _ if object_type_re.is_match(token) => token_type = "type",
        _ if array_type_re.is_match(token) => token_type = "type",

        _ if whitespace_re.is_match(token) => token_type = "whitespace",

        _ if comment_re.is_match(token) => token_type = "comment",
        _ if comment_block_re.is_match(token) => token_type = "comment",

        _ if identifier_re.is_match(token) => token_type = "identifier",
        _ => token_type = "unknown",
    }

    return Token {
        token_type: token_type.to_string(),
        value: token.to_string(),
    };
}

fn compress_string(input: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes()).unwrap();
    encoder.finish().unwrap()
}
