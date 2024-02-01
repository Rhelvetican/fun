pub mod act;
pub mod help;
pub mod render;

use std::{
    convert::TryFrom,
    fmt::Write as _,
    io::{Stderr, Write},
    path::PathBuf,
};

const HEADER_SIZE: usize = 1;
const INFO_WIN_SIZE: usize = 1;
const FOOTER_SIZE: usize = 1;
