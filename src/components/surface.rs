//! Square surfaces and the static pieces drawn on them: panels and cards,
//! separators, badges, keycaps and empty states.
//!
//! Each constructor returns a plain `View` (or `Text`), so callers keep the
//! builder API: add children with `.child(..)`, adjust size and spacing.

use nsh_core::element::AdaptiveColor;
use nsh_core::prelude::*;

use super::icon::{IconName, icon};
use crate::tokens;

/// The size of the icon above an empty state's title.
const EMPTY_ICON_SIZE: f32 = 24.0;
/// The height of a keycap and a badge: a line of `TEXT_XS` text with a little air.
const CHIP_HEIGHT: f32 = 18.0;

// ---- Panels and cards -----------------------------------------------------

/// A raised square surface with a 1px border and padding, laying out its
/// children in a column. The base of [`panel`].
pub fn card() -> View {
    view()
        .flex_col()
        .min_w(px(0.0))
        .gap(px(tokens::SPACE_3))
        .p(px(tokens::SPACE_4))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::surface())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
}

/// A [`card`] with a bold title at its top; its children follow the title.
pub fn panel(title: impl Into<SharedString>) -> View {
    card().child(
        text(title.into())
            .role(AccessibilityRole::Header)
            .text_size(px(tokens::TEXT_LG))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(move || tokens::bright()),
    )
}

/// A recessed well (code, previews, tracks) inside a surface.
pub fn inset_well() -> View {
    view()
        .p(px(tokens::SPACE_3))
        .border_1()
        .border_color(move || tokens::divider())
        .bg(move || tokens::inset())
}

// ---- Separators -----------------------------------------------------------

/// A 1px horizontal rule across its container, in the divider color.
pub fn separator() -> View {
    view()
        .w_full()
        .h(px(tokens::BORDER))
        .flex_shrink_0()
        .bg(move || tokens::divider())
        .accessibility_elements_hidden(true)
}

/// A 1px vertical rule for toolbars and rows, as tall as a small control
/// (override with `.h(..)` or `.self_stretch()`).
pub fn vertical_separator() -> View {
    view()
        .w(px(tokens::BORDER))
        .h(px(tokens::CONTROL_SM))
        .flex_shrink_0()
        .bg(move || tokens::divider())
        .accessibility_elements_hidden(true)
}

// ---- Badges ---------------------------------------------------------------

/// The color a [`badge`] carries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BadgeVariant {
    /// Neutral: secondary text in a control border.
    #[default]
    Default,
    /// The theme's accent.
    Accent,
    Success,
    Warning,
    Danger,
}

impl BadgeVariant {
    /// Every variant, in order.
    pub const ALL: [BadgeVariant; 5] = [
        BadgeVariant::Default,
        BadgeVariant::Accent,
        BadgeVariant::Success,
        BadgeVariant::Warning,
        BadgeVariant::Danger,
    ];

    /// The text color of the variant (read it in a binding closure).
    pub fn color(self) -> AdaptiveColor {
        match self {
            BadgeVariant::Default => tokens::secondary(),
            BadgeVariant::Accent => tokens::accent(),
            BadgeVariant::Success => tokens::success(),
            BadgeVariant::Warning => tokens::warning(),
            BadgeVariant::Danger => tokens::danger(),
        }
    }

    fn border(self) -> AdaptiveColor {
        match self {
            BadgeVariant::Default => tokens::control_border(),
            other => other.color(),
        }
    }

    fn fill(self) -> AdaptiveColor {
        match self {
            BadgeVariant::Default => tokens::normal_fill(),
            other => other.color().opacity(0.1),
        }
    }
}

/// A small outlined status label: text and border in the variant's color
/// over a faint wash of it.
pub fn badge(label: impl Into<SharedString>, variant: BadgeVariant) -> View {
    view()
        .flex_row()
        .flex_shrink_0()
        .self_start()
        .items_center()
        .h(px(CHIP_HEIGHT))
        .px(px(tokens::SPACE_1 * 1.5))
        .border_1()
        .border_color(move || variant.border())
        .bg(move || variant.fill())
        .child(
            text(label.into())
                .text_size(px(tokens::TEXT_XS))
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || variant.color()),
        )
}

// ---- Keycaps --------------------------------------------------------------

/// One key as printed on a keyboard (`ctrl`, `⌘K`, `esc`): regular-weight
/// monospace on an inset square with a border. Screen readers read it as a key.
pub fn keycap(key: impl Into<SharedString>) -> View {
    view()
        .flex_row()
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .h(px(CHIP_HEIGHT))
        .min_w(px(CHIP_HEIGHT))
        .px(px(tokens::SPACE_1))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::inset())
        .role(AccessibilityRole::KeyboardKey)
        .child(
            text(key.into())
                .font_family(move || tokens::font_mono())
                .text_size(px(tokens::TEXT_XS))
                .text_color(move || tokens::foreground()),
        )
}

/// A row of keycaps, one per keystroke of a shortcut: `keycaps(["super", "space"])`.
pub fn keycaps<K: Into<SharedString>>(keys: impl IntoIterator<Item = K>) -> View {
    view()
        .flex_row()
        .flex_shrink_0()
        .items_center()
        .gap(px(tokens::SPACE_1))
        .children(keys.into_iter().map(|key| keycap(key).into_element()))
}

// ---- Empty state ----------------------------------------------------------

/// What a view shows when it has nothing yet: an icon, a title and a line of
/// description, centered. Add an action with `.child(button(..))`.
pub fn empty_state(
    icon_name: IconName,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
) -> View {
    view()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(tokens::SPACE_2))
        .p(px(tokens::SPACE_6))
        .child(
            icon(icon_name)
                .size(px(EMPTY_ICON_SIZE))
                .mb(px(tokens::SPACE_1))
                .text_color(move || tokens::secondary()),
        )
        .child(
            text(title.into())
                .role(AccessibilityRole::Header)
                .text_size(px(tokens::TEXT_LG))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::bright()),
        )
        .child(
            text(description.into())
                .text_size(px(tokens::TEXT_MD))
                .text_center()
                .text_color(move || tokens::secondary()),
        )
}
