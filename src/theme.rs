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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, PropEnum)]
#[non_exhaustive]
pub enum ThemeMode {
    /// Light text on a dark background.
    #[default]
    Dark,
    /// Dark text on a light background.
    Light,
}

/// An Omarchy palette: the semantic roles Omarchy's shell draws with.
///
/// Made by [`Theme::tokyo_night`], [`Theme::flexoki_light`],
/// [`Theme::load_system`] or [`Theme::from_colors_toml`]; drawn with
/// [`Theme::apply`]. Components read the colors through [`crate::tokens`].
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Display name (`theme.name`, or the built-in palette's name).
    pub(crate) name: SharedString,
    pub(crate) mode: ThemeMode,
    /// The window background.
    pub(crate) background: Hsla,
    /// Raised surfaces: panels, popups, cards.
    pub(crate) surface: Hsla,
    /// Recessed surfaces: wells, code blocks, tracks.
    pub(crate) inset: Hsla,
    /// Body text.
    pub(crate) foreground: Hsla,
    /// Secondary text: captions, hints, placeholders.
    pub(crate) secondary: Hsla,
    /// Emphasized text: titles, the selected row.
    pub(crate) bright: Hsla,
    /// The theme's accent: focus, primary actions, the current item.
    pub(crate) accent: Hsla,
    /// Text drawn on an accent fill.
    pub(crate) on_accent: Hsla,
    /// Persistent selection fill.
    pub(crate) selection: Hsla,
    /// Borders and dividers.
    pub(crate) border: Hsla,
    /// Destructive actions and errors.
    pub(crate) danger: Hsla,
    /// Warnings.
    pub(crate) warning: Hsla,
    /// Success.
    pub(crate) success: Hsla,
    /// Series colors for charts.
    pub(crate) chart: [Hsla; 5],
    /// The 16 ANSI colors, when the theme provides them (terminal views).
    pub(crate) ansi: Option<[Hsla; 16]>,
}

fn c(hex: u32) -> Hsla {
    rgb(hex).to_hsla()
}

impl Theme {
    /// Its display name: the system theme's `theme.name`, or the built-in
    /// palette's ("Tokyo Night").
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Dark or light.
    pub fn mode(&self) -> ThemeMode {
        self.mode
    }

    /// The window background.
    pub fn background(&self) -> Hsla {
        self.background
    }

    /// Raised surfaces: panels, popups, cards.
    pub fn surface(&self) -> Hsla {
        self.surface
    }

    /// Recessed surfaces: wells, code blocks, tracks.
    pub fn inset(&self) -> Hsla {
        self.inset
    }

    /// Body text.
    pub fn foreground(&self) -> Hsla {
        self.foreground
    }

    /// Secondary text: captions, hints, placeholders.
    pub fn secondary(&self) -> Hsla {
        self.secondary
    }

    /// Emphasized text: titles, the selected row.
    pub fn bright(&self) -> Hsla {
        self.bright
    }

    /// The accent: focus, primary actions, the current item.
    pub fn accent(&self) -> Hsla {
        self.accent
    }

    /// Text drawn on an accent fill.
    pub fn on_accent(&self) -> Hsla {
        self.on_accent
    }

    /// The persistent selection fill.
    pub fn selection(&self) -> Hsla {
        self.selection
    }

    /// Borders and dividers.
    pub fn border(&self) -> Hsla {
        self.border
    }

    /// Destructive actions and errors.
    pub fn danger(&self) -> Hsla {
        self.danger
    }

    /// Warnings.
    pub fn warning(&self) -> Hsla {
        self.warning
    }

    /// Success.
    pub fn success(&self) -> Hsla {
        self.success
    }

    /// The five chart series colors.
    pub fn chart(&self) -> [Hsla; 5] {
        self.chart
    }

    /// The 16 ANSI colors, when the theme provides them.
    pub fn ansi(&self) -> Option<[Hsla; 16]> {
        self.ansi
    }

    /// Omarchy's default palette, used whenever the system theme is missing or invalid.
    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".into(),
            mode: ThemeMode::Dark,
            background: c(0x1a1b26),
            surface: c(0x24283b),
            inset: c(0x13141c),
            foreground: c(0xa9b1d6),
            // The background mixed 75% toward the foreground, as a system theme derives it.
            secondary: c(0x858baa),
            bright: c(0xc0caf5),
            accent: c(0x7aa2f7),
            on_accent: c(0x13141c),
            selection: c(0x292e42),
            border: c(0x414868),
            danger: c(0xf7768e),
            warning: c(0xe0af68),
            success: c(0x9ece6a),
            chart: [
                c(0x7aa2f7),
                c(0xbb9af7),
                c(0x7dcfff),
                c(0xe0af68),
                c(0x9ece6a),
            ],
            ansi: Some([
                c(0x32344a),
                c(0xf7768e),
                c(0x9ece6a),
                c(0xe0af68),
                c(0x7aa2f7),
                c(0xad8ee6),
                c(0x449dab),
                c(0x787c99),
                c(0x444b6a),
                c(0xff7a93),
                c(0xb9f27c),
                c(0xff9e64),
                c(0x7da6ff),
                c(0xbb9af7),
                c(0x0db9d7),
                c(0xacb0d0),
            ]),
        }
    }

    /// A warm light palette for previewing light colors.
    pub fn flexoki_light() -> Self {
        Self {
            name: "Flexoki Light".into(),
            mode: ThemeMode::Light,
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
            chart: [
                c(0x205ea6),
                c(0xa02f6f),
                c(0x24837b),
                c(0x855b00),
                c(0x526600),
            ],
            ansi: None,
        }
    }

    /// The design tokens of this palette: shadcn's roles for `nsh_base` and
    /// Tailwind classes, plus Omarchy's own (`surface`, `inset`, `bright`, ...).
    pub(crate) fn to_tokens(&self) -> Tokens {
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
    pub(crate) fn roles(&self) -> Vec<(String, Hsla)> {
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
            roles.extend(
                ansi.iter()
                    .enumerate()
                    .map(|(i, c)| (format!("ansi-{i}"), *c)),
            );
        }
        roles
    }

    /// A theme file (shadcn's `cssVars`, what `nsh.json` `"theme"` takes)
    /// drawing `light` in the light appearance and `dark` in the dark one.
    /// `themes/omarchy.json` is `theme_json(&flexoki_light(), &tokyo_night())`.
    #[cfg(test)]
    pub(crate) fn theme_json(light: &Theme, dark: &Theme) -> String {
        let section = |theme: &Theme| {
            theme
                .roles()
                .iter()
                .map(|(name, color)| {
                    format!("      \"{name}\": \"{}\"", color.to_hex().to_lowercase())
                })
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
    /// the system theme; [`follow_system`](crate::follow_system) resumes it.
    pub fn apply(self) {
        crate::system_theme::stop_following();
        self.apply_palette();
    }

    /// Applies the palette without changing whether the system theme is followed.
    pub(crate) fn apply_palette(self) {
        // The window takes the palette's appearance, so what adapts to it
        // (native controls, scroll bars, highlighted code) matches.
        nsh_core::prelude::set_color_scheme(match self.mode {
            ThemeMode::Dark => nsh_core::platform::Appearance::Dark,
            ThemeMode::Light => nsh_core::platform::Appearance::Light,
        });
        set_tokens(self.to_tokens());
        crate::ui::omarchy::theme_applied(&self);
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
pub(crate) struct ActiveTheme(pub(crate) Option<Theme>);

/// The active palette (Tokyo Night until another one is applied). Reading it
/// in a binding subscribes to theme changes.
pub fn active_theme() -> Theme {
    global::<ActiveTheme>()
        .with(|t| t.0.clone())
        .unwrap_or_default()
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
        assert_eq!(
            std::fs::read_to_string(path).unwrap(),
            json,
            "run with UPDATE_THEME_JSON=1"
        );
    }
}
