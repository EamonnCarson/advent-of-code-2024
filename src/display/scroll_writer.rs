use console::{pad_str, pad_str_with, Alignment};
use itertools::Itertools;
pub struct ScrollWriter {
    terminal_width: usize,
    curr_line_num: usize,
}

const SCROLL_SIDE: &[&str] = &[
    r"  \  ", //
    r"   \ ", //
    r"   | ", //
    r"   | ", //
    r"   / ", //
    r"  /  ", //
    r" |   ", //
    r" |   ", //
];

// pub
impl ScrollWriter {
    pub fn new() -> Self {
        let mut s = Self {
            terminal_width: console::Term::stdout().size().1 as usize,
            curr_line_num: 0,
        };
        s.write_header();
        s
    }

    pub fn writeln_aligned(&mut self, text: &str, align: Alignment) {
        if text.is_empty() {
            self.write_single_line("", align);
        }
        for line in text.lines() {
            self.write_long_line(line, align);
        }
    }

    pub fn writeln(&mut self, text: &str) {
        self.writeln_aligned(text, Alignment::Left);
    }

    pub fn finish(mut self) {
        self.write_footer();
    }
}

// private
impl ScrollWriter {
    fn write_header(&mut self) {
        println!(
            " {}",
            std::iter::repeat('_')
                .take(self.terminal_width - self.padding_size())
                .join("")
        );
    }
    fn write_side(&mut self) {
        let curr_edge = SCROLL_SIDE[self.curr_line_num % SCROLL_SIDE.len()];
        print!("{curr_edge}");
    }
    fn padding_size(&self) -> usize {
        5
    }
    fn writeable_width(&self) -> usize {
        self.terminal_width - 2 * self.padding_size()
    }
    fn go_to_next_line(&mut self) {
        self.curr_line_num += 1;
        println!();
    }
    fn write_single_line(&mut self, line: &str, align: Alignment) {
        self.write_side();
        print!("{}", pad_str(line, self.writeable_width(), align, None));
        self.write_side();
        self.go_to_next_line();
    }
    fn write_long_line(&mut self, line: &str, align: Alignment) {
        if line.is_empty() {
            self.write_single_line("", align);
        }
        let writable_width = self.writeable_width();
        for chunk in split_str_into_chunks(line, writable_width) {
            self.write_single_line(chunk, align);
        }
    }
    fn write_footer(&mut self) {
        let print_with_spacer = |left: &str, right: &str, spacer: char| {
            print!("{left}");
            println!(
                "{}",
                pad_str_with(
                    right,
                    self.terminal_width - left.chars().count(),
                    Alignment::Right,
                    None,
                    spacer
                )
            );
        };
        let mut left_scroll = include_str!("assets/scroll_roll_left_edge.txt").lines();
        let mut right_scroll = include_str!("assets/scroll_roll_right_edge.txt").lines();
        let left_top_of_scroll = left_scroll.next().unwrap();
        let right_top_of_scroll = right_scroll.next().unwrap();
        print_with_spacer(left_top_of_scroll, right_top_of_scroll, '_');
        let left_bottom_of_scroll = left_scroll.next_back().unwrap();
        let right_bottom_of_scroll = right_scroll.next_back().unwrap();
        for line in left_scroll.zip(right_scroll) {
            print_with_spacer(line.0, line.1, ' ');
        }
        print_with_spacer(left_bottom_of_scroll, right_bottom_of_scroll, '‾');
    }
}

fn split_str_into_chunks(s: &str, chunk_size: usize) -> impl Iterator<Item = &str> {
    let mut start = 0;
    let len = s.len();
    std::iter::from_fn(move || {
        if start >= len {
            return None;
        }
        let end = s[start..]
            .char_indices()
            .nth(chunk_size)
            .map(|(idx, _)| start + idx)
            .unwrap_or(len);
        let chunk = &s[start..end];
        start = end;
        Some(chunk)
    })
}
