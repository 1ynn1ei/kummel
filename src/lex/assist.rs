use crate::def::pattern;
use crate::stream::Stream;

pub fn walk_until_terminate(
    stream: &mut Stream) -> (usize, usize) {
    let start = stream.cursor();
    while !pattern::is_line_terminator(&stream.current()) {
        stream.step();
    }
   (start, stream.cursor()) 
}

pub fn walk_until_expect_expect(
    stream: &mut Stream,
    expect1: u8,
    expect2: u8) -> (usize, usize) {
    let start = stream.cursor();
    loop {
        let Some(next) = stream.peek() else { break; };
        if !(stream.current() == expect1 && next == expect2) {
            stream.step();
        } else {
            break;
        }
    }
    let end = stream.cursor();
    stream.step();
    stream.step();
    (start, end)
}

pub fn walk_until_expect_or_terminate(
    stream: &mut Stream,
    expect: u8) -> (usize, usize) {
    let start_idx = stream.cursor(); 
    while
        stream.current() != expect
        && !pattern::is_line_terminator(&stream.current()) {
            stream.step();
        }
    (start_idx, stream.cursor())
}

pub fn walk_until_not_matches(
    stream: &mut Stream,
    f: &dyn Fn(&u8) -> bool) -> (usize, usize) {
    let start_idx = stream.cursor();
    while
        !stream.is_eof() && f(&stream.current()) {
            stream.step();
        }
    (start_idx, stream.cursor())
}

pub fn slice_into_str(slice: &[u8]) -> String {
    std::str::from_utf8(slice).unwrap().to_string()
}
