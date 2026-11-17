//! Gallery pages of the `display` group. Each `pub fn` is one page.

use nsh_base::button;
use nsh_core::prelude::*;

use crate::components::{
    AvatarSize, BadgeVariant, IconName, avatar as person, badge as chip, body_text, caption_text,
    card, code_text, empty_state as empty, heading_text, icon as glyph, inset_well, keycap as cap,
    keycaps, label_text, labeled_icon, markdown as markdown_view, panel as surface_panel,
    separator as rule, title_text, vertical_separator,
};
use crate::tokens;

/// A labeled group of demos on a page.
fn section(title: &'static str, content: impl IntoElement) -> View {
    view()
        .gap(px(tokens::SPACE_2))
        .child(caption_text(title).font_weight(FontWeight::SEMIBOLD))
        .child(content)
}

/// A small outlined button for the demos.
fn demo_button(label: &'static str, on_press: impl Fn() + 'static) -> impl IntoElement {
    button()
        .on_press(on_press)
        .accessibility_label(label)
        .self_center()
        .flex_row()
        .items_center()
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_3))
        .border_1()
        .border_color(move || tokens::accent())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| {
            s.border_color(move || tokens::focus_border())
                .bg(move || tokens::hover_fill())
        })
        .child(
            text(label)
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tokens::accent()),
        )
}

pub fn panel() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .max_w(px(560.0))
        .child(section(
            "Panel",
            surface_panel("Workspace")
                .child(body_text(
                    "A focused desktop workspace for files, notes and project activity.",
                ))
                .child(rule())
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap(px(tokens::SPACE_2))
                        .child(glyph(IconName::Folder).text_color(move || tokens::secondary()))
                        .child(body_text("~/Work/omarchy"))
                        .child(view().flex_1())
                        .child(chip("Synced", BadgeVariant::Success)),
                ),
        ))
        .child(section(
            "Card",
            card()
                .child(label_text("Release checklist"))
                .child(caption_text("3 of 5 done · updated 2 minutes ago")),
        ))
        .child(section(
            "Inset well",
            inset_well().child(code_text("omarchy-theme-set tokyo-night")),
        ))
        .into_element()
}

pub fn separator() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .max_w(px(560.0))
        .child(section(
            "Horizontal",
            card()
                .child(body_text("Above the rule"))
                .child(rule())
                .child(body_text("Below the rule")),
        ))
        .child(section(
            "Vertical",
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_3))
                .child(body_text("Files"))
                .child(vertical_separator())
                .child(body_text("Edit"))
                .child(vertical_separator())
                .child(body_text("View")),
        ))
        .into_element()
}

pub fn badge() -> AnyElement {
    let names = ["Default", "Accent", "Success", "Warning", "Danger"];
    view()
        .gap(px(tokens::SPACE_6))
        .child(section(
            "Variants",
            view()
                .flex_row()
                .flex_wrap()
                .gap(px(tokens::SPACE_2))
                .children(
                    BadgeVariant::ALL
                        .iter()
                        .zip(names)
                        .map(|(variant, name)| chip(name, *variant).into_element()),
                ),
        ))
        .child(section(
            "In context",
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_2))
                .child(body_text("Build #482"))
                .child(chip("Passing", BadgeVariant::Success))
                .child(chip("2 warnings", BadgeVariant::Warning)),
        ))
        .into_element()
}

pub fn keycap() -> AnyElement {
    let shortcut = |keys: &[&'static str], action: &'static str| {
        view()
            .flex_row()
            .items_center()
            .gap(px(tokens::SPACE_3))
            .child(keycaps(keys.iter().copied()).w(px(160.0)))
            .child(body_text(action))
            .into_element()
    };
    view()
        .gap(px(tokens::SPACE_6))
        .child(section(
            "Single keys",
            view()
                .flex_row()
                .gap(px(tokens::SPACE_2))
                .child(cap("esc"))
                .child(cap("enter"))
                .child(cap("⌘K"))
                .child(cap("j"))
                .child(cap("k")),
        ))
        .child(section(
            "Shortcuts",
            view()
                .gap(px(tokens::SPACE_2))
                .child(shortcut(&["super", "space"], "Open the launcher"))
                .child(shortcut(&["super", "enter"], "Open a terminal"))
                .child(shortcut(&["super", "w"], "Close the window")),
        ))
        .into_element()
}

pub fn empty_state() -> AnyElement {
    let created = signal(false);
    view()
        .gap(px(tokens::SPACE_6))
        .max_w(px(560.0))
        .child(section(
            "With an action",
            card().child(dynamic(move || {
                if created.get() {
                    view()
                        .flex_row()
                        .items_center()
                        .gap(px(tokens::SPACE_2))
                        .p(px(tokens::SPACE_4))
                        .child(glyph(IconName::CircleCheck).text_color(move || tokens::success()))
                        .child(body_text("Project created"))
                        .into_element()
                } else {
                    empty(
                        IconName::Folder,
                        "No projects yet",
                        "Create a project to collect files, notes and tasks in one place.",
                    )
                    .child(demo_button("Create project", move || created.set(true)))
                    .into_element()
                }
            })),
        ))
        .child(section(
            "Without an action",
            card().child(empty(
                IconName::Inbox,
                "Inbox zero",
                "New notifications will show up here.",
            )),
        ))
        .into_element()
}

pub fn avatar() -> AnyElement {
    let people = ["Ada Lovelace", "Grace Hopper", "Linus Torvalds", "huacnlee"];
    view()
        .gap(px(tokens::SPACE_6))
        .child(section(
            "Sizes",
            view()
                .flex_row()
                .items_end()
                .gap(px(tokens::SPACE_3))
                .child(person("Ada Lovelace", AvatarSize::Small))
                .child(person("Ada Lovelace", AvatarSize::Medium))
                .child(person("Ada Lovelace", AvatarSize::Large)),
        ))
        .child(section(
            "Initials",
            view().gap(px(tokens::SPACE_2)).children(people.map(|name| {
                view()
                    .flex_row()
                    .items_center()
                    .gap(px(tokens::SPACE_2))
                    .child(person(name, AvatarSize::Medium))
                    .child(body_text(name))
                    .into_element()
            })),
        ))
        .into_element()
}

pub fn icon() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .child(section(
            "Set",
            view()
                .flex_row()
                .flex_wrap()
                .gap(px(tokens::SPACE_2))
                .children(IconName::ALL.iter().map(|name| {
                    view()
                        .w(px(96.0))
                        .items_center()
                        .gap(px(tokens::SPACE_1))
                        .py(px(tokens::SPACE_2))
                        .child(glyph(*name))
                        .child(caption_text(name.name()))
                        .into_element()
                })),
        ))
        .child(section(
            "Tinted and sized",
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_3))
                .child(glyph(IconName::CircleCheck).text_color(move || tokens::success()))
                .child(glyph(IconName::TriangleAlert).text_color(move || tokens::warning()))
                .child(glyph(IconName::CircleX).text_color(move || tokens::danger()))
                .child(glyph(IconName::Star).text_color(move || tokens::accent()))
                .child(glyph(IconName::Settings).size(px(tokens::CONTROL_LG))),
        ))
        .child(section(
            "Labeled",
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_2))
                .child(labeled_icon(IconName::Lock, "Encrypted"))
                .child(caption_text("A standalone icon screen readers announce")),
        ))
        .into_element()
}

pub fn typography() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_3))
        .max_w(px(560.0))
        .child(heading_text("Heading"))
        .child(title_text("Title"))
        .child(body_text(
            "Body text sets paragraphs in the compact desktop size Omarchy's shell uses.",
        ))
        .child(label_text("Label"))
        .child(caption_text("Caption: secondary hints and timestamps"))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap(px(tokens::SPACE_2))
                .child(body_text("Run"))
                .child(code_text("omarchy-update"))
                .child(body_text("to update.")),
        )
        .into_element()
}

const SAMPLE: &str = "\
# Release notes

Omarchy's **Markdown** view sets documents in the compact type scale, with \
`inline code`, *emphasis* and [links](https://omarchy.org).

## Highlights

- Square code blocks on the inset surface
- Task lists that follow the accent
- [x] Theme tokens
- [ ] Reduced motion

> Quotes keep a rule in the accent color.

```sh
omarchy-theme-set tokyo-night
```

| Key | Action |
|---|---|
| super + space | Launcher |
| super + enter | Terminal |
";

pub fn markdown() -> AnyElement {
    let pressed = signal(SharedString::from(""));
    view()
        .gap(px(tokens::SPACE_4))
        .max_w(px(640.0))
        .child(
            card().child(
                markdown_view()
                    .source(SAMPLE)
                    .on_link_press(move |url: &SharedString| pressed.set(url.clone())),
            ),
        )
        .child(caption_text(move || {
            let url = pressed.get();
            if url.is_empty() {
                SharedString::from("Press a link")
            } else {
                SharedString::from(format!("Link pressed: {url}"))
            }
        }))
        .into_element()
}
