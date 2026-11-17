//! Reading and following the Omarchy system theme.
//!
//! Omarchy's current theme is a directory (often a symlink), not an OS flag:
//! `$HOME/.local/state/omarchy/current/theme/colors.toml` holds the palette and
//! `$HOME/.local/state/omarchy/current/theme.name` its name. Older
//! installations keep it in `$HOME/.config/omarchy/current`, which is read only
//! when the state directory is absent: an existing but broken current theme
//! falls back to Tokyo Night, never to a stale legacy theme.

mod parse;
#[cfg(not(target_arch = "wasm32"))]
mod watch;

use std::cell::RefCell;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use nsh_core::prelude::*;

use crate::theme::Theme;

/// Why the system theme could not be read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeLoadError(pub(crate) String);

impl fmt::Display for ThemeLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ThemeLoadError {}

/// The name of a theme without a non-empty `theme.name`.
const DEFAULT_NAME: &str = "Omarchy";

/// The `current` directory to read under `home`: the state directory, or the
/// legacy one when the state directory does not exist at all.
pub(crate) fn current_dir(home: &Path) -> PathBuf {
    let current = home.join(".local/state/omarchy/current");
    match fs::symlink_metadata(&current) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            home.join(".config/omarchy/current")
        }
        _ => current,
    }
}

/// Reads the system theme of `home` (see the module's docs for where).
pub fn load_from_home(home: &Path) -> Result<Theme, ThemeLoadError> {
    Theme::from_current_dir(current_dir(home))
}

/// Reads the system theme of `$HOME`. An error when `HOME` is unset or empty,
/// or the theme is missing or invalid.
pub fn load_system_theme() -> Result<Theme, ThemeLoadError> {
    let home = home().ok_or_else(|| ThemeLoadError("HOME is not set".into()))?;
    load_from_home(&home)
}

fn home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .map(PathBuf::from)
}

impl Theme {
    /// The current system theme, or Tokyo Night when it cannot be read.
    pub fn system_or_default() -> Self {
        load_system_theme().unwrap_or_else(|_| Theme::tokyo_night())
    }

    /// Reads an Omarchy `current` directory: `theme/colors.toml` and the
    /// optional `theme.name` ("Omarchy" when missing or empty).
    pub fn from_current_dir(path: impl AsRef<Path>) -> Result<Theme, ThemeLoadError> {
        let path = path.as_ref();
        let file = path.join("theme/colors.toml");
        let text = fs::read_to_string(&file)
            .map_err(|error| ThemeLoadError(format!("{}: {error}", file.display())))?;
        let name = fs::read_to_string(path.join("theme.name")).unwrap_or_default();
        let name = match name.trim() {
            "" => DEFAULT_NAME,
            name => name,
        };
        Theme::from_colors_toml(&text, name)
    }
}

/// The task following the system theme, while there is one.
#[derive(Clone, Default)]
struct Following(Rc<RefCell<Option<Spawned<()>>>>);

/// Stops following the system theme (an explicit theme was applied). Does
/// nothing when not following, or outside an engine.
pub fn stop_following() {
    if let Some(following) = try_global::<Following>()
        && let Some(task) = following.with_untracked(|f| f.0.borrow_mut().take())
    {
        task.cancel();
    }
}

/// Applies the system theme now and follows its changes until an explicit
/// theme is applied ([`Theme::apply`]) or [`stop_following`] is called.
/// Calling it again restarts the one watch. Call it from a component body, a
/// listener or a task. On the web, where there is no Omarchy filesystem, it
/// applies Tokyo Night.
pub fn follow_system() {
    #[cfg(not(target_arch = "wasm32"))]
    follow_home(home());
    #[cfg(target_arch = "wasm32")]
    {
        stop_following();
        Theme::tokyo_night().apply_palette();
    }
}

/// [`follow_system`] for the Omarchy files of `home` (none: Tokyo Night, and
/// nothing to follow).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn follow_home(home: Option<PathBuf>) {
    stop_following();
    let Some(home) = home else {
        Theme::tokyo_night().apply_palette();
        return;
    };
    // The fingerprint is taken before reading, so a change in between is
    // seen by the first poll.
    let fingerprint = watch::Fingerprint::of(&home);
    let theme = load_from_home(&home).unwrap_or_else(|_| Theme::tokyo_night());
    theme.clone().apply_palette();
    let task = watch::follow(home, fingerprint, theme);
    global::<Following>().with_untracked(|f| f.0.borrow_mut().replace(task));
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANSI: &str = parse::tests::ANSI;
    const SEMANTIC_LIGHT: &str = "background = '#ffffff'\nforeground = '#222222'\naccent = '#0055aa'\nred = '#aa0000'\ngreen = '#007700'\nyellow = '#885500'\n";

    fn write_theme(dir: &Path, colors: &str, name: Option<&str>) {
        fs::create_dir_all(dir.join("theme")).unwrap();
        fs::write(dir.join("theme/colors.toml"), colors).unwrap();
        if let Some(name) = name {
            fs::write(dir.join("theme.name"), name).unwrap();
        }
    }

    #[test]
    fn reads_the_state_directory() {
        let home = tempfile::tempdir().unwrap();
        let current = home.path().join(".local/state/omarchy/current");
        write_theme(&current, ANSI, Some("Custom Light\n"));
        let theme = load_from_home(home.path()).unwrap();
        assert_eq!(theme.name.as_ref(), "Custom Light");
        assert_eq!(theme.accent, nsh_core::color::rgb(0x205ea6).to_hsla());
    }

    #[test]
    fn reads_semantic_format_and_defaults_the_name() {
        let home = tempfile::tempdir().unwrap();
        let current = home.path().join(".local/state/omarchy/current");
        write_theme(&current, SEMANTIC_LIGHT, None);
        let theme = load_from_home(home.path()).unwrap();
        assert_eq!(theme.name.as_ref(), "Omarchy");
        assert_eq!(theme.appearance, crate::theme::Appearance::Light);
        fs::write(current.join("theme.name"), "  \n").unwrap();
        assert_eq!(
            load_from_home(home.path()).unwrap().name.as_ref(),
            "Omarchy"
        );
    }

    #[test]
    fn missing_files_are_errors() {
        let home = tempfile::tempdir().unwrap();
        assert!(load_from_home(home.path()).is_err());
        assert!(Theme::from_current_dir("/nonexistent-omarchy-theme-test/current").is_err());
        let current = home.path().join(".local/state/omarchy/current");
        fs::create_dir_all(current.join("theme")).unwrap();
        fs::write(current.join("theme.name"), "Named").unwrap();
        assert!(load_from_home(home.path()).is_err());
    }

    #[test]
    fn invalid_color_is_an_error_and_the_whole_palette_falls_back() {
        let home = tempfile::tempdir().unwrap();
        let current = home.path().join(".local/state/omarchy/current");
        write_theme(&current, &ANSI.replace("#205ea6", "#zzzzzz"), Some("Bad"));
        assert!(load_from_home(home.path()).is_err());
        assert_eq!(
            load_from_home(home.path()).unwrap_or_else(|_| Theme::tokyo_night()),
            Theme::tokyo_night()
        );
    }

    #[test]
    fn legacy_location_is_read_when_the_state_directory_is_absent() {
        let home = tempfile::tempdir().unwrap();
        let legacy = home.path().join(".config/omarchy/current");
        write_theme(&legacy, ANSI, Some("Legacy"));
        assert_eq!(load_from_home(home.path()).unwrap().name.as_ref(), "Legacy");
        // A state directory takes precedence once it exists.
        let current = home.path().join(".local/state/omarchy/current");
        write_theme(&current, ANSI, Some("Current"));
        assert_eq!(
            load_from_home(home.path()).unwrap().name.as_ref(),
            "Current"
        );
        // Removed again, the legacy theme is back.
        fs::remove_dir_all(&current).unwrap();
        assert_eq!(load_from_home(home.path()).unwrap().name.as_ref(), "Legacy");
    }

    #[test]
    fn invalid_current_theme_does_not_revive_the_legacy_one() {
        let home = tempfile::tempdir().unwrap();
        write_theme(
            &home.path().join(".config/omarchy/current"),
            ANSI,
            Some("Legacy"),
        );
        let current = home.path().join(".local/state/omarchy/current");
        write_theme(&current, "broken", Some("Current"));
        assert!(load_from_home(home.path()).is_err());
        // An empty current directory is no theme either.
        fs::remove_dir_all(&current).unwrap();
        fs::create_dir_all(&current).unwrap();
        assert!(load_from_home(home.path()).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn follows_a_theme_directory_symlink() {
        let home = tempfile::tempdir().unwrap();
        let current = home.path().join(".local/state/omarchy/current");
        let target = home.path().join("themes/flexoki");
        fs::create_dir_all(&current).unwrap();
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("colors.toml"), ANSI).unwrap();
        std::os::unix::fs::symlink(&target, current.join("theme")).unwrap();
        assert_eq!(
            load_from_home(home.path()).unwrap().appearance,
            crate::theme::Appearance::Light
        );
    }
}
