use std::fs;
use std::env;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn is_keyword(word: &str, language: &str) -> bool {
    match language {
        "none" => false,
        "rust" => match word {
            "fn" | "let" | "mut" | "if" | "else" | "while" | "loop" | "match" | "use" | "mod" | "const" | "impl" | "struct" | "enum" | "trait" | "pub" | "self" | "super" | "break" | "continue" | "return" | "as" | "crate" | "for" => true,
            _ => false,
        },
        "python" => match word {
            "and" | "as" | "assert" | "break" | "class" | "continue" | "def" | "del" | "elif" | "else" | "except" | "finally" | "for" | "from" | "global" | "if" | "import" | "in" | "is" | "lambda" | "nonlocal" | "not" | "or" | "pass" | "raise" | "return" | "try" | "while" | "with" | "yield" => true,
            _ => false,
        },
        "c++" => match word {
            "alignas" | "alignof" | "and" | "and_eq" | "asm" | "auto" | "bitand" | "bitor" | "bool" | "break" | "case" | "catch" | "char" | "char16_t" | "char32_t" | "class" | "compl" | "const" | "constexpr" | "const_cast" | "continue" | "decltype" | "default" | "delete" | "do" | "double" | "dynamic_cast" | "else" | "enum" | "explicit" | "export" | "extern" | "false" | "float" | "for" | "friend" | "goto" | "if" | "inline" | "include" | "int" | "long" | "mutable" | "namespace" | "new" | "noexcept" | "not" | "not_eq" | "nullptr" | "operator" | "or" | "or_eq" | "private" | "protected" | "public" | "register" | "reinterpret_cast" | "return" | "short" | "signed" | "sizeof" | "static" | "static_assert" | "static_cast" | "struct" | "switch" | "template" | "this" | "thread_local" | "throw" | "true" | "try" | "typedef" | "typeid" | "typename" | "union" | "unsigned" | "using" | "virtual" | "void" | "volatile" | "wchar_t" | "while" | "xor" | "xor_eq" => true,
            _ => false,
        },
        "niklang" => match word {
            // Ключові слова niklang
            _ => false,
        },
        "udav" => match word {
            "друк" | "ввід" | "якщо" | "інакше" | "інакшеЯкщо" | "правда" | "брехня" | "або" | "не" | "та" | "для" | "поки" | "функція" | "зупинити" | "продовжити" | "повернути" | "пропустити" | "клас" | "як" | "від" | "отримати" | "очікувати" | "ніщо" | "окрім" | "до" | "викинути" | "нарешті" | "існує" | "лямбда" | "спробувати" | "глобально" | "неЛокально" | "ствердити" | "вилучити" | "застосовуючи" | "асинхронний" | "генерувати" | "ціле" | "дійсне" | "рядок" | "діапазон" | "себе" => true,
            _ => false,
        },
        "based" => match word {
            "ЗМІННА" | "ТАКОЖ" | "АБО" | "НЕ" | "ЯКЩО" | "ІНАКШЕ_ЯКЩО" | "ІНАКШЕ" | "ДЛЯ" | "ДО" | "КРОК" | "ПОКИ" | "ФУНКЦІЯ" | "ТОДІ" | "КІНЕЦЬ" | "ПОВЕРНУТИ" | "ПРОДОВЖИТИ" | "ЗУПИНИТИ" | "НІЩО" | "БРЕХНЯ" | "ПРАВДА" | "ПІ" | "ДРУК" | "ДРУК_РЕЗУЛЬТАТУ" | "ВВІД" | "ВВІД_ЧИСЛА" | "ЧИ_ЧИСЛО" | "ЧИ_РЯДОК" | "ЧИ_МАСИВ" | "ЧИ_ФУНКЦІЯ" | "ДОДАТИ" | "ВИЛУЧИТИ" | "РОЗШИРИТИ" | "ДОВЖИНА" | "ЗАПУСК" | "ОТРИМАТИ" => true,
            _ => false,
        },
        _ => false,
    }
}

fn is_brackets(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' | '"' | '\'' => true,
        _ => false,
    }
}

fn is_operators(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '>' | '<' | '=' | '.' | ':' | '|' => true,
        _ => false,
    }
}

fn is_comment(next_chars: &[char], language: &str, is_in_quotes: bool) -> bool {
    if is_in_quotes {
        return false;
    }

    match language {
        "rust" => next_chars.len() >= 2 && next_chars[0] == '/' && next_chars[1] == '/',
        "python" => next_chars.len() >= 1 && next_chars[0] == '#',
        "c++" => (next_chars.len() >= 2 && next_chars[0] == '/' && next_chars[1] == '/') ||
                 (next_chars.len() >= 2 && next_chars[0] == '/' && next_chars[1] == '*'),
        "niklang" => next_chars.len() >= 1 && next_chars[0] == '#',
        "udav" => next_chars.len() >= 1 && next_chars[0] == '#',
        "based" => next_chars.len() >= 1 && next_chars[0] == '#',
        _ => false,
    }
}

fn get_language_from_extension(file_path: &str) -> &str {
    match file_path.split('.').last() {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("cpp") => "c++",
        Some("nl") => "niklang",
        Some("udav") => "udav",
        Some("based") => "based",
        _ => "none",
    }
}

fn main() {
    const COLOR_RESET: &str = "\x1b[0m";
    const COLOR_KEYWORD: &str = "\x1b[0;36m";
    const COLOR_QUOTES: &str = "\x1b[0;35m";
    const COLOR_COMMENT: &str = "\x1b[0;90m";
    const COLOR_BRACKETS: &str = "\x1b[0;35m";
    const COLOR_OPERATORS: &str = "\x1b[0;35m";

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let file_path = &args[1];
        let language = get_language_from_extension(file_path);
        let mut sleep_interval: u64 = 100;

        if args.len() == 3 {
            if let Some(sleep_interval_str) = args.get(2) {
                if let Ok(interval) = sleep_interval_str.parse::<f64>() {
                    if interval >= 0.0 {
                        sleep_interval = (interval * 1000.0) as u64;
                    } else {
                        eprintln!("Неправильний інтервал сну, використовується значення за замовчуванням");
                    }
                } else {
                    eprintln!("Неправильний формат інтервалу сну, використовується значення за замовчуванням");
                }
            } else {
                eprintln!("Не вдалося отримати значення інтервалу сну, використовується значення за замовчуванням");
            }
        }

        // Зчитуємо вміст файлу
        match fs::read_to_string(file_path) {
            Ok(content) => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                let mut prev_char: char = ' ';
                let mut current_word: String = String::new();
                let mut is_in_quotes: bool = false;
                let mut is_in_comment: bool = false;
                let mut is_multiline_comment: bool = false;
                let mut next_chars: Vec<char> = Vec::new();

                for (i, c) in content.chars().enumerate() {
                    next_chars.clear();
                    for j in i+1..i+3 {
                        if j < content.len() {
                            next_chars.push(content.chars().nth(j).unwrap());
                        }
                    }

                    if is_comment(&&next_chars[..], language, is_in_quotes) {
                        is_in_comment = true;
                        if language == "c++" && next_chars.len() >= 2 && next_chars[0] == '/' && next_chars[1] == '*' {
                            is_multiline_comment = true;
                        }
                        print!("{}{}", COLOR_COMMENT, c);
                    } else if is_in_comment {
                        print!("{}{}", COLOR_COMMENT, c);
                        if (language == "c++" && is_multiline_comment && c == '/' && prev_char == '*') ||
                           (language != "c++" && c == '\n') {
                            is_in_comment = false;
                            is_multiline_comment = false;
                        }
                        if c == '\n' {
                            is_in_comment = false;
                            is_multiline_comment = false;
                        }
                    } else if c == '"' && prev_char != '\\' {
                        is_in_quotes = !is_in_quotes;
                        print!("{}{}{}", COLOR_QUOTES, c, COLOR_RESET);
                    } else if c == '\'' && prev_char != '\\' {
                        is_in_quotes = !is_in_quotes;
                        print!("{}{}{}", COLOR_QUOTES, c, COLOR_RESET);
                    } else if is_in_quotes {
                        print!("{}{}{}", COLOR_QUOTES, c, COLOR_RESET);
                    } else if c.is_alphanumeric() || c == '_' {
                        current_word.push(c);
                    } else {
                        if !current_word.is_empty() {
                            if is_keyword(&current_word, language) {
                                print!("{}{}{}", COLOR_KEYWORD, current_word, COLOR_RESET);
                            } else {
                                print!("{}", current_word);
                            }
                            current_word.clear();
                        }
                        if is_brackets(c) {
                            print!("{}{}{}", COLOR_BRACKETS, c, COLOR_RESET);
                            if c == '\'' {
                                is_in_quotes = !is_in_quotes;
                            }
                        } else if is_operators(c) {
                            print!("{}{}{}", COLOR_OPERATORS, c, COLOR_RESET)
                        } else {
                            print!("{}", c);
                        }
                        handle.flush().expect("Не вдалося очистити буфер виведення");
                        thread::sleep(Duration::from_millis(sleep_interval));
                    }
                    prev_char = c;
                }

                if !current_word.is_empty() {
                    if is_keyword(&current_word, language) {
                        print!("{}{}{}", COLOR_KEYWORD, current_word, COLOR_RESET);
                    } else {
                        print!("{}", current_word);
                    }
                }
            }
            Err(err) => eprintln!("Помилка: {}", err),
        }
    } else {
        eprintln!("Usage: code-printer <file_path> <sleep interval in seconds>");
    }
}