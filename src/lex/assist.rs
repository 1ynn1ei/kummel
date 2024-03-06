use crate::def::pattern;
use crate::stream::Stream;

pub fn walk_until_terminate(
    stream: &mut Stream) -> usize {
    let start_idx = stream.cursor();
    while !pattern::is_line_terminator(&stream.current()) {
        stream.step();
    }
    start_idx
}

pub fn walk_until_expect_expect(
    stream: &mut Stream,
    expect1: u8,
    expect2: u8) -> usize {
    let start_idx = stream.cursor();
    let prev = stream.current();
    while 
        !(stream.is_eof() || prev == expect1 && stream.current() == expect2) {
        stream.step();
    }
    start_idx
}

pub fn walk_until_expect_or_terminate(
    stream: &mut Stream,
    expect: u8) -> usize {
    let start_idx = stream.cursor(); 
    while
        stream.current() != expect
        && !pattern::is_line_terminator(&stream.current()) {
            stream.step();
        }
    start_idx
}

pub fn walk_until_not_matches(
    stream: &mut Stream,
    f: &dyn Fn(&u8) -> bool) -> usize {
    let start_idx = stream.cursor();
    while
        !stream.is_eof() && f(&stream.current()) {
            stream.step();
        }
    start_idx
}

pub fn slice_into_str(slice: &[u8]) -> &str {
    std::str::from_utf8(slice).unwrap()
}
