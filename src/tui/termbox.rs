//! Some utilities for termbox

use config::Style;
use termbox_simple::Termbox;

pub fn print(tb: &mut Termbox, mut pos_x: i32, pos_y: i32, style: Style, str: &str) {
    for char in str.chars() {
        tb.change_cell(pos_x, pos_y, char, style.fg, style.bg);
        pos_x += 1;
    }
}

pub fn print_chars<C>(tb: &mut Termbox, mut pos_x: i32, pos_y: i32, style: Style, chars: C)
where
    C: Iterator<Item = char>,
{
    for char in chars {
        tb.change_cell(pos_x, pos_y, char, style.fg, style.bg);
        pos_x += 1;
    }
}
