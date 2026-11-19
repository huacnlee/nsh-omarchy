//! Navigation: tabs, the sidebar, accordions, collapsibles, pagination and
//! the title bar, one file each under `navigation/`. (The menus are in
//! [`super::menu`].)

use nsh_core::prelude::*;

pub mod accordion;
pub mod pagination;
pub mod sidebar;
pub mod tabs;
pub mod title_bar;

/// The chosen item of a [`tabs::Tabs`] or a [`sidebar::Sidebar`]: the
/// app's `value` while it gives one (controlled), else the component's own.
#[derive(Clone, Copy)]
pub(crate) struct Chosen {
    value: Prop<Option<SharedString>>,
    own: Signal<Option<SharedString>>,
}

impl Chosen {
    pub(crate) fn new(value: Prop<Option<SharedString>>) -> Self {
        Self {
            value,
            own: signal(None),
        }
    }

    /// The chosen value (tracked).
    pub(crate) fn current(self) -> Option<SharedString> {
        self.value.get().or_else(|| self.own.get())
    }

    /// Whether `value` is the chosen one (tracked).
    pub(crate) fn is(self, value: &SharedString) -> bool {
        self.current().as_ref() == Some(value)
    }

    /// The first enabled item an uncontrolled component meets becomes its
    /// value.
    pub(crate) fn seed(self, value: SharedString) {
        if self.value.with_untracked(Option::is_none) && self.own.with_untracked(Option::is_none) {
            self.own.set(Some(value));
        }
    }

    /// The user asks for `value`: whether it is a change (then the caller
    /// reports it). Uncontrolled, the component takes it at once.
    pub(crate) fn request(self, value: &SharedString) -> bool {
        let current = untracked(move || self.current());
        if current.as_ref() == Some(value) {
            return false;
        }
        if self.value.with_untracked(Option::is_none) {
            self.own.set(Some(value.clone()));
        }
        true
    }
}

/// Registers this area's components for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<tabs::Tabs>()
        .register::<tabs::Tab>()
        .register::<sidebar::Sidebar>()
        .register::<sidebar::SidebarSection>()
        .register::<sidebar::SidebarItem>()
        .register::<accordion::Accordion>()
        .register::<accordion::AccordionItem>()
        .register::<accordion::Collapsible>()
        .register::<pagination::Pagination>()
        .register::<title_bar::TitleBar>();
    super::menu::register(registry);
}
