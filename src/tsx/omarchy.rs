//! `Omarchy`, the theme module TSX calls: follow the Omarchy system theme,
//! apply a built-in palette, read the active one and hear its changes.
//!
//! Rust owns the theme: TSX asks for changes through the functions and
//! mirrors the active theme in a signal it sets from [`Omarchy::on_change`].
//! The palettes reach TSX as tokens, so `bg-primary`, `text-foreground` and
//! `token("surface")` follow every change with no mirror at all.

use std::cell::{Cell, RefCell};

use nsh_core::prelude::*;

use crate::theme::{Appearance, Theme};

/// Whether a theme draws on a dark or a light background.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PropEnum)]
pub enum ThemeAppearance {
    /// Light text on a dark background.
    Dark,
    /// Dark text on a light background.
    Light,
}

impl From<Appearance> for ThemeAppearance {
    fn from(appearance: Appearance) -> Self {
        match appearance {
            Appearance::Dark => Self::Dark,
            Appearance::Light => Self::Light,
        }
    }
}

/// A palette built into nsh-omarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PropEnum)]
pub enum BuiltInTheme {
    /// Tokyo Night, Omarchy's default dark palette.
    #[prop(rename = "tokyo-night")]
    TokyoNight,
    /// Flexoki Light, a warm light palette.
    #[prop(rename = "flexoki-light")]
    FlexokiLight,
}

impl BuiltInTheme {
    /// The palette.
    pub fn theme(self) -> Theme {
        match self {
            Self::TokyoNight => Theme::tokyo_night(),
            Self::FlexokiLight => Theme::flexoki_light(),
        }
    }
}

/// The theme the app is drawn with.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct ThemeInfo {
    /// Its display name: the system theme's `theme.name`, or the built-in
    /// palette's ("Tokyo Night").
    pub name: String,
    /// Whether it is dark or light.
    pub appearance: ThemeAppearance,
}

impl From<&Theme> for ThemeInfo {
    fn from(theme: &Theme) -> Self {
        Self {
            name: theme.name.to_string(),
            appearance: theme.appearance.into(),
        }
    }
}

thread_local! {
    /// The theme last applied on this (the engine's) thread.
    static CURRENT: RefCell<Option<ThemeInfo>> = const { RefCell::new(None) };
    /// The listeners of [`Omarchy::on_change`], by id.
    static LISTENERS: RefCell<Vec<(u64, Callback<ThemeInfo>)>> = const { RefCell::new(Vec::new()) };
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
}

/// Records `theme` as the one drawn and tells the listeners, when it is
/// another one. Called by [`Theme::apply_palette`], on the engine's thread.
pub(crate) fn theme_applied(theme: &Theme) {
    let info = ThemeInfo::from(theme);
    let changed = CURRENT.with_borrow_mut(|current| {
        let changed = current.as_ref() != Some(&info);
        *current = Some(info.clone());
        changed
    });
    if changed {
        let listeners: Vec<_> =
            LISTENERS.with_borrow(|l| l.iter().map(|(_, c)| c.clone()).collect());
        for listener in listeners {
            listener.post(info.clone());
        }
    }
}

/// The Omarchy theme: follow the system's, apply a built-in palette, read
/// the active one and hear its changes. Every color of the app's tokens
/// (`bg-primary`, `text-foreground`, `token("surface")`) follows it.
#[module]
pub struct Omarchy;

#[module]
impl Omarchy {
    /// Applies the Omarchy system theme now and follows its changes until
    /// `applyTheme` or `stopFollowing` is called. Where there is no Omarchy
    /// theme (another OS, the web) it applies Tokyo Night. Call it once,
    /// when the app starts.
    pub fn follow_system(_cx: &mut ModuleCx) {
        crate::system_theme::follow_system();
    }

    /// Stops following the system theme, keeping the palette drawn now.
    pub fn stop_following(_cx: &mut ModuleCx) {
        crate::system_theme::stop_following();
    }

    /// Draws the app with the built-in palette `name`, and stops following
    /// the system theme.
    pub fn apply_theme(name: BuiltInTheme, _cx: &mut ModuleCx) {
        name.theme().apply();
    }

    /// The theme drawn now; `null` until `followSystem` or `applyTheme`
    /// applied one (the app then draws with its `nsh.json` theme).
    pub fn current() -> Option<ThemeInfo> {
        CURRENT.with_borrow(Clone::clone)
    }

    /// Calls `listener` with the theme after each change of it: an applied
    /// palette, or the system theme changing while followed. Returns the
    /// function that stops listening; a listener added in a component stops
    /// with it.
    pub fn on_change(#[args(theme)] listener: Callback<ThemeInfo>) -> Stop {
        let id = NEXT_ID.with(|next| {
            let id = next.get();
            next.set(id + 1);
            id
        });
        LISTENERS.with_borrow_mut(|l| l.push((id, listener)));
        Stop::new(move || {
            // The thread may be ending (the engine stopping): then there
            // is nothing left to remove.
            let _ = LISTENERS.try_with(|l| l.borrow_mut().retain(|(i, _)| *i != id));
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_current_theme_follows_what_is_applied() {
        theme_applied(&Theme::flexoki_light());
        assert_eq!(
            Omarchy::current(),
            Some(ThemeInfo {
                name: "Flexoki Light".into(),
                appearance: ThemeAppearance::Light,
            })
        );
        theme_applied(&BuiltInTheme::TokyoNight.theme());
        assert_eq!(
            Omarchy::current().map(|t| t.name),
            Some("Tokyo Night".into())
        );
    }
}
