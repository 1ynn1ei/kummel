pub fn is_numeric(char: &u8) -> bool {
    matches!(
        char,
        b'-'|
        b'.'|
        b'_'|
        b'a'|
        b'b'|
        b'c'|
        b'd'|
        b'e'|
        b'f'|
        b'x'|
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

pub fn is_operator_candidate(char: &u8) -> bool {
    matches!(
        char,
        b'!'|
        b'%'|
        b'&'|
        b'('|
        b')'|
        b'+'|
        b','|
        b'-'|
        b'.'|
        b':'|
        b';'|
        b'<'|
        b'='|
        b'>'|
        b'?'|
        b'['|
        b']'|
        b'^'|
        b'{'|
        b'|'|
        b'}'|
        b'~'|
        b'/'
    )
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
