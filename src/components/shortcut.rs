//! Keyboard shortcuts as the platform writes them, in Omarchy's form:
//! lowercase key names joined by `-` (`ctrl-c`, `super-space`).
//!
//! `mod` is the platform's command key: `cmd` on macOS and iOS, `ctrl`
//! elsewhere; on macOS `alt` reads `option`. Any spelling is accepted
//! (`Ctrl+C`, `Ctrl C`, `ctrl-c`), so every shortcut on screen reads alike.

use nsh_core::platform::Os;
use nsh_core::prelude::*;

/// `keys` (`"mod-c"`, `"Ctrl+Shift+P"`, `"F2"`) as this platform writes it:
/// `"cmd-c"` on macOS, `"ctrl-c"` on Linux and Windows.
pub fn shortcut_label(keys: &str) -> SharedString {
    format_shortcut(keys, matches!(os(), Os::MacOs | Os::Ios)).into()
}

pub(crate) fn format_shortcut(keys: &str, apple: bool) -> String {
    keys.split(['+', '-', ' '])
        .filter(|k| !k.is_empty())
        .map(|key| {
            let key = key.to_lowercase();
            match (key.as_str(), apple) {
                ("mod", true) | ("cmd" | "command" | "meta" | "⌘", _) => "cmd".to_owned(),
                ("mod", false) | ("control" | "⌃", _) => "ctrl".to_owned(),
                ("alt" | "option" | "⌥", true) => "option".to_owned(),
                ("option" | "⌥", false) => "alt".to_owned(),
                ("escape", _) => "esc".to_owned(),
                ("return", _) => "enter".to_owned(),
                ("delete", false) => "del".to_owned(),
                ("del", true) => "delete".to_owned(),
                ("⇧", _) => "shift".to_owned(),
                _ => key,
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::format_shortcut;

    #[test]
    fn every_spelling_reads_the_same_on_each_platform() {
        assert_eq!(format_shortcut("mod-c", false), "ctrl-c");
        assert_eq!(format_shortcut("mod-c", true), "cmd-c");
        assert_eq!(format_shortcut("Ctrl+Shift+P", false), "ctrl-shift-p");
        assert_eq!(format_shortcut("Super Space", false), "super-space");
        assert_eq!(format_shortcut("Alt+F4", true), "option-f4");
        assert_eq!(format_shortcut("Del", true), "delete");
        assert_eq!(format_shortcut("F2", false), "f2");
    }
}
