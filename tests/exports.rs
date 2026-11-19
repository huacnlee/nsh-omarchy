//! What `#[native_module("nsh-omarchy")]` exports to TSX, read from the
//! declarations `nsh` writes for a mixed app (`.nsh/types/nsh-omarchy.d.ts`).

fn declarations() -> String {
    nsh_core::element::declarations(&nsh_omarchy::registry(), &[])
}

#[test]
fn the_set_is_imported_as_nsh_omarchy() {
    assert_eq!(nsh_omarchy::registry().specifier(), Some("nsh-omarchy"));
}

#[test]
fn every_export_is_declared() {
    let ts = declarations();
    for line in [
        "export declare const Omarchy: OmarchyModule;",
        "export declare const LineChart: ((props: LineChartProps) => JSX.Element);",
        "export declare const AreaChart: ((props: AreaChartProps) => JSX.Element);",
        "export declare const BarChart: ((props: BarChartProps) => JSX.Element);",
        "export declare const PieChart: ((props: PieChartProps) => JSX.Element);",
        "export declare const Markdown: ((props: MarkdownProps) => JSX.Element);",
        "export declare const Icon: ((props: IconProps) => JSX.Element);",
        "export declare const Button: ((props: ButtonProps) => JSX.Element);",
        "export declare const IconButton: ((props: IconButtonProps) => JSX.Element);",
        "export declare const DropdownMenu: ((props: DropdownMenuProps) => JSX.Element);",
        "export declare const ContextMenu: ((props: ContextMenuProps) => JSX.Element);",
        "export declare const MenuItem: ((props: MenuItemProps) => JSX.Element);",
        "export declare const MenuSeparator: ((props: MenuSeparatorProps) => JSX.Element);",
        "export declare const MenuLabel: ((props: MenuLabelProps) => JSX.Element);",
    ] {
        assert!(ts.contains(line), "missing {line:?} in\n{ts}");
    }
}

#[test]
fn the_module_and_props_have_their_ts_types() {
    let ts = declarations();
    for line in [
        "  followSystem(): void;",
        "  stopFollowing(): void;",
        "  applyTheme(name: BuiltInTheme): void;",
        "  current(): ThemeInfo | null;",
        "  onChange(listener: (theme: ThemeInfo) => void): () => void;",
        "  shortcutLabel(keys: string): string;",
        "export type BuiltInTheme = \"tokyo-night\" | \"flexoki-light\";",
        "export type ThemeAppearance = \"dark\" | \"light\";",
        "  data: (ChartPoint)[];",
        "  curve?: ChartCurve;",
        "  onLinkPress?: (event: { value: string }) => void;",
        "  name: IconName;",
    ] {
        assert!(ts.contains(line), "missing {line:?} in\n{ts}");
    }
    assert!(
        ts.contains("\"chevron-down\""),
        "icons by Lucide name:\n{ts}"
    );
    assert!(
        !ts.contains("Gallery"),
        "the gallery is the repository's app, not an export:\n{ts}"
    );
    assert!(
        !ts.contains("markdown()"),
        "Rust notes stay out of TS:\n{ts}"
    );
}

#[test]
fn every_export_is_documented() {
    assert_eq!(nsh_omarchy::registry().undocumented(), Vec::<String>::new());
}

#[test]
fn every_icon_has_a_ts_name() {
    use nsh_core::element::PropEnum;
    for icon in nsh_omarchy::components::IconName::ALL {
        assert_eq!(icon.js_name(), icon.name());
    }
}
