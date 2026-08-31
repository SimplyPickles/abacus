use abacus::Abacus;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Config, Editor, Helper};
use std::borrow::Cow;
use std::env;
use std::io::Write;
use std::path::PathBuf;

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

    fn highlight_char(
        &self,
        _line: &str,
        _pos: usize,
        _kind: rustyline::highlight::CmdKind,
    ) -> bool {
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
                // Command like .help or .clear
                result.push_str("\x1b[38;2;189;147;249m");
                result.push_str(word);
                result.push_str("\x1b[0m");
            } else {
                match word.to_lowercase().as_str() {
                    // Functions & constants
                    "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sqrt" | "log" | "ln"
                    | "exp" | "abs" | "ceil" | "floor" | "round" | "min" | "max" | "sum"
                    | "avg" | "mean" | "std" | "e" | "pi" => {
                        result.push_str("\x1b[38;2;139;233;253m"); // Lime Green Function
                        result.push_str(word);
                        result.push_str("\x1b[0m");
                    }
                    // Date & Relative Date keywords
                    "last" | "next" | "this" | "today" | "tomorrow" | "yesterday" | "monday"
                    | "tuesday" | "wednesday" | "thursday" | "friday" | "saturday" | "sunday"
                    | "january" | "february" | "march" | "april" | "may" | "june" | "july"
                    | "august" | "september" | "october" | "november" | "december" | "pm"
                    | "am" => {
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

fn print_welcome_banner() {
    println!("\x1b[1;36m=============================================================\x1b[0m");
    println!("               \x1b[1;33mABACUS INTERACTIVE SHELL (1.0)\x1b[0m               ");
    println!(" \x1b[1;34mType physical math expressions, date calculations, etc.\x1b[0m   ");
    println!(" \x1b[90mUse ↑/↓ Arrow Keys to navigate command history.\x1b[0m            ");
    println!(" \x1b[1;33mExamples:\x1b[0m                                                 ");
    println!(
        "    • \x1b[38;2;241;196;15m14 meters to inches\x1b[0m                               "
    );
    println!(
        "    • \x1b[38;2;139;233;253msin(30deg)\x1b[0m                                          "
    );
    println!(
        "    • \x1b[38;2;80;250;123mlast thursday at 3pm\x1b[0m                                 "
    );
    println!(
        "    • \x1b[38;2;139;233;253msqrt(14 m^3)\x1b[0m                                       "
    );
    println!(" \x1b[90mType '.help' for help, '.clear' to clear, 'exit' to leave.\x1b[0m");
    println!("\x1b[1;36m=============================================================\x1b[0m\n");
}

use rustyline::ColorMode;

fn main() {
    let calc = Abacus::standard();
    let config = Config::builder().color_mode(ColorMode::Forced).build();
    let mut rl = match Editor::with_config(config) {
        Ok(rl) => rl,
        Err(e) => {
            eprintln!("Failed to initialize interactive editor: {e}");
            return;
        }
    };
    rl.set_helper(Some(AbacusHelper));

    let history_path = get_history_path();
    if let Some(ref path) = history_path {
        let _ = rl.load_history(path);
    }

    print_welcome_banner();

    let prompt = "\x1b[36m›\x1b[0m ";

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

                if trimmed == "clear" {
                    print!("\x1b[2J\x1b[1;1H");
                    let _ = std::io::stdout().flush();
                    print_welcome_banner();
                    continue;
                }

                if trimmed == "help" {
                    println!("\x1b[38;2;189;147;249m  Abacus REPL Commands & Syntax:\x1b[0m");
                    println!("    help       Show this help summary");
                    println!("    clear      Clear the terminal screen");
                    println!("    :q / exit  Exit the REPL\n");
                    continue;
                }

                match calc.eval(trimmed) {
                    Ok(result) => {
                        println!(
                            "\x1b[38;2;139;233;253m{}\x1b[0m",
                            format_repl_result(&result)
                        );
                    }
                    Err(err) => {
                        println!("\x1b[31mError: {}\x1b[0m", err);
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

fn format_repl_result(result: &abacus::units::eval_result::EvalResult) -> String {
    match result {
        abacus::units::eval_result::EvalResult::Scalar(v) => {
            if v.unit.is_standard_duration_unit() {
                abacus::units::value::format_human_duration(v.canonical)
            } else {
                v.to_display()
            }
        }
        other => other.to_display(),
    }
}
