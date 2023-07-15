pub fn get_last_bracket<T: Into<String>>(
    doc: T,
    offset: usize,
    bracket_type: BracketType,
) -> usize {
    let doc: String = doc.into();
    let mut minuser: isize = 0;

    let pat = r#"\""#;
    let counts = doc.matches(pat).count();

    minuser -= counts as isize * 2;

    let doc = doc.replace(pat, "");

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
                let mut last_index: isize = index as isize + offset as isize + 1;

                if minuser < 0 {
                    last_index -= minuser;
                }

                if let Ok(last_index) = usize::try_from(last_index) {
                    return last_index;
                }

                return offset;
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
