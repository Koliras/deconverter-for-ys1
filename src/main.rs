use std::{env, fs, io::Write, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_content = fs::read_to_string(config.file_path).unwrap();

    read_and_write_file_contents(&file_content);
}

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a path to file"),
        };

        Ok(Self { file_path })
    }
}

fn read_and_write_file_contents<'a>(content: &'a str) {
    let mut result_file = fs::File::create("converted.txt").unwrap();
    let mut should_convert = false;
    content.lines().for_each(|line| {
        if line.starts_with("msgstr") {
            should_convert = true;

            let deconverted_line = deconvert(&line[7..]);

            let mut string_to_write = String::from("msgstr ");
            string_to_write.push_str(&deconverted_line);
            string_to_write.push_str("\n");

            result_file
                .write(string_to_write.as_bytes())
                .expect("failed to write string with msgstr");
            return;
        }

        if line.len() < 3 {
            should_convert = false;
        }

        if !should_convert {
            result_file
                .write(line.as_bytes())
                .expect("failed to write to file");
            result_file
                .write("\n".as_bytes())
                .expect("failed to write to file");
            return;
        }

        result_file
            .write(deconvert(&line).as_bytes())
            .expect("failed to write converted line without msgstr");
        result_file
            .write("\n".as_bytes())
            .expect("failed to write to file");
    })
}

pub fn deconvert(line: &str) -> String {
    let mut deconverted_line = String::new();
    line.chars().for_each(|char| match char {
        'a' => deconverted_line.push_str("а"),
        'b' => deconverted_line.push_str("б"),
        'c' => deconverted_line.push_str("в"),
        'd' => deconverted_line.push_str("г"),
        '~' => deconverted_line.push_str("ґ"),
        'e' => deconverted_line.push_str("д"),
        'f' => deconverted_line.push_str("е"),
        '%' => deconverted_line.push_str("є"),
        'g' => deconverted_line.push_str("ж"),
        'h' => deconverted_line.push_str("з"),
        'i' => deconverted_line.push_str("и"),
        '&' => deconverted_line.push_str("і"),
        '@' => deconverted_line.push_str("ї"),
        'j' => deconverted_line.push_str("й"),
        'k' => deconverted_line.push_str("к"),
        'l' => deconverted_line.push_str("л"),
        'm' => deconverted_line.push_str("м"),
        'n' => deconverted_line.push_str("н"),
        'o' => deconverted_line.push_str("о"),
        'p' => deconverted_line.push_str("п"),
        'q' => deconverted_line.push_str("р"),
        'r' => deconverted_line.push_str("с"),
        's' => deconverted_line.push_str("т"),
        't' => deconverted_line.push_str("у"),
        'u' => deconverted_line.push_str("ф"),
        'v' => deconverted_line.push_str("х"),
        'w' => deconverted_line.push_str("ц"),
        'x' => deconverted_line.push_str("ч"),
        'y' => deconverted_line.push_str("ш"),
        'z' => deconverted_line.push_str("щ"),
        '{' => deconverted_line.push_str("ь"),
        '|' => deconverted_line.push_str("ю"),
        '}' => deconverted_line.push_str("я"),
        'A' => deconverted_line.push_str("А"),
        'B' => deconverted_line.push_str("Б"),
        'C' => deconverted_line.push_str("В"),
        'D' => deconverted_line.push_str("Г"),
        ']' => deconverted_line.push_str("Ґ"),
        'E' => deconverted_line.push_str("Д"),
        'F' => deconverted_line.push_str("Е"),
        '^' => deconverted_line.push_str("Є"),
        'G' => deconverted_line.push_str("Ж"),
        'H' => deconverted_line.push_str("З"),
        'I' => deconverted_line.push_str("И"),
        '+' => deconverted_line.push_str("І"),
        ';' => deconverted_line.push_str("Ї"),
        'J' => deconverted_line.push_str("Й"),
        'K' => deconverted_line.push_str("К"),
        'L' => deconverted_line.push_str("Л"),
        'M' => deconverted_line.push_str("М"),
        'N' => deconverted_line.push_str("Н"),
        'O' => deconverted_line.push_str("О"),
        'P' => deconverted_line.push_str("П"),
        'Q' => deconverted_line.push_str("Р"),
        'R' => deconverted_line.push_str("С"),
        'S' => deconverted_line.push_str("Т"),
        'T' => deconverted_line.push_str("У"),
        'U' => deconverted_line.push_str("Ф"),
        'V' => deconverted_line.push_str("Х"),
        'W' => deconverted_line.push_str("Ц"),
        'X' => deconverted_line.push_str("Ч"),
        'Y' => deconverted_line.push_str("Ш"),
        'Z' => deconverted_line.push_str("Щ"),
        '=' => deconverted_line.push_str("Ь"),
        '[' => deconverted_line.push_str("Ю"),
        '/' => deconverted_line.push_str("Я"),
        x => deconverted_line.push_str(&x.to_string()),
    });
    return deconverted_line;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deconvert_test() {
        let test_string = "Rq&bnij mfx";
        let wanted_result = "Срібний меч";
        let actual_result = deconvert(test_string);
        assert_eq!(wanted_result, actual_result)
    }
}
