pub fn is_numeric(char: &u8) -> bool {
    matches!(
        char,
        b'_'|
        b'.'|
        b'e'|
        b'0'..=b'9'
    )
}

pub fn is_literal(char: &u8) -> bool {
    matches!(
        char,
        b'_'|
        b'0'..=b'9'|
        b'a'..=b'z'|
        b'A'..=b'Z'
     )
}

// b'!'|
// b'%'|
// b'&'|
// b'('|
// b')'|
// b'+'|
// b','|
// b'-'|
// b'.'|
// b':'|
// b';'|
// b'<'|
// b'='|
// b'>'|
// b'?'|
// b'['|
// b']'|
// b'^'|
// b'{'|
// b'|'|
// b'}'|
// b'~'|
// b'/'
pub fn is_operator_candidate(char: &u8) -> bool {
    !is_numeric(char)
        && !is_literal(char)
        && !is_whitespaceish(char)
        && !is_line_terminator(char)
}

pub fn is_whitespaceish(char: &u8) -> bool {
    matches!(
        char,
        b'\t'  |
        0x000C | /* form feed */
        0x000B | /* line tab */
        0x00A0 | /* no-break space */
        b' '
    )
}

pub fn is_line_terminator(char: &u8) -> bool {
    matches!(
        char,
        b'\r'|
        b'\n'
    )
}
