use crate::tui::{
    act::{Action, ActionContext},
    render::{wrap_stylize, README_STR},
};
use crossterm::{event::KeyEvent, style::StyledContent};
use std::collections::HashMap;

pub fn format_help_txt(
    width: usize,
    key_map: &HashMap<(KeyEvent, ActionContext), Action>,
) -> Vec<Vec<StyledContent<String>>> {
    let help_str = &README_STR[README_STR
        .find("## User guide")
        .expect("Could not find user guide in README")
        ..README_STR
            .find("## Similar projects")
            .expect("Could not find end of user guide in README")];

    let (help_str, rest) = help_str
        .split_once("\n\n|")
        .expect("Could not find keyboard shortcuts table in readme");

    let rest = rest
        .split_once("\n\n")
        .expect("Could not find end of keyboard shortcuts table in readme")
        .1;

    let mut help_str = help_str.to_string();
    help_str.push_str("\n\n");
    help_str = help_str.replace("shortcuts by default", "shortcuts");
    help_str.push_str(&get_justified_keyboard_shortcuts_table(key_map));

    let rest = rest
        .split('\n')
        .filter(|line| !line.contains("should be familiar to"))
        .collect::<Vec<_>>()
        .join("\n");
    help_str.push_str(&rest);

    let help_str = help_str.replace("<kbd>", "`").replace("</kbd>", "`");

    wrap_stylize(&help_str, width)
}

fn get_keyboard_shortcuts_table() -> &'static str {
    let keyboard_shortcuts = README_STR
        .split_once("keyboard shortcuts by default:\n\n")
        .expect("Couldn't find table of keyboard shortcuts in README")
        .1;
    let keyboard_shortcuts = keyboard_shortcuts
        .split_once("\n\n")
        .expect("Couldn't find end of keyboard shortcuts table in README")
        .0;

    keyboard_shortcuts
}

fn get_justified_keyboard_shortcuts_table(
    key_map: &HashMap<(KeyEvent, ActionContext), Action>,
) -> String {
    let formatter = crokey::KeyCombinationFormat::default();

    let keyboard_shortcuts = get_keyboard_shortcuts_table();

    let first_column_width = keyboard_shortcuts
        .lines()
        .map(|line| line.split('|').nth(1).unwrap_or("").replace('`', "").len())
        .max()
        .unwrap_or(10);

    // Mapping from action names to list of key combinations
    let key_map_invert = invert_keymap_sorted(key_map);

    let mut justified = String::new();

    for (i, line) in keyboard_shortcuts.lines().enumerate() {
        // cols[0] is empty, because the lines start with '|'.
        let cols: Vec<&str> = line.split('|').map(|c| c.trim()).collect();

        let (action_desc, shortcuts) = match i {
            0 => (
                format!("`{}`", cols[1]),
                format!("`{}`", cols[2].replace("Default s", "S")),
            ),
            1 => continue,
            _ => {
                let action_name = cols[3].replace('`', "").trim().to_string();
                let shortcuts_formatted: String = match key_map_invert.get(&action_name) {
                    Some(shortcuts) => shortcuts
                        .iter()
                        .map(|(keys, ctx)| {
                            let mut shortcut = format!("`{}`", formatter.to_string(*keys));
                            let ctx = match ctx {
                                ActionContext::None => String::new(),
                                _ => format!(" ({})", ctx.short_desc()),
                            };
                            shortcut.push_str(&ctx);
                            shortcut
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                    None => "No mapping found".to_string(),
                };

                (cols[1].to_string(), shortcuts_formatted)
            }
        };

        justified.push_str(&action_desc);

        let extra_len = action_desc.chars().filter(|c| *c == '`').count();
        let padding = first_column_width + extra_len + 2 - action_desc.len();
        justified.push_str(&" ".repeat(padding));
        justified.push_str(&shortcuts);
        justified.push('\n');
    }

    justified
}

fn invert_keymap_sorted(
    key_map: &HashMap<(KeyEvent, ActionContext), Action>,
) -> HashMap<String, Vec<(KeyEvent, ActionContext)>> {
    let mut key_map_invert: HashMap<String, Vec<(KeyEvent, ActionContext)>> = HashMap::new();

    fn cmp_key_events(k1: &KeyEvent, k2: &KeyEvent) -> std::cmp::Ordering {
        let formatter = crokey::KeyCombinationFormat::default();
        match (k1.modifiers.is_empty(), k2.modifiers.is_empty()) {
            (true, true) | (false, false) => {
                // both or neither have modifiers, sort alphabetically
                formatter.to_string(*k1).cmp(&formatter.to_string(*k2))
            }
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
        }
    }

    for ((k, c), a) in key_map {
        key_map_invert
            .entry(a.to_string())
            .or_insert(vec![])
            .push((*k, c.clone()))
    }

    for (_, mappings) in key_map_invert.iter_mut() {
        mappings.sort_unstable_by(|(k1, c1), (k2, c2)| match (c1, c2) {
            (ActionContext::None, ActionContext::None) => cmp_key_events(k1, k2),
            (_, ActionContext::None) => std::cmp::Ordering::Greater,
            (ActionContext::None, _) => std::cmp::Ordering::Less,
            (_, _) => c1.to_string().cmp(&c2.to_string()),
        })
    }

    key_map_invert
}
