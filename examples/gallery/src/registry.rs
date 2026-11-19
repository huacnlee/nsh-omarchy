//! Every gallery page: its group, id, title and maker. Each area's module
//! lists its own pages; this joins them in sidebar order.

use nsh_core::prelude::*;

use crate::pages;

/// One gallery page.
pub struct Page {
    /// The sidebar group it is listed under.
    pub group: &'static str,
    /// A stable id (test ids, keys).
    pub id: &'static str,
    /// The sidebar label and page heading.
    pub title: &'static str,
    /// Makes the page.
    pub make: fn() -> AnyElement,
}

/// Every page, in sidebar order.
pub fn all() -> Vec<&'static Page> {
    [
        pages::actions::PAGES,
        pages::choices::PAGES,
        pages::fields::PAGES,
        pages::navigation::PAGES,
        pages::overlays::PAGES,
        pages::data::PAGES,
        pages::display::PAGES,
        pages::charts::PAGES,
    ]
    .into_iter()
    .flatten()
    .collect()
}
