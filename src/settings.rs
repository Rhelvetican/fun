use crate::error::ProgramErr;
use crate::tui::act::{Action, ActionContext};

use clap::{error::ErrorKind as ClapErrKind, ArgMatches, Error as ClapErr, ValueEnum};
use crokey::key;
use crossterm::event::KeyEvent;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    path::PathBuf,
    result::Result as StdResult,
    str::FromStr,
};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq)]
pub enum FileHandleMode {
    Ignore,
    Hide,
    Match,
}

impl FileHandleMode {
    pub fn no_matches_msg(&self) -> &'static str {
        match self {
            Self::Ignore => "No folder matching search term.",
            _ => "No matches.",
        }
    }
}

impl Default for FileHandleMode {
    fn default() -> Self {
        Self::Ignore
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CaseSensitiveMode {
    Ignore,
    Sensitive,
    Smart,
}

impl Default for CaseSensitiveMode {
    fn default() -> Self {
        Self::Smart
    }
}

impl Display for CaseSensitiveMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Ignore => write!(f, "Ignore"),
            Self::Sensitive => write!(f, "Sensitive"),
            Self::Smart => write!(f, "Smart"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GapSearchMode {
    Normal,
    NormalAny,
    GapFromStart,
    GapAny,
}

impl Default for GapSearchMode {
    fn default() -> Self {
        Self::GapFromStart
    }
}

impl Display for GapSearchMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Normal => write!(f, "Normal Search"),
            Self::NormalAny => write!(f, "Normal Search Anywhere"),
            Self::GapFromStart => write!(f, "Gap Search From Start"),
            Self::GapAny => write!(f, "Gap Search Anywhere"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, clap::ValueEnum)]
pub enum SortMode {
    Name,
    Created,
    Modified,
}

impl Default for SortMode {
    fn default() -> Self {
        Self::Name
    }
}

impl Display for SortMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Name => write!(f, "Name"),
            Self::Created => write!(f, "Created"),
            Self::Modified => write!(f, "Modified"),
        }
    }
}

#[derive(Default)]
pub struct Settings {
    pub file_handle_mode: FileHandleMode,
    pub filter_search: bool,
    pub case_sensitive_mode: CaseSensitiveMode,
    pub sort_mode: SortMode,
    pub gap_search_mode: GapSearchMode,
    pub autocd_timeout: Option<u32>,
    pub history_file: Option<PathBuf>,
    pub mouse_enable: bool,
    pub keybinds: HashMap<(KeyEvent, ActionContext), Action>,
}

pub type DeprecationWarns = Vec<&'static str>;

impl Settings {
    pub fn parse_from_cli(args: &ArgMatches) -> StdResult<(Self, DeprecationWarns), ProgramErr> {
        let mut ret = Self::default();
        let mut warns: DeprecationWarns = vec![];

        ret.file_handle_mode = match args.get_one::<String>("files").unwrap().as_str() {
            "ignore" | "i" => FileHandleMode::Ignore,
            "hide" | "h" => FileHandleMode::Hide,
            "match" | "m" => FileHandleMode::Match,
            _ => {
                warns.push("Invalid value for '--files' / '-F', defaulting to 'ignore'.");
                FileHandleMode::default()
            }
        };

        if args.get_flag("folders-only") {
            ret.file_handle_mode = FileHandleMode::Hide;
            warns.push("The option '--folders-only' / '-d' has been deprecated, please use '--files hide' instead.");
        }

        if args.get_flag("no-folders-only") {
            ret.file_handle_mode = FileHandleMode::Ignore;
            warns.push("The option '--no-folders-only' / '-D' has been deprecated, please use '--files ignore' or '--files match' instead.");
        }

        if args.get_flag("filter-search") {
            ret.filter_search = true;
            warns.push("The option '--filter-search' / '-f' has been deprecated, please use '--files match' instead.");
        }

        ret.case_sensitive_mode = match args.get_one::<String>("case-sensitive").unwrap().as_str() {
            "ignore" | "i" => CaseSensitiveMode::Ignore,
            "sensitive" | "s" => CaseSensitiveMode::Sensitive,
            "smart" | "S" => CaseSensitiveMode::Smart,
            _ => {
                warns.push("Invalid value for '--case-sensitive' / '-c', defaulting to 'smart'.");
                CaseSensitiveMode::default()
            }
        };

        ret.gap_search_mode = match args.get_one::<String>("gap-search").unwrap().as_str() {
            "normal" | "n" => GapSearchMode::Normal,
            "normal-any" | "N" => GapSearchMode::NormalAny,
            "gap-from-start" | "g" => GapSearchMode::GapFromStart,
            "gap-any" | "G" => GapSearchMode::GapAny,
            _ => {
                warns.push(
                    "Invalid value for '--gap-search' / '-g', defaulting to 'gap-from-start'.",
                );
                GapSearchMode::default()
            }
        };

        ret.autocd_timeout = match  {
            
        }

        Ok((ret, warns))
    }
}
