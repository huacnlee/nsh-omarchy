//! Following the system theme by polling.
//!
//! Every [`POLL`] a blocking thread takes a cheap fingerprint of the files the
//! theme is read from: which `current` directory applies, where each link
//! resolves, and each file's size, modification time and identity. Only when
//! it changes is the theme read again, and only a changed palette is applied.
//! Fingerprinting resolved paths catches the replacement of the
//! `current/theme` symlink as well as edits and atomic saves of the files.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use nsh_core::prelude::*;

use super::{current_dir, load_from_home};
use crate::theme::Theme;

/// How often the files are looked at.
pub(super) const POLL: Duration = Duration::from_secs(1);

/// What the theme was read from, cheaply: when it is equal, the theme is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Fingerprint {
    dir: PathBuf,
    entries: Vec<Entry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Entry {
    resolved: Option<PathBuf>,
    stamp: Option<Stamp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stamp {
    len: u64,
    modified: Option<SystemTime>,
    identity: (u64, u64),
}

impl Fingerprint {
    /// The fingerprint of `home`'s theme files now (blocking IO).
    pub(crate) fn of(home: &Path) -> Self {
        let dir = current_dir(home);
        let entries = [
            dir.clone(),
            dir.join("theme"),
            dir.join("theme/colors.toml"),
            dir.join("theme.name"),
        ]
        .iter()
        .map(|path| Entry {
            resolved: fs::canonicalize(path).ok(),
            stamp: fs::metadata(path).ok().map(|metadata| Stamp {
                len: metadata.len(),
                modified: metadata.modified().ok(),
                identity: identity(&metadata),
            }),
        })
        .collect();
        Self { dir, entries }
    }
}

/// The file's device and inode, so a file replaced by a rename is a change
/// even with the same size and time.
#[cfg(unix)]
fn identity(metadata: &fs::Metadata) -> (u64, u64) {
    use std::os::unix::fs::MetadataExt;
    (metadata.dev(), metadata.ino())
}

#[cfg(not(unix))]
fn identity(metadata: &fs::Metadata) -> (u64, u64) {
    let created = metadata
        .created()
        .ok()
        .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
        .unwrap_or_default();
    (created.as_secs(), u64::from(created.subsec_nanos()))
}

/// Starts the task that follows `home`'s theme, from what was last read and
/// applied. It belongs to no component: it runs until it is cancelled
/// ([`super::stop_following`]).
pub(super) fn follow(
    home: PathBuf,
    mut fingerprint: Fingerprint,
    mut applied: Theme,
) -> Spawned<()> {
    Owner::detached().run(move || {
        spawn(async move {
            loop {
                timer(POLL).await;
                let home = home.clone();
                let last = fingerprint.clone();
                let polled = blocking(move || {
                    let next = Fingerprint::of(&home);
                    let theme = (next != last).then(|| {
                        load_from_home(&home).unwrap_or_else(|error| {
                            log::warn!("Omarchy theme unavailable, using Tokyo Night: {error}");
                            Theme::tokyo_night()
                        })
                    });
                    (next, theme)
                })
                .await;
                let Ok((next, theme)) = polled else { continue };
                fingerprint = next;
                if let Some(theme) = theme
                    && theme != applied
                {
                    applied = theme.clone();
                    theme.apply_palette();
                }
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLORS: &str = super::super::parse::tests::ANSI;

    #[test]
    fn edits_of_the_same_size_change_the_fingerprint() {
        let home = tempfile::tempdir().unwrap();
        let theme = home.path().join(".local/state/omarchy/current/theme");
        fs::create_dir_all(&theme).unwrap();
        fs::write(theme.join("colors.toml"), COLORS).unwrap();
        let before = Fingerprint::of(home.path());
        assert_eq!(before, Fingerprint::of(home.path()));
        std::thread::sleep(Duration::from_millis(20));
        fs::write(
            theme.join("colors.toml"),
            COLORS.replace("#205ea6", "#7aa2f7"),
        )
        .unwrap();
        assert_ne!(before, Fingerprint::of(home.path()));
    }

    #[test]
    fn a_new_state_directory_changes_the_fingerprint() {
        let home = tempfile::tempdir().unwrap();
        let legacy = home.path().join(".config/omarchy/current/theme");
        fs::create_dir_all(&legacy).unwrap();
        fs::write(legacy.join("colors.toml"), COLORS).unwrap();
        let before = Fingerprint::of(home.path());
        fs::create_dir_all(home.path().join(".local/state/omarchy/current")).unwrap();
        assert_ne!(before, Fingerprint::of(home.path()));
    }

    #[cfg(unix)]
    #[test]
    fn replacing_the_theme_symlink_changes_the_fingerprint() {
        let home = tempfile::tempdir().unwrap();
        let current = home.path().join(".local/state/omarchy/current");
        fs::create_dir_all(&current).unwrap();
        for name in ["a", "b"] {
            let dir = home.path().join(name);
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("colors.toml"), COLORS).unwrap();
        }
        std::os::unix::fs::symlink(home.path().join("a"), current.join("theme")).unwrap();
        let before = Fingerprint::of(home.path());
        // Omarchy swaps the link atomically: a new link renamed over the old.
        std::os::unix::fs::symlink(home.path().join("b"), current.join("next")).unwrap();
        fs::rename(current.join("next"), current.join("theme")).unwrap();
        assert_ne!(before, Fingerprint::of(home.path()));
    }
}
