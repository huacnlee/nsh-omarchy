//! The design tokens every component draws with. Each function reads the
//! current theme, so call it inside a binding closure to follow theme
//! changes: `.bg(move || tokens::surface())`.
//!
//! Omarchy's look: square corners, a single accent, foreground-tinted fills
//! for interaction states, a visible border for focus and a stronger fill
//! (no marker, no accent) for persistent selection.

use nsh_base::theme;
use nsh_core::element::AdaptiveColor;
use nsh_core::prelude::*;

fn named(name: &str, fallback: impl FnOnce() -> AdaptiveColor) -> AdaptiveColor {
    theme().color(name).unwrap_or_else(fallback)
}

// ---- Surfaces -------------------------------------------------------------

/// The window background.
pub fn background() -> AdaptiveColor {
    theme().background
}
/// Raised surfaces: panels, popups, cards.
pub fn surface() -> AdaptiveColor {
    named("surface", || theme().card)
}
/// Recessed surfaces: wells, tracks, code.
pub fn inset() -> AdaptiveColor {
    named("inset", || theme().muted)
}
/// Popup and overlay background.
pub fn popover() -> AdaptiveColor {
    theme().popover
}

// ---- Text -----------------------------------------------------------------

/// Body text.
pub fn foreground() -> AdaptiveColor {
    theme().foreground
}
/// Secondary text: captions, hints, placeholders.
pub fn secondary() -> AdaptiveColor {
    named("text-secondary", || theme().muted_foreground)
}
/// Emphasized text: titles, selected rows.
pub fn bright() -> AdaptiveColor {
    named("bright", || theme().accent_foreground)
}
/// Disabled text.
pub fn disabled() -> AdaptiveColor {
    foreground().opacity(0.38)
}

// ---- Accent and status ----------------------------------------------------

/// The theme's accent.
pub fn accent() -> AdaptiveColor {
    named("omarchy-accent", || theme().primary)
}
/// Text drawn on an accent fill.
pub fn on_accent() -> AdaptiveColor {
    named("on-accent", || theme().primary_foreground)
}
/// Persistent selection fill.
pub fn selection() -> AdaptiveColor {
    named("selection", || theme().accent)
}
pub fn danger() -> AdaptiveColor {
    named("danger", || theme().destructive)
}
pub fn warning() -> AdaptiveColor {
    named("warning", || theme().chart_4)
}
pub fn success() -> AdaptiveColor {
    named("success", || theme().chart_5)
}
/// Chart series color `index` (0..5, wraps).
pub fn chart(index: usize) -> AdaptiveColor {
    let t = theme();
    match index % 5 {
        0 => t.chart_1,
        1 => t.chart_2,
        2 => t.chart_3,
        3 => t.chart_4,
        _ => t.chart_5,
    }
}
/// ANSI color `index` (0..16), falling back to the chart colors.
pub fn ansi(index: usize) -> AdaptiveColor {
    named(&format!("ansi-{}", index % 16), || chart(index))
}

// ---- Lines ----------------------------------------------------------------

/// Borders of surfaces.
pub fn border() -> AdaptiveColor {
    theme().border
}
/// Dividers inside a surface.
pub fn divider() -> AdaptiveColor {
    foreground().opacity(0.12)
}
/// Borders of controls (inputs, checkboxes, outline buttons).
pub fn control_border() -> AdaptiveColor {
    foreground().opacity(0.4)
}
/// The visible focus border.
pub fn focus_border() -> AdaptiveColor {
    accent()
}

// ---- Interaction fills (foreground-tinted) --------------------------------

/// Resting fill of a control.
pub fn normal_fill() -> AdaptiveColor {
    foreground().opacity(0.04)
}
pub fn hover_fill() -> AdaptiveColor {
    foreground().opacity(0.08)
}
pub fn pressed_fill() -> AdaptiveColor {
    foreground().opacity(0.22)
}
/// Persistent selection: the fill alone, border width 0, no accent marker.
pub fn selected_fill() -> AdaptiveColor {
    foreground().opacity(0.18)
}
/// Text selection highlight.
pub fn text_selection() -> AdaptiveColor {
    foreground().opacity(0.35)
}

/// The backdrop behind modal surfaces (dialogs, sheets).
pub fn scrim() -> AdaptiveColor {
    background().opacity(0.72)
}

// ---- Geometry -------------------------------------------------------------

/// Corner radius: Omarchy is square.
pub const RADIUS: f32 = 0.0;
/// Border width of controls and surfaces.
pub const BORDER: f32 = 1.0;
/// Focus border width.
pub const FOCUS_BORDER: f32 = 1.0;

/// Control heights.
pub const CONTROL_SM: f32 = 24.0;
pub const CONTROL_MD: f32 = 28.0;
pub const CONTROL_LG: f32 = 32.0;

/// Spacing scale (points).
pub const SPACE_1: f32 = 4.0;
pub const SPACE_2: f32 = 8.0;
pub const SPACE_3: f32 = 12.0;
pub const SPACE_4: f32 = 16.0;
pub const SPACE_6: f32 = 24.0;

// ---- Typography -----------------------------------------------------------

/// Type scale (points), after Omarchy's shell: compact desktop text.
pub const TEXT_XS: f32 = 10.0;
pub const TEXT_SM: f32 = 11.0;
pub const TEXT_MD: f32 = 12.0;
pub const TEXT_LG: f32 = 14.0;
pub const TEXT_XL: f32 = 16.0;
pub const TEXT_2XL: f32 = 20.0;

/// The monospaced family, used for keycaps, code and tabular data.
pub fn font_mono() -> SharedString {
    theme()
        .font_mono
        .clone()
        .unwrap_or_else(|| "monospace".into())
}

// ---- Motion ---------------------------------------------------------------

/// Hover and color washes.
pub const DURATION_FAST: f32 = 90.0;
/// Popups, selection slides, thumb travel.
pub const DURATION_BASE: f32 = 140.0;
/// Sheets and larger surfaces.
pub const DURATION_SLOW: f32 = 220.0;
/// Press scale of buttons.
pub const PRESS_SCALE: f32 = 0.97;
/// Distance a popup travels as it enters.
pub const ENTER_DISTANCE: f32 = 4.0;

// ---- Theme-role classes --------------------------------------------------

/// The interaction tokens above as Tailwind classes, for the parts of
/// `nsh_base` components whose states only a class reaches (list rows, part
/// borders, placeholders). Each names the same theme role as its function,
/// so it follows a theme change.
pub mod classes {
    /// `tokens::hover_fill()` while hovered.
    pub const HOVER: &str = "hover:bg-foreground/8";
    /// `tokens::pressed_fill()` while pressed.
    pub const PRESSED: &str = "active:bg-foreground/22";
    /// `tokens::hover_fill()` on the highlighted row of a list (`focus:`).
    pub const HIGHLIGHT: &str = "focus:bg-foreground/8";
    /// `tokens::hover_fill()` while the keyboard focus shows
    /// (`focus-visible:`): the keys' row of a tree.
    pub const FOCUS_FILL: &str = "focus-visible:bg-foreground/8";
    /// A border reserved but not drawn, as a class so a state class (the
    /// focus border) can colour it.
    pub const CLEAR_BORDER: &str = "border-transparent";
    /// `tokens::focus_border()` while the keyboard focus shows.
    pub const FOCUS_VISIBLE_BORDER: &str = "focus-visible:border-omarchy-accent";
    /// `tokens::control_border()`, as a class so a `focus:` class can
    /// replace it (a part's typed style is over its classes).
    pub const CONTROL_BORDER: &str = "border-foreground/40";
    /// `tokens::focus_border()` on the part that takes the next keystroke.
    pub const FOCUS_BORDER: &str = "focus:border-omarchy-accent";
    /// `tokens::disabled()`: the foreground at 38%.
    pub const DISABLED: &str = "disabled:opacity-40";
    /// `tokens::secondary()` for a part's placeholder.
    pub const PLACEHOLDER: &str = "placeholder:text-text-secondary";
}
