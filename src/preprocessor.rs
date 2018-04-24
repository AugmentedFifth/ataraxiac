pub fn strip_comments(source: &mut String) {
    loop {
        let (mut begin, mut end) = (None, None);
        let (mut char_, mut str_, mut block, mut line) =
            (false, false, false, false);
        let mut char_indices = source.char_indices();
        while let Some((i, c)) = char_indices.next() {
            if char_ {
                if c == '\\' {
                    char_indices.next();
                } else if c == '\'' {
                    char_ = false;
                }
            } else if str_ {
                if c == '\\' {
                    char_indices.next();
                } else if c == '"' {
                    str_ = false;
                }
            } else if block {
                if c == '\\' {
                    while let Some((i_, c_)) = char_indices.next() {
                        if c_ == '}' {
                            end = Some(i_ + 1);
                            break;
                        } else if c_ == '\\' {
                            continue;
                        } else {
                            break;
                        }
                    }

                    if end.is_some() {
                        break;
                    }
                }
            } else if line {
                if c == '\n' || c == '\r' {
                    end = Some(i);
                    break;
                }
            } else {
                match c {
                    '\\' => {
                        begin = Some(i);
                        match char_indices.next() {
                            Some((_, '{'))     => block = true,
                            Some((i_, '\n'))
                            | Some((i_, '\r')) => end = Some(i_),
                            _                  => line = true,
                        }
                    },
                    '\'' => char_ = true,
                    '"' => str_ = true,
                    _ => (),
                }
            }
        }

        if let Some(begin) = begin {
            if let Some(end) = end {
                source.drain(begin..end);
            } else {
                source.drain(begin..);
            }
        } else {
            break;
        }
    }
}
