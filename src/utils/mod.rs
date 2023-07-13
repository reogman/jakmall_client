mod macros;

pub fn get_last_bracket<T: Into<String>>(doc: T, offset: usize) -> usize {
    let doc: String = doc.into();
    let mut counter = 0;

    for (index, char) in doc.chars().enumerate() {
        if char == '{' {
            counter += 1;
        }

        if char == '}' {
            counter -= 1;

            if counter == 0 {
                return index + offset;
            }
        }
    }

    offset
}
