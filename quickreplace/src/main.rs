use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                    "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}",
                        "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to write to file'{}': {:?}",
                        "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another",
                "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    // args[0]은 실행하는 프로그램의 이름이기 때문에 skip(1)로 한 개 무시
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.",
                  "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}

fn replace(target: &str, replacement: &str, text: &str)
    -> Result<String, regex::Error>
{
    let regex = Regex::new(target)?;
    // replace_all은 매칭되는 부분이 발견되면 해당 내용을 주어진 텍스트로 대체한 뒤 이를 새 String에 넣어 반환
    // 만약 없다면 원본 텍스트의 포인터를 반환한다.
    // ㄴ 매칭이 하든 말든 복사본이 필요하기 때문에 to_String()을 사용한다.
    Ok(regex.replace_all(text, replacement).to_string()) //
}
