//! An app follows the system theme: switching to explicit themes and back,
//! and picking up edits, broken files and symlink swaps while it follows.
//!
//! `HOME` is set once, for this test binary only, to a temporary Omarchy
//! installation, so every test here shares it and the one test runs in order.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_omarchy as om;
use nsh_testing::Headless;

/// Shows the active theme's name and mode, the last name `on_theme_change`
/// heard, and buttons for each way to pick a theme.
#[component]
fn Themes() -> impl IntoElement {
    om::init();
    let heard = signal(String::new());
    let _stop = om::on_theme_change(move |theme| heard.set(theme.name().to_owned()));
    view()
        .child(text(move || om::active_theme().name().to_owned()))
        .child(text(move || format!("mode {:?}", om::active_theme().mode())))
        .child(text(move || format!("heard {}", heard.get())))
        .child(om::button("Light").on_press(|| om::Theme::flexoki_light().apply()))
        .child(om::button("Dark").on_press(|| om::Theme::tokyo_night().apply()))
        .child(om::button("System").on_press(om::follow_system))
}

const COLORS: &str = "\
background = '#1e1e2e'
foreground = '#cdd6f4'
accent = '#89b4fa'
color1 = '#f38ba8'
color2 = '#a6e3a1'
color3 = '#f9e2af'
";

/// Longer than a few polls of the watcher (one a second).
const FOLLOW: Duration = Duration::from_secs(10);

struct Install {
    _home: tempfile::TempDir,
    current: PathBuf,
    themes: PathBuf,
}

fn install() -> &'static Install {
    static INSTALL: OnceLock<Install> = OnceLock::new();
    INSTALL.get_or_init(|| {
        let home = tempfile::tempdir().unwrap();
        let current = home.path().join(".local/state/omarchy/current");
        let themes = home.path().join("themes");
        fs::create_dir_all(&current).unwrap();
        for (name, colors) in [("good", COLORS), ("broken", "not a palette")] {
            fs::create_dir_all(themes.join(name)).unwrap();
            fs::write(themes.join(name).join("colors.toml"), colors).unwrap();
        }
        link_theme(&current, &themes.join("good"));
        fs::write(current.join("theme.name"), "System One").unwrap();
        // SAFETY: set once, before any engine of this binary starts a thread
        // that could read the environment.
        unsafe { std::env::set_var("HOME", home.path()) };
        Install {
            _home: home,
            current,
            themes,
        }
    })
}

/// Points `current/theme` at `target`, swapping the link atomically as
/// Omarchy does.
fn link_theme(current: &Path, target: &Path) {
    #[cfg(unix)]
    {
        let next = current.join("theme.next");
        let _ = fs::remove_file(&next);
        std::os::unix::fs::symlink(target, &next).unwrap();
        fs::rename(&next, current.join("theme")).unwrap();
    }
    #[cfg(not(unix))]
    {
        let _ = fs::remove_dir_all(current.join("theme"));
        fs::create_dir_all(current.join("theme")).unwrap();
        fs::copy(
            target.join("colors.toml"),
            current.join("theme/colors.toml"),
        )
        .unwrap();
    }
}

fn start() -> Headless {
    install();
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        ..EngineConfig::new(Root::of::<Themes>())
    })
    .unwrap()
}

#[test]
fn an_app_follows_the_system_theme() -> Result<(), String> {
    let install = install();
    let mut app = start();
    app.wait_for("text:System One")?;

    // Explicit themes stop following; System resumes.
    app.press("Light")?;
    app.wait_for("text:Flexoki Light")?;
    app.wait_for("text:mode Light")?;
    app.wait_for("text:heard Flexoki Light")?;
    app.press("System")?;
    app.wait_for("text:System One")?;
    // Pressing it again keeps one watch.
    app.press("System")?;
    app.press("System")?;

    // An edited name is picked up.
    fs::write(install.current.join("theme.name"), "System Two").unwrap();
    app.wait_for_within("text:System Two", FOLLOW)?;

    // A broken palette falls back to Tokyo Night, and recovers.
    let colors = install.themes.join("good/colors.toml");
    fs::write(&colors, "broken").unwrap();
    app.wait_for_within("text:Tokyo Night", FOLLOW)?;
    fs::write(&colors, COLORS).unwrap();
    app.wait_for_within("text:System Two", FOLLOW)?;

    // Replacing the `current/theme` symlink is followed both ways.
    link_theme(&install.current, &install.themes.join("broken"));
    app.wait_for_within("text:Tokyo Night", FOLLOW)?;
    link_theme(&install.current, &install.themes.join("good"));
    app.wait_for_within("text:System Two", FOLLOW)?;

    // An explicit theme is not overridden by later system changes.
    app.press("Dark")?;
    app.wait_for("text:Tokyo Night")?;
    fs::write(install.current.join("theme.name"), "System Three").unwrap();
    app.wait(Duration::from_millis(2500))?;
    assert!(app.shows("Tokyo Night"));
    assert!(!app.shows("System Three"));
    app.press("System")?;
    app.wait_for("text:System Three")?;
    Ok(())
}
