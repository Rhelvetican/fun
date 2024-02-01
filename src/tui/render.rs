use crossterm::style::{StyledContent, Stylize};

pub const README_STR: &str = include_str!("../../README.md");

pub fn wrap_stylize(text: &str, size: usize) -> Vec<Vec<StyledContent<String>>> {
    let (mut txt, bold_pos) = strip_markup_extract_bold_pos(text);

    textwrap::fill_inplace(&mut txt, size);

    let mut res = stylize_lines(txt.split('\n').collect(), bold_pos);

    res.drain(..)
        .map(|mut line| {
            line.drain(..)
                .filter(|item| !item.content().is_empty())
                .collect()
        })
        .collect()
}

fn strip_markup_extract_bold_pos(text: &str) -> (String, Vec<usize>) {
    let mut bold_pos: Vec<usize> = vec![];
    let mut help_str_no_mkup = String::new();
    let mut prev_char: Option<char> = None;
    let mut parse_heading = false;
    let mut count = 0;

    for c in text.chars() {
        if c == '#' {
            if !parse_heading {
                parse_heading = true;
                bold_pos.push(count);
            }
        } else if c == ' ' && parse_heading && prev_char == Some('#') {
        } else if c == '\n' && parse_heading {
            bold_pos.push(count);
            parse_heading = false;
            count += 1;
            help_str_no_mkup.push(c);
        } else if c == '`' {
            bold_pos.push(count);
        } else {
            count += 1;
            help_str_no_mkup.push(c);
        }
        prev_char = Some(c);
    }

    (help_str_no_mkup, bold_pos)
}

fn stylize_lines<S>(lines: Vec<S>, bold_pos: Vec<usize>) -> Vec<Vec<StyledContent<String>>>
where
    S: AsRef<str>,
{
    let mut count: usize = 0;
    let mut bold_pos = bold_pos.iter();
    let mut next_bold_pos: Option<&usize> = bold_pos.next();
    let mut res: Vec<Vec<StyledContent<String>>> = vec![];
    let mut bold = false;

    for line in lines {
        let mut line_chunk = vec![];
        let mut current_chunk = String::new();

        for c in line.as_ref().chars() {
            if Some(&count) == next_bold_pos {
                line_chunk.push(if bold {
                    current_chunk.bold()
                } else {
                    current_chunk.stylize()
                });
                bold = !bold;
                next_bold_pos = bold_pos.next();
                current_chunk = String::new();
            }
            current_chunk.push(c);
            count += 1;
        }

        if !current_chunk.is_empty() {
            line_chunk.push(if bold {
                current_chunk.bold()
            } else {
                current_chunk.stylize()
            });
        };

        if bold {
            bold = false;
            next_bold_pos = bold_pos.next();
        }

        res.push(line_chunk);
        count += 1;
    }

    res
}
