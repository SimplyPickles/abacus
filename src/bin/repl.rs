use abacus::Abacus;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Config, Editor, Helper};
use std::borrow::Cow;
use std::env;
use std::path::PathBuf;
use terminal_size::{Width, terminal_size};

struct AbacusHelper;

impl Completer for AbacusHelper {
    type Candidate = String;
}

impl Hinter for AbacusHelper {
    type Hint = String;
}

impl Validator for AbacusHelper {}

impl Helper for AbacusHelper {}

impl Highlighter for AbacusHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        highlight_syntax(line).into()
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: rustyline::highlight::CmdKind) -> bool {
        true
    }
}

fn highlight_syntax(line: &str) -> String {
    let mut result = String::new();
    let mut chars = line.char_indices().peekable();

    while let Some(&(i, ch)) = chars.peek() {
        if ch.is_ascii_digit() {
            let start = i;
            while let Some(&(_, c)) = chars.peek() {
                if c.is_ascii_digit() || c == '.' || c == '_' {
                    chars.next();
                } else {
                    break;
                }
            }
            let end = chars.peek().map(|&(idx, _)| idx).unwrap_or(line.len());
            let num = &line[start..end];
            // Numbers in Yellow/Gold
            result.push_str("\x1b[38;2;241;196;15m");
            result.push_str(num);
            result.push_str("\x1b[0m");
        } else if ch.is_alphabetic() || ch == '_' || ch == '.' {
            let start = i;
            while let Some(&(_, c)) = chars.peek() {
                if c.is_alphanumeric() || c == '_' || c == '.' {
                    chars.next();
                } else {
                    break;
                }
            }
            let end = chars.peek().map(|&(idx, _)| idx).unwrap_or(line.len());
            let word = &line[start..end];

            if word.starts_with('.') {
                // Command like .help
                result.push_str("\x1b[38;2;189;147;249m");
                result.push_str(word);
                result.push_str("\x1b[0m");
            } else {
                match word.to_lowercase().as_str() {
                    // Functions & constants
                    "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sqrt" | "log" | "ln"
                    | "exp" | "abs" | "ceil" | "floor" | "round" | "min" | "max" | "sum" | "avg"
                    | "mean" | "std" | "e" | "pi" => {
                        result.push_str("\x1b[38;2;139;233;253m"); // Lime Green Function
                        result.push_str(word);
                        result.push_str("\x1b[0m");
                    }
                    // Date & Relative Date keywords
                    "last" | "next" | "this" | "today" | "tomorrow" | "yesterday" | "monday"
                    | "tuesday" | "wednesday" | "thursday" | "friday" | "saturday" | "sunday"
                    | "january" | "february" | "march" | "april" | "may" | "june" | "july"
                    | "august" | "september" | "october" | "november" | "december" | "pm" | "am" => {
                        result.push_str("\x1b[38;2;80;250;123m"); // Teal Date Keyword
                        result.push_str(word);
                        result.push_str("\x1b[0m");
                    }
                    // Units & Conversion keywords
                    "meters" | "meter" | "m" | "inches" | "inch" | "in" | "feet" | "ft" | "cm"
                    | "mm" | "km" | "miles" | "mi" | "kg" | "g" | "lbs" | "to" | "as" | "at"
                    | "hours" | "hour" | "h" | "minutes" | "mins" | "seconds" | "s" | "days"
                    | "day" | "weeks" | "week" | "months" | "years" | "workdays" | "business" => {
                        result.push_str("\x1b[38;2;189;147;249m"); // Soft Purple Unit
                        result.push_str(word);
                        result.push_str("\x1b[0m");
                    }
                    _ => {
                        result.push_str(word);
                    }
                }
            }
        } else if "+-*/^!()[]{},".contains(ch) {
            chars.next();
            result.push_str("\x1b[38;2;139;233;253m"); // Lime Green Operator
            result.push(ch);
            result.push_str("\x1b[0m");
        } else {
            chars.next();
            result.push(ch);
        }
    }

    result
}

fn get_history_path() -> Option<PathBuf> {
    env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".abacus_history"))
}

use rustyline::ColorMode;

fn main() {
    let calc = Abacus::standard();
    let config = Config::builder()
        .color_mode(ColorMode::Forced)
        .build();
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(AbacusHelper));

    let history_path = get_history_path();
    if let Some(ref path) = history_path {
        let _ = rl.load_history(path);
    }

    let prompt = "\x1b[36m›\x1b[0m ";
    let prompt_len = 2; // "› "

    loop {
        let readline = rl.readline(prompt);

        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(trimmed);
                if let Some(ref path) = history_path {
                    let _ = rl.save_history(path);
                }

                if trimmed.eq_ignore_ascii_case("exit")
                    || trimmed.eq_ignore_ascii_case("quit")
                    || trimmed == ":q"
                {
                    println!("\x1b[33mGoodbye!\x1b[0m");
                    break;
                }

                if trimmed == ".help" || trimmed == "help" {
                    println!("\x1b[38;2;189;147;249m  Abacus REPL Commands & Syntax:\x1b[0m");
                    println!("    • \x1b[38;2;241;196;15m14 meters to inches\x1b[0m");
                    println!("    • \x1b[38;2;139;233;253msin(1..3)\x1b[0m");
                    println!("    • \x1b[38;2;80;250;123mlast thursday at 3pm\x1b[0m");
                    println!("    • \x1b[38;2;139;233;253me^3 - 3!\x1b[0m");
                    println!("    • \x1b[38;2;139;233;253msqrt(14 m^3)\x1b[0m");
                    continue;
                }

                match calc.eval(trimmed) {
                    Ok(result) => {
                        let res_str = result.to_display();
                        let input_len = line.chars().count();
                        let res_len = res_str.chars().count();
                        let term_width = terminal_size()
                            .map(|(Width(w), _)| w as usize)
                            .unwrap_or(80);

                        let max_width = term_width.saturating_sub(2);
                        let occupied = prompt_len + input_len + res_len;

                        if max_width > occupied {
                            let padding_len = max_width - occupied;
                            let padding = " ".repeat(padding_len);
                            println!(
                                "\x1b[1A\r\x1b[K\x1b[36m›\x1b[0m {} {}\x1b[38;2;139;233;253m{}\x1b[0m",
                                highlight_syntax(&line),
                                padding,
                                res_str
                            );
                        } else {
                            let pad_left = max_width.saturating_sub(res_len);
                            let padding = " ".repeat(pad_left);
                            println!("{}\x1b[38;2;139;233;253m{}\x1b[0m", padding, res_str);
                        }
                    }
                    Err(err) => {
                        let err_str = format!("Error: {}", err);
                        let input_len = line.chars().count();
                        let err_len = err_str.chars().count();
                        let term_width = terminal_size()
                            .map(|(Width(w), _)| w as usize)
                            .unwrap_or(80);

                        let max_width = term_width.saturating_sub(2);
                        let occupied = prompt_len + input_len + err_len;

                        if max_width > occupied {
                            let padding_len = max_width - occupied;
                            let padding = " ".repeat(padding_len);
                            println!(
                                "\x1b[1A\r\x1b[K\x1b[36m›\x1b[0m {} {}\x1b[31m{}\x1b[0m",
                                highlight_syntax(&line),
                                padding,
                                err_str
                            );
                        } else {
                            let pad_left = max_width.saturating_sub(err_len);
                            let padding = " ".repeat(pad_left);
                            println!("{}\x1b[31m{}\x1b[0m", padding, err_str);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\x1b[33mCTRL-C\x1b[0m");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("\x1b[33mGoodbye!\x1b[0m");
                break;
            }
            Err(err) => {
                println!("\x1b[31mError: {:?}\x1b[0m", err);
                break;
            }
        }
    }
}
