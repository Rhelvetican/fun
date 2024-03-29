use std::{
    cmp::{Eq, PartialEq},
    hash::Hash,
};
use strum_macros::{Display as StrumDisplay, EnumIter, EnumString};

#[derive(Debug, PartialEq, Eq, Hash, EnumIter, EnumString, StrumDisplay)]
pub enum Action {
    // Change Directories
    ChangeDir,
    ChangeDirParent,
    ChangeDirRoot,
    ChangeDirHome,
    ChangeDirAndExit,
    // Cursor Actions
    CursorUp,
    CursorDown,
    CursorUpScreen,
    CursorDownScreen,
    CursorTop,
    CursorBottom,
    // Search Actions
    EraseSearchChar,
    ClearSearch,
    ChangeFilterSearchMode,
    ChangeCaseSensitiveMode,
    ChangeGapSearchMode,
    ChangeSortMode,
    RefreshListing,
    // System Actions
    Help,
    Exit,
    ExitWithoutCd,
    None,
}

impl Action {
    pub fn desc(&self) -> &'static str {
        match self {
            Self::ChangeDir => "Enter directory under the cursor.",
            Self::ChangeDirParent => "Go to the parent directory.",
            Self::ChangeDirHome => "Go to the home directory.",
            Self::ChangeDirRoot => "Go to the root directory.",
            Self::ChangeDirAndExit => "Enter the directory under the cursor and exit.",
            Self::CursorUp => "Move the cursor up by one step.",
            Self::CursorDown => "Move the cursor down by one step.",
            Self::CursorUpScreen => "Move the cursor up by one screenful.",
            Self::CursorDownScreen => "Move the cursor down by one screenful.",
            Self::CursorTop => "Move the cursor to the first item in the listing.",
            Self::CursorBottom => "Move the cursor to the last item in the listing.",
            Self::EraseSearchChar => "Erase one character from the search.",
            Self::ClearSearch => "Clear the search.",
            Self::ChangeFilterSearchMode => "Toggle the filter-search mode.",
            Self::ChangeCaseSensitiveMode => "Change the case-sensitive mode.",
            Self::ChangeGapSearchMode => "Change the gap-search mode.",
            Self::ChangeSortMode => "Change the sorting mode.",
            Self::RefreshListing => "Refresh the directory listing.",
            Self::Help => "Show the help screen.",
            Self::Exit => "Exit the program.",
            Self::ExitWithoutCd => "Exit the program without changing the working directory.",
            Self::None => "Disable this mapping.",
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, StrumDisplay, EnumString, EnumIter)]
pub enum ActionContext {
    None,
    Search,
    NotSearch,
}

impl ActionContext {
    pub fn desc(&self) -> &'static str {
        match self {
            Self::None => "This mapping applies if no other context applies. This is the behavior if no context is specified: the mapping 'key-combination:action' is equivalent to 'key-combination:None:action'.",
            Self::Search => "This mapping only applies while searching (at least one search character has been given).",
            Self::NotSearch => "This mapping only applies while not searching.",
        }
    }
    pub fn short_desc(&self) -> &'static str {
        match self {
            Self::None => "No Context",
            Self::Search => "Search",
            Self::NotSearch => "Not Search",
        }
    }
}
