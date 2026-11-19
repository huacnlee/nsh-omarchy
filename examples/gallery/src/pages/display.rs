//! The display pages: surfaces, badges, keycaps, empty states, avatars,
//! icons, the type ramp and Markdown.

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, ControlSize, IconName, tokens};

use crate::registry::Page;

/// A labeled group of demos on a page.
fn section(title: &'static str, content: impl IntoElement) -> View {
    view()
        .gap(px(tokens::SPACE_2))
        .child(om::caption(title).font_weight(FontWeight::SEMIBOLD))
        .child(content)
}

/// A wrapping row of demos.
fn row() -> View {
    view().flex_row().flex_wrap().items_center().gap(px(tokens::SPACE_2))
}

pub fn panel() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .max_w(px(560.0))
        .child(section(
            "Panel",
            om::panel("Workspace")
                .child(om::body(
                    "A focused desktop workspace for files, notes and project activity.",
                ))
                .child(om::separator())
                .child(
                    row()
                        .child(om::icon(IconName::Folder).text_color(move || tokens::secondary()))
                        .child(om::body("~/Work/omarchy"))
                        .child(view().flex_1())
                        .child(om::badge("Synced").success()),
                ),
        ))
        .child(section(
            "Outline",
            om::panel("Release checklist")
                .outline()
                .child(om::caption("3 of 5 done · updated 2 minutes ago")),
        ))
        .child(section(
            "Card",
            om::card()
                .title("Updates")
                .description("3 packages can be upgraded.")
                .footer(om::button("Update").primary().small())
                .child(om::code("omarchy-theme-set tokyo-night")),
        ))
        .into_element()
}

pub fn separator() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .max_w(px(560.0))
        .child(section(
            "Horizontal",
            om::card()
                .child(om::body("Above the rule"))
                .child(om::separator())
                .child(om::body("Below the rule")),
        ))
        .child(section(
            "Vertical",
            view()
                .flex_row()
                .h(px(20.0))
                .items_center()
                .gap(px(tokens::SPACE_3))
                .child(om::body("Files"))
                .child(om::separator().vertical())
                .child(om::body("Edit"))
                .child(om::separator().vertical())
                .child(om::body("View")),
        ))
        .into_element()
}

pub fn badge() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .child(section(
            "Variants",
            row()
                .child(om::badge("Default"))
                .child(om::badge("Primary").primary())
                .child(om::badge("Success").success())
                .child(om::badge("Warning").warning())
                .child(om::badge("Danger").danger()),
        ))
        .child(section(
            "Outline",
            row()
                .child(om::badge("Neutral").outline(true))
                .child(om::badge("New").primary().outline(true))
                .child(om::badge("Online").success().outline(true))
                .child(om::badge("Beta").warning().outline(true))
                .child(om::badge("Failed").danger().outline(true)),
        ))
        .child(section(
            "Sizes",
            // A badge keeps its own width (`self_start`); centered in a row.
            row()
                .child(om::badge("Small").primary().size(ControlSize::Small).self_center())
                .child(om::badge("Medium").primary().self_center())
                .child(om::badge("Large").primary().size(ControlSize::Large).self_center()),
        ))
        .child(section(
            "In context",
            row()
                .child(om::body("Build #482"))
                .child(om::badge("Passing").success().outline(true))
                .child(om::badge("2 warnings").warning().outline(true)),
        ))
        .into_element()
}

pub fn keycap() -> AnyElement {
    let shortcut = |keys: &'static str, action: &'static str| {
        view()
            .flex_row()
            .items_center()
            .gap(px(tokens::SPACE_3))
            .child(view().w(px(160.0)).child(om::shortcut(keys)))
            .child(om::body(action))
            .into_element()
    };
    view()
        .gap(px(tokens::SPACE_6))
        .child(section(
            "Single keys",
            row()
                .child(om::keycap("esc"))
                .child(om::keycap("enter"))
                .child(om::keycap("tab"))
                .child(om::keycap("mod"))
                .child(om::keycap("j"))
                .child(om::keycap("k")),
        ))
        .child(section(
            "Shortcuts",
            view()
                .gap(px(tokens::SPACE_2))
                .child(shortcut("super-space", "Open the launcher"))
                .child(shortcut("super-enter", "Open a terminal"))
                .child(shortcut("mod-shift-p", "Show the commands")),
        ))
        .child(section(
            "As text",
            om::caption(format!("Save is {} here", om::shortcut_label("mod-s"))),
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
            dynamic(move || {
                if created.get() {
                    row()
                        .p(px(tokens::SPACE_4))
                        .child(
                            om::icon(IconName::CircleCheck).text_color(move || tokens::success()),
                        )
                        .child(om::body("Project created"))
                        .into_element()
                } else {
                    om::empty_state("No projects yet")
                        .description(
                            "Create a project to collect files, notes and tasks in one place.",
                        )
                        .icon(IconName::Folder)
                        .child(
                            om::button("Create project")
                                .primary()
                                .on_press(move || created.set(true)),
                        )
                        .into_element()
                }
            }),
        ))
        .child(section(
            "Without an action",
            om::empty_state("Inbox zero")
                .description("New notifications will show up here.")
                .icon(IconName::Inbox),
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
                .child(om::avatar("Ada Lovelace").size(ControlSize::Small))
                .child(om::avatar("Ada Lovelace"))
                .child(om::avatar("Ada Lovelace").size(ControlSize::Large)),
        ))
        .child(section(
            "Initials",
            view().gap(px(tokens::SPACE_2)).children(people.map(|name| {
                row().child(om::avatar(name)).child(om::body(name)).into_element()
            })),
        ))
        .into_element()
}

pub fn icon() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .child(section(
            "Set",
            row().children(IconName::ALL.iter().map(|name| {
                view()
                    .w(px(96.0))
                    .items_center()
                    .gap(px(tokens::SPACE_1))
                    .py(px(tokens::SPACE_2))
                    .child(om::icon(*name))
                    .child(om::caption(name.name()))
                    .into_element()
            })),
        ))
        .child(section(
            "Tinted and sized",
            row()
                .child(om::icon(IconName::CircleCheck).text_color(move || tokens::success()))
                .child(om::icon(IconName::TriangleAlert).text_color(move || tokens::warning()))
                .child(om::icon(IconName::CircleX).text_color(move || tokens::danger()))
                .child(om::icon(IconName::Star).text_color(move || tokens::accent()))
                .child(om::icon(IconName::Settings).size(tokens::CONTROL_LG)),
        ))
        .child(section(
            "Labeled",
            row()
                .child(om::icon(IconName::Lock).label("Encrypted"))
                .child(om::caption("A standalone icon screen readers announce")),
        ))
        .into_element()
}

pub fn typography() -> AnyElement {
    view()
        .gap(px(tokens::SPACE_3))
        .max_w(px(560.0))
        .child(om::heading("Heading"))
        .child(om::title("Title"))
        .child(om::body(
            "Body text sets paragraphs in the compact desktop size Omarchy's shell uses.",
        ))
        .child(om::caption("Caption: secondary hints and timestamps"))
        .child(
            row()
                .child(om::body("Run"))
                .child(om::code("omarchy-update"))
                .child(om::body("to update.")),
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
    let pressed = signal(String::new());
    view()
        .gap(px(tokens::SPACE_4))
        .max_w(px(640.0))
        .child(
            om::card().child(
                om::markdown(SAMPLE).on_link_press(move |url: &SharedString| pressed.set(url.to_string())),
            ),
        )
        .child(om::caption(move || {
            let url = pressed.get();
            if url.is_empty() {
                SharedString::from("Press a link")
            } else {
                SharedString::from(format!("Link pressed: {url}"))
            }
        }))
        .into_element()
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Display", id: "panel", title: "Panel", make: panel },
    Page { group: "Display", id: "separator", title: "Separator", make: separator },
    Page { group: "Display", id: "badge", title: "Badge", make: badge },
    Page { group: "Display", id: "keycap", title: "Keycap", make: keycap },
    Page { group: "Display", id: "empty_state", title: "Empty state", make: empty_state },
    Page { group: "Display", id: "avatar", title: "Avatar", make: avatar },
    Page { group: "Display", id: "icon", title: "Icon", make: icon },
    Page { group: "Display", id: "typography", title: "Typography", make: typography },
    Page { group: "Display", id: "markdown", title: "Markdown", make: markdown },
];
