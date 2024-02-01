use crossterm::style::{StyledContent, Stylize};

pub const README_STR: &str = include_str!("../../README.md");

pub fn wrap_stylize(text: &str, size: usize) -> Vec<Vec<StyledContent<String>>> {
    todo!()
}

pub fn strip_markup_extract_bold_pos(text: &str) -> (String, Vec<usize>) {
    todo!()
}

fn stylize_lines<S>(lines: Vec<S>, bold_pos: Vec<usize>) -> Vec<Vec<StyledContent<String>>>
where
    S: AsRef<str>,
{
    todo!()
}
