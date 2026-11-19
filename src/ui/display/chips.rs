//! [`Badge`], [`Keycap`], [`Shortcut`] and [`Avatar`].

use nsh_core::prelude::*;

use crate::components::avatar::{AvatarSize, avatar as avatar_view};
use crate::components::button::ControlSize;
use crate::components::shortcut::shortcut_label;
use crate::tokens;

// ---- Badge ----------------------------------------------------------------

/// A [`Badge`]'s meaning, and so its color.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum BadgeVariant {
    /// No meaning of its own: the neutral fill.
    #[default]
    Default,
    /// The theme's accent: new, highlighted.
    Primary,
    /// Done, online, passing.
    Success,
    /// Needs a look.
    Warning,
    /// Failed.
    Danger,
}

impl BadgeVariant {
    /// The variant's color: an outlined badge's border and text, a filled
    /// badge's fill (read it in a binding closure).
    fn color(self) -> AdaptiveColor {
        match self {
            BadgeVariant::Primary => tokens::accent(),
            BadgeVariant::Success => tokens::success(),
            BadgeVariant::Warning => tokens::warning(),
            BadgeVariant::Danger => tokens::danger(),
            _ => tokens::control_border(),
        }
    }
}

/// The props of [`Badge`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct BadgeProps {
    /// The status or count, shown in capitals.
    pub label: Prop<SharedString>,
    /// `default` (neutral), `primary`, `success`, `warning` or `danger`.
    #[prop(default)]
    pub variant: Prop<BadgeVariant>,
    /// Only a border and text in the variant's color, no fill.
    #[prop(default)]
    pub outline: Prop<bool>,
    /// `small` (16 points tall), `medium` (20, the default) or `large` (24).
    #[prop(default)]
    pub size: Prop<ControlSize>,
}

/// A short status or count: filled in its variant's color, or `outline`.
#[component]
pub fn Badge(props: BadgeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let variant = props.variant;
    let outline = props.outline;
    let size = props.size;
    let label = props.label;
    view()
        .flex_row()
        .flex_shrink_0()
        // Its own width in a column; `self-center` sits it in a row.
        .self_start()
        .items_center()
        .justify_center()
        .h(move || {
            px(match size.get() {
                ControlSize::Small => 16.0,
                ControlSize::Large => 24.0,
                _ => 20.0,
            })
        })
        .px(move || {
            px(match size.get() {
                ControlSize::Small => tokens::SPACE_1,
                ControlSize::Large => tokens::SPACE_2,
                _ => tokens::SPACE_1 * 1.5,
            })
        })
        .border(px(tokens::BORDER))
        .border_color(move || {
            if outline.get() {
                variant.get().color()
            } else {
                AdaptiveColor::TRANSPARENT
            }
        })
        .bg(move || match (outline.get(), variant.get()) {
            (true, _) => AdaptiveColor::TRANSPARENT,
            (false, BadgeVariant::Default) => tokens::selected_fill(),
            (false, variant) => variant.color(),
        })
        .child(
            text(move || label.get())
                .text_size(move || {
                    px(match size.get() {
                        ControlSize::Large => tokens::TEXT_SM,
                        _ => tokens::TEXT_XS,
                    })
                })
                .uppercase()
                .letter_spacing(px(0.25))
                .text_color(move || match (outline.get(), variant.get()) {
                    (true, BadgeVariant::Default) => tokens::foreground(),
                    (true, variant) => variant.color(),
                    (false, BadgeVariant::Default) => tokens::bright(),
                    (false, _) => tokens::on_accent(),
                }),
        )
}

// ---- Keycap and Shortcut --------------------------------------------------

/// One key as this platform writes it: `"mod"` reads `cmd` on macOS and
/// `ctrl` elsewhere.
fn key_label(key: &str) -> SharedString {
    let label = shortcut_label(key);
    if label.is_empty() { key.to_owned().into() } else { label }
}

/// The keycap look: monospace on an inset square, its key following `label`.
fn keycap_view(label: impl IntoReactive<SharedString>) -> View {
    view()
        .flex_row()
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .h(px(18.0))
        .min_w(px(18.0))
        .px(px(tokens::SPACE_1))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::inset())
        .role(AccessibilityRole::KeyboardKey)
        .child(
            text(label)
                .font_family(move || tokens::font_mono())
                .text_size(px(tokens::TEXT_XS))
                .text_color(move || tokens::foreground()),
        )
}

/// The props of [`Keycap`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct KeycapProps {
    /// One key as Omarchy writes it: `"mod"`, `"shift"`, `"p"`, `"esc"`
    /// (`"mod"` shows as `cmd` on macOS, `ctrl` elsewhere).
    pub key: Prop<SharedString>,
}

/// One key, in monospace on an inset square: `<Keycap key="esc" />`.
#[component]
pub fn Keycap(props: KeycapProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let key = props.key;
    keycap_view(move || key_label(&key.get()))
}

/// The props of [`Shortcut`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ShortcutProps {
    /// The chord, keys joined by `-` as Omarchy writes them:
    /// `"mod-shift-p"`.
    pub keys: Prop<SharedString>,
}

/// A key combination, a keycap per key: `<Shortcut keys="mod-shift-p" />`.
/// Screen readers hear it as one element, the keys in order.
#[component]
pub fn Shortcut(props: ShortcutProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let keys = props.keys;
    let parts = move || -> Vec<SharedString> {
        shortcut_label(&keys.get())
            .split('-')
            .map(|k| SharedString::from(k.to_owned()))
            .collect()
    };
    view()
        .flex_row()
        .flex_shrink_0()
        .items_center()
        .gap(px(tokens::SPACE_1))
        .accessible(true)
        .label(move || SharedString::from(parts().join(" ")))
        .child(each(
            parts,
            |key: &SharedString| key.clone(),
            |key, _| keycap_view(move || key.get()),
        ))
}

// ---- Avatar ---------------------------------------------------------------

/// The props of [`Avatar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarProps {
    /// The person: screen readers say it, the initials come from it.
    pub name: Prop<SharedString>,
    /// The picture (a URL or a path), shown once it loads.
    pub src: Prop<Option<SharedString>>,
    /// `small` (24 points), `medium` (32, the default) or `large` (40).
    #[prop(default)]
    pub size: Prop<ControlSize>,
}

/// A square avatar: the picture, or the person's initials while it loads,
/// when it fails and without one.
#[component]
pub fn Avatar(props: AvatarProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let name = props.name;
    let src = props.src;
    let size = props.size;
    // A view around `nsh_base`'s avatar: the root a trigger slot reaches.
    view().flex_shrink_0().child(dynamic(move || {
        let (base, side) = match size.get() {
            ControlSize::Small => (AvatarSize::Small, tokens::CONTROL_SM),
            ControlSize::Large => (AvatarSize::Large, 40.0),
            _ => (AvatarSize::Medium, tokens::CONTROL_LG),
        };
        avatar_view(name.get(), base)
            .size(px(side))
            .src(move || src.get().map(Into::into))
            .into_element()
    }))
}
