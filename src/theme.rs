//! Omarchy palettes and their projection onto NSH's design tokens.
//!
//! An Omarchy theme has a single appearance, so each role is projected with
//! the same value for light and dark. Components never read a palette
//! directly: they call the functions of [`crate::tokens`], which read the
//! current tokens and follow a theme change.

use nsh_base::Theme as BaseTheme;
use nsh_core::color::{Hsla, rgb};
use nsh_core::element::{AdaptiveColor, SharedString, Tokens};
use nsh_core::prelude::*;

/// Whether a palette is drawn on a dark or a light background.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Appearance {
    #[default]
    Dark,
    Light,
}

/// An Omarchy palette: the semantic roles Omarchy's shell draws with.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Display name (`theme.name`, or the built-in palette's name).
    pub name: SharedString,
    pub appearance: Appearance,
    /// The window background.
    pub background: Hsla,
    /// Raised surfaces: panels, popups, cards.
    pub surface: Hsla,
    /// Recessed surfaces: wells, code blocks, tracks.
    pub inset: Hsla,
    /// Body text.
    pub foreground: Hsla,
    /// Secondary text: captions, hints, placeholders.
    pub secondary: Hsla,
    /// Emphasized text: titles, the selected row.
    pub bright: Hsla,
    /// The theme's accent: focus, primary actions, the current item.
    pub accent: Hsla,
    /// Text drawn on an accent fill.
    pub on_accent: Hsla,
    /// Persistent selection fill.
    pub selection: Hsla,
    /// Borders and dividers.
    pub border: Hsla,
    pub danger: Hsla,
    pub warning: Hsla,
    pub success: Hsla,
    /// Series colors for charts.
    pub chart: [Hsla; 5],
    /// The 16 ANSI colors, when the theme provides them (terminal views).
    pub ansi: Option<[Hsla; 16]>,
}

fn c(hex: u32) -> Hsla {
    rgb(hex).to_hsla()
}

impl Theme {
    /// Omarchy's default palette, used whenever the system theme is missing or invalid.
    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".into(),
            appearance: Appearance::Dark,
            background: c(0x1a1b26),
            surface: c(0x24283b),
            inset: c(0x13141c),
            foreground: c(0xa9b1d6),
            secondary: c(0xa9b1d6),
            bright: c(0xc0caf5),
            accent: c(0x7aa2f7),
            on_accent: c(0x13141c),
            selection: c(0x292e42),
            border: c(0x414868),
            danger: c(0xf7768e),
            warning: c(0xe0af68),
            success: c(0x9ece6a),
            chart: [c(0x7aa2f7), c(0xbb9af7), c(0x7dcfff), c(0xe0af68), c(0x9ece6a)],
            ansi: Some([
                c(0x32344a), c(0xf7768e), c(0x9ece6a), c(0xe0af68),
                c(0x7aa2f7), c(0xad8ee6), c(0x449dab), c(0x787c99),
                c(0x444b6a), c(0xff7a93), c(0xb9f27c), c(0xff9e64),
                c(0x7da6ff), c(0xbb9af7), c(0x0db9d7), c(0xacb0d0),
            ]),
        }
    }

    /// A warm light palette for previewing light colors.
    pub fn flexoki_light() -> Self {
        Self {
            name: "Flexoki Light".into(),
            appearance: Appearance::Light,
            background: c(0xfffcf0),
            surface: c(0xf2f0e5),
            inset: c(0xe6e4d9),
            foreground: c(0x100f0f),
            secondary: c(0x575653),
            bright: c(0x100f0f),
            accent: c(0x205ea6),
            on_accent: c(0xfffcf0),
            selection: c(0xdad8ce),
            border: c(0xb7b5ac),
            danger: c(0xaf3029),
            warning: c(0x855b00),
            success: c(0x526600),
            chart: [c(0x205ea6), c(0xa02f6f), c(0x24837b), c(0x855b00), c(0x526600)],
            ansi: None,
        }
    }

    /// The design tokens of this palette: shadcn's roles for `nsh_base` and
    /// Tailwind classes, plus Omarchy's own (`surface`, `inset`, `bright`, ...).
    pub fn to_tokens(&self) -> Tokens {
        let same = |h: Hsla| AdaptiveColor::new(h, h);
        let mut t = BaseTheme::default();
        for (name, color) in self.roles() {
            t.set_color(&name, same(color));
        }
        // Omarchy draws square corners.
        t.set_number("radius", 0.0);
        t.into_tokens()
    }

    /// Every token name this palette sets, with its color: shadcn's roles
    /// (for `nsh_base` and Tailwind's `bg-primary`, ...) and Omarchy's own
    /// (`surface`, `inset`, `bright`, `selection`, `text-secondary`,
    /// `omarchy-accent`, ...).
    pub fn roles(&self) -> Vec<(String, Hsla)> {
        let mut roles: Vec<(String, Hsla)> = [
            ("background", self.background),
            ("foreground", self.foreground),
            ("card", self.surface),
            ("card-foreground", self.foreground),
            ("popover", self.background),
            ("popover-foreground", self.foreground),
            ("primary", self.accent),
            ("primary-foreground", self.on_accent),
            ("secondary", self.surface),
            ("secondary-foreground", self.foreground),
            ("muted", self.inset),
            ("muted-foreground", self.secondary),
            ("accent", self.selection),
            ("accent-foreground", self.bright),
            ("destructive", self.danger),
            ("border", self.border),
            ("input", self.border),
            ("ring", self.accent),
            ("chart-1", self.chart[0]),
            ("chart-2", self.chart[1]),
            ("chart-3", self.chart[2]),
            ("chart-4", self.chart[3]),
            ("chart-5", self.chart[4]),
            ("sidebar", self.background),
            ("sidebar-foreground", self.foreground),
            // Omarchy roles.
            ("surface", self.surface),
            ("inset", self.inset),
            ("bright", self.bright),
            ("selection", self.selection),
            ("on-accent", self.on_accent),
            ("danger", self.danger),
            ("warning", self.warning),
            ("success", self.success),
            ("text-secondary", self.secondary),
            ("omarchy-accent", self.accent),
        ]
        .into_iter()
        .map(|(n, c)| (n.to_owned(), c))
        .collect();
        if let Some(ansi) = self.ansi {
            roles.extend(ansi.iter().enumerate().map(|(i, c)| (format!("ansi-{i}"), *c)));
        }
        roles
    }

    /// A theme file (shadcn's `cssVars`, what `nsh.json` `"theme"` takes)
    /// drawing `light` in the light appearance and `dark` in the dark one.
    /// `themes/omarchy.json` is `theme_json(&flexoki_light(), &tokyo_night())`.
    pub fn theme_json(light: &Theme, dark: &Theme) -> String {
        let section = |theme: &Theme| {
            theme
                .roles()
                .iter()
                .map(|(name, color)| format!("      \"{name}\": \"{}\"", color.to_hex().to_lowercase()))
                .collect::<Vec<_>>()
                .join(",\n")
        };
        format!(
            "{{\n  \"cssVars\": {{\n    \"theme\": {{\n      \"radius\": \"0rem\"\n    }},\n    \"light\": {{\n{}\n    }},\n    \"dark\": {{\n{}\n    }}\n  }}\n}}\n",
            section(light),
            section(dark)
        )
    }

    /// Draws the app with this palette and records it as the active theme.
    /// Call it from a component body, a listener or a task. Stops following
    /// the system theme; [`crate::system_theme::follow_system`] resumes it.
    pub fn apply(self) {
        crate::system_theme::stop_following();
        self.apply_palette();
    }

    /// Applies the palette without changing whether the system theme is followed.
    pub(crate) fn apply_palette(self) {
        set_tokens(self.to_tokens());
        global::<ActiveTheme>().set(ActiveTheme(Some(self)));
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::tokyo_night()
    }
}

/// The palette the app is drawn with (`None` before one is applied).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ActiveTheme(pub Option<Theme>);

/// The active palette (Tokyo Night until another one is applied). Reading it
/// in a binding subscribes to theme changes.
pub fn active_theme() -> Theme {
    global::<ActiveTheme>().with(|t| t.0.clone()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::Theme;

    /// `themes/omarchy.json` is generated from the palettes; set
    /// `UPDATE_THEME_JSON=1` to write it after changing one.
    #[test]
    fn theme_json_is_current() {
        let json = Theme::theme_json(&Theme::flexoki_light(), &Theme::tokyo_night());
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/themes/omarchy.json");
        if std::env::var_os("UPDATE_THEME_JSON").is_some() {
            std::fs::create_dir_all(std::path::Path::new(path).parent().unwrap()).unwrap();
            std::fs::write(path, &json).unwrap();
        }
        let (parsed, ignored) = nsh_base::Theme::from_json_lenient(&json).unwrap();
        assert!(ignored.is_empty(), "{ignored:?}");
        assert_eq!(parsed.number("radius"), Some(0.0));
        assert!(parsed.color("omarchy-accent").is_some());
        assert_eq!(std::fs::read_to_string(path).unwrap(), json, "run with UPDATE_THEME_JSON=1");
    }
}
