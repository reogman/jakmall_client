pub fn get_last_bracket<T: Into<String>>(
    doc: T,
    offset: usize,
    bracket_type: BracketType,
) -> usize {
    let doc: String = doc.into();
    let mut counter = 0;

    let mut open = '{';
    let mut close = '}';

    if bracket_type == BracketType::Square {
        open = '[';
        close = ']';
    }

    for (index, char) in doc.chars().enumerate() {
        if char == open {
            counter += 1;
        }

        if char == close {
            counter -= 1;

            if counter == 0 {
                return index + offset;
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