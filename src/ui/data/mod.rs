//! Data: tables, the data table, list items, trees, scrollbars and
//! resizable panels.

pub mod data_table;
pub mod list;
pub mod resizable;
pub mod table;
pub mod tree;

use nsh_core::prelude::*;

/// Registers this area's components for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<table::Table>()
        .register::<table::TableHeader>()
        .register::<table::TableBody>()
        .register::<table::TableRow>()
        .register::<table::TableHead>()
        .register::<table::TableCell>()
        .register::<data_table::DataTable>()
        .register::<list::ListItem>()
        .register::<list::Scrollbar>()
        .register::<tree::Tree>()
        .register::<resizable::ResizablePanelGroup>()
        .register::<resizable::ResizablePanel>();
}
