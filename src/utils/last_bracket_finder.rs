use regex::Regex;

pub fn get_last_bracket<T: Into<String>>(
    doc: T,
    offset: usize,
    bracket_type: BracketType,
) -> usize {
    let doc: String = doc.into();

    let re = Regex::new(r#"\\""#).unwrap();
    let doc = re.replace(&doc, "");

    let mut counter = 0;
    let mut switch = false;

    let mut open = '{';
    let mut close = '}';

    if bracket_type == BracketType::Square {
        open = '[';
        close = ']';
    }

    for (index, char) in doc.chars().enumerate() {
        if char == '"' {
            switch = !switch;
        }

        if switch {
            continue;
        }

        if char == open {
            counter += 1;
        }

        if char == close {
            counter -= 1;

            if counter == 0 {
                return index + 2 + offset;
            }
        }
    }

    offset
}

#[derive(Debug, PartialEq)]
pub enum BracketType {
    Curly,
    Square,
}
