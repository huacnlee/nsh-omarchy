//! Navigation: tabs, the sidebar, accordions, collapsibles, pagination,
//! menus and the title bar.

use nsh_core::element::IntoProp;
use nsh_core::prelude::*;

use crate::ui::menu;
use crate::ui::navigation::{accordion, pagination as pages, sidebar, tabs, title_bar as bar};

pub use crate::ui::menu::{
    ContextMenu, ContextMenuElement, Menu, MenuElement, MenuItem, MenuItemElement, MenuLabel,
    MenuLabelElement, MenuSeparator, MenuSeparatorElement,
};
pub use crate::ui::navigation::accordion::{
    Accordion, AccordionElement, AccordionItem, AccordionItemElement, Collapsible,
    CollapsibleElement,
};
pub use crate::ui::navigation::pagination::{Pagination, PaginationElement};
pub use crate::ui::navigation::sidebar::{
    Sidebar, SidebarElement, SidebarItem, SidebarItemElement, SidebarSection,
    SidebarSectionElement,
};
pub use crate::ui::navigation::tabs::{Tab, TabElement, Tabs, TabsElement, TabsVariant};
pub use crate::ui::navigation::title_bar::{TitleBar, TitleBarElement};

/// A tab list: `tabs().value(move || tab.get()).on_change(..).child(tab("general", "General"))`.
pub fn tabs() -> TabsElement {
    tabs::tabs()
}

/// One tab of a [`tabs`] list, named by `value`: `tab("general", "General")`.
pub fn tab<M1, M2>(
    value: impl IntoProp<SharedString, M1>,
    label: impl IntoProp<SharedString, M2>,
) -> TabElement {
    tabs::tab().value(value).label(label)
}

/// A navigation sidebar of [`sidebar_section`]s and [`sidebar_item`]s.
pub fn sidebar() -> SidebarElement {
    sidebar::sidebar()
}

/// A titled group of [`sidebar_item`]s: `sidebar_section("Mail")`.
pub fn sidebar_section<M>(title: impl IntoProp<SharedString, M>) -> SidebarSectionElement {
    sidebar::sidebar_section().title(title)
}

/// One row of a [`sidebar`], named by `value`: `sidebar_item("inbox", "Inbox")`.
pub fn sidebar_item<M1, M2>(
    value: impl IntoProp<SharedString, M1>,
    label: impl IntoProp<SharedString, M2>,
) -> SidebarItemElement {
    sidebar::sidebar_item().value(value).label(label)
}

/// Stacked [`accordion_item`]s that open under their headings.
pub fn accordion() -> AccordionElement {
    accordion::accordion()
}

/// One item of an [`accordion`], named by `value`, its content as children:
/// `accordion_item("shipping", "How long does shipping take?").child(..)`.
pub fn accordion_item<M1, M2>(
    value: impl IntoProp<SharedString, M1>,
    title: impl IntoProp<SharedString, M2>,
) -> AccordionItemElement {
    accordion::accordion_item().value(value).title(title)
}

/// A region under a trigger row titled `title`, its content as children:
/// `collapsible("Advanced").child(..)`.
pub fn collapsible<M>(title: impl IntoProp<SharedString, M>) -> CollapsibleElement {
    accordion::collapsible().title(title)
}

/// Page cells with Previous and Next: `pagination(move || page.get(), 12)
/// .on_change(move |p: &u32| page.set(*p))`.
pub fn pagination(
    page: impl IntoReactive<u32>,
    total_pages: impl IntoReactive<u32>,
) -> PaginationElement {
    pages::pagination().page(page).total_pages(total_pages)
}

/// A menu under its `trigger`: `menu().trigger(button("File")).child(menu_item("New"))`.
pub fn menu() -> MenuElement {
    menu::menu()
}

/// A menu opened by a secondary click on its children:
/// `context_menu().menu(view().child(menu_item("Copy"))).child(area)`.
pub fn context_menu() -> ContextMenuElement {
    menu::context_menu()
}

/// A row of a [`menu`] or [`context_menu`]: `menu_item("New").shortcut("mod-n")`.
pub fn menu_item<M>(label: impl IntoProp<SharedString, M>) -> MenuItemElement {
    menu::menu_item().label(label)
}

/// A 1pt divider between groups of menu rows.
pub fn menu_separator() -> MenuSeparatorElement {
    menu::menu_separator()
}

/// A heading over a group of menu rows: `menu_label("View")`.
pub fn menu_label<M>(label: impl IntoProp<SharedString, M>) -> MenuLabelElement {
    menu::menu_label().label(label)
}

/// The window's title bar, drawn by the app: `title_bar().title("Files")`.
pub fn title_bar() -> TitleBarElement {
    bar::title_bar()
}
