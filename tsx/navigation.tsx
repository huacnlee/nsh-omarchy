// Omarchy navigation on `nsh/base`: Tabs, DropdownMenu (and its rows),
// ContextMenu, Accordion, Pagination, Sidebar and Link.
//
// Rows wash under the pointer and the keyboard, deepen while pressed and dim
// while disabled; the current item carries the selected fill and bright text,
// with no marker. Underline tabs keep their 2pt accent underline, which
// slides natively (at once under reduced motion).
import type { JSX } from "nsh";
import { Text, View } from "nsh";
import {
  Accordion as BaseAccordion,
  AccordionHeader,
  AccordionItem as BaseAccordionItem,
  AccordionPanel,
  AccordionTrigger,
  ContextMenu as BaseContextMenu,
  Link as BaseLink,
  Menu as BaseMenu,
  MenuItem as BaseMenuItem,
  MenuSeparator as BaseMenuSeparator,
  Pagination as BasePagination,
  type Placement,
  type Align,
  Tab,
  Tabs as BaseTabs,
} from "nsh/base";
import { Glyph } from "./icons";
import { cx, fill, ink, line, motion, surface, text } from "./theme";

/** One tab, sidebar row or segment. */
export interface NavItem {
  /** The app's id of the item, never its label. */
  value: string;
  label: string;
  disabled?: boolean;
  /** A count or note at the row's end (sidebar). */
  badge?: string;
}

export interface TabsProps {
  items: NavItem[];
  /** The selected tab's value (controlled). */
  value: string;
  /** The tab asked for: write it back to `value`, and show its panel. */
  onChange?: (event: { value: string }) => void;
  /** `underline` (default): a divider under the row, the selected tab in the selected fill. `segmented`: the same fill in a frame. */
  variant?: "underline" | "segmented";
  /** What screen readers call the list ("Sections"). */
  accessibilityLabel?: string;
  class?: string;
}

/**
 * A tab list, one stop of Tab: Left/Right (and Up/Down, Home/End) move between
 * tabs and select them as they go; the indicator slides to the selected tab.
 * Show the selected tab's content under it yourself, by `value`.
 */
export function Tabs(props: TabsProps) {
  const segmented = () => props.variant === "segmented";
  return (
    <BaseTabs
      class={cx(
        "flex-row self-start",
        segmented() ? "border border-border p-0.5" : "self-stretch border-b border-foreground/12",
        props.class,
      )}
      accessibilityLabel={props.accessibilityLabel}
      indicatorClass={segmented() ? "inset-y-0.5 bg-foreground/18" : "inset-y-0 bg-foreground/18"}
    >
      <Each of={props.items} key={(item) => item.value}>
        {(item) => (
          <Tab
            class={cx(
              "h-7 px-3 items-center justify-center border border-transparent",
              text.md,
              ink.secondary,
              fill.hover,
              fill.focus,
              line.focus,
              fill.pressed,
              "selected:text-bright",
              "disabled:bg-transparent disabled:text-foreground/38",
              motion.fast,
            )}
            selected={props.value === item().value}
            disabled={item().disabled}
            onPress={() => props.onChange?.({ value: item().value })}
          >
            <Text>{item().label}</Text>
          </Tab>
        )}
      </Each>
    </BaseTabs>
  );
}

/** A popup menu's surface and entrance. */
const menuPopup = cx(surface.popup, "min-w-44 p-1", motion.base);

export interface DropdownMenuProps {
  /** What opens it: a `Button` usually. Press, Return, Space or Down open it. */
  trigger: JSX.Element;
  /** `MenuItem`, `MenuSeparator`, `MenuLabel`. */
  children: JSX.Element;
  /** Open (controlled), with `onOpenChange`; without it the menu keeps its own state. */
  open?: boolean;
  onOpenChange?: (event: { open: boolean }) => void;
  placement?: Placement;
  align?: Align;
  testID?: string;
}

/**
 * A menu under its trigger: arrows, Home / End and typeahead move between
 * rows, Return chooses, Escape and a press outside close it, and the focus
 * goes back to the trigger.
 */
export function DropdownMenu(props: DropdownMenuProps) {
  return (
    <BaseMenu
      trigger={props.trigger}
      open={props.open}
      onOpenChange={props.onOpenChange}
      placement={props.placement}
      align={props.align}
      offset={2}
      testID={props.testID}
      popupClass={menuPopup}
      popupStartingClass={motion.popupStarting}
      popupEndingClass={motion.popupEnding}
    >
      {props.children}
    </BaseMenu>
  );
}

export interface ContextMenuProps {
  /** The area a right click (a long press on touch screens) opens it over. */
  children: JSX.Element;
  /** The menu's rows: `MenuItem`, `MenuSeparator`, `MenuLabel`. */
  menu: JSX.Element;
  disabled?: boolean;
  class?: string;
}

/** A menu at the pointer, opened by a right click on its area. */
export function ContextMenu(props: ContextMenuProps) {
  return (
    <BaseContextMenu
      class={props.class}
      disabled={props.disabled}
      trigger={props.children}
      popupClass={menuPopup}
      popupStartingClass={motion.popupStarting}
      popupEndingClass={motion.popupEnding}
    >
      {props.menu}
    </BaseContextMenu>
  );
}

export interface MenuItemProps {
  label: string;
  /** A shortcut shown at the row's end, in monospace ("Ctrl+S"). */
  shortcut?: string;
  /** A checkable row: an accent check while checked. */
  checked?: boolean;
  /** One choice of a group of checkable rows (screen readers say radio). */
  radio?: boolean;
  /** Destructive: danger text. */
  danger?: boolean;
  disabled?: boolean;
  /** Choosing it closes the menu (default true). */
  closeOnSelect?: boolean;
  onSelect?: () => void;
  testID?: string;
}

/** A row of a menu. */
export function MenuItem(props: MenuItemProps) {
  const checkable = props.checked !== undefined;
  return (
    <BaseMenuItem
      class={cx(
        "group h-7 flex-row items-center gap-2 px-2",
        text.md,
        props.danger ? ink.danger : ink.body,
        fill.hover,
        fill.highlight,
        fill.pressed,
        "disabled:bg-transparent disabled:text-foreground/38",
        motion.fast,
      )}
      checked={checkable ? props.checked : undefined}
      role={props.radio ? "menuitemradio" : undefined}
      disabled={props.disabled}
      closeOnSelect={props.closeOnSelect}
      testID={props.testID}
      onSelect={props.onSelect}
      indicatorClass={checkable ? "size-3 items-center justify-center" : undefined}
      indicator={
        checkable ? (
          <Glyph name={props.radio ? "dot" : "check"} class="text-omarchy-accent opacity-0 group-checked:opacity-100" />
        ) : undefined
      }
    >
      <Text class="flex-1">{props.label}</Text>
      <If when={props.shortcut}>
        <Text class={cx("font-mono", text.sm, ink.secondary)}>{props.shortcut}</Text>
      </If>
    </BaseMenuItem>
  );
}

/** A 1pt divider between groups of rows. */
export function MenuSeparator() {
  return <BaseMenuSeparator class="my-1 h-px bg-foreground/12" />;
}

/** A group's heading inside a menu. */
export function MenuLabel(props: { label: string }) {
  return <Text class={cx("px-2 pt-1.5 pb-1", text.xs, ink.secondary)}>{props.label}</Text>;
}

export interface AccordionProps {
  /** The open sections' values (controlled). */
  value: string[];
  /** The open sections as they would be: write them back to `value`. */
  onChange?: (event: { value: string[] }) => void;
  /** Several open at once. */
  multiple?: boolean;
  disabled?: boolean;
  /** The `AccordionSection`s. */
  children: JSX.Element;
  class?: string;
}

/** Sections that open under their headings, with a springing height. */
export function Accordion(props: AccordionProps) {
  return (
    <BaseAccordion
      class={cx("border-t border-foreground/12", props.class)}
      value={props.value}
      multiple={props.multiple}
      disabled={props.disabled}
      onChange={props.onChange}
    >
      {props.children}
    </BaseAccordion>
  );
}

export interface AccordionSectionProps {
  /** The app's id of the section. */
  value: string;
  title: string;
  disabled?: boolean;
  children: JSX.Element;
}

/** One section of an `Accordion`: a heading whose trigger toggles its panel. */
export function AccordionSection(props: AccordionSectionProps) {
  return (
    <BaseAccordionItem value={props.value} disabled={props.disabled} class="border-b border-foreground/12">
      <AccordionHeader>
        <AccordionTrigger
          class={cx(
            "group h-8 flex-row items-center justify-between px-2 border border-transparent",
            text.md,
            ink.body,
            fill.hover,
            fill.focus,
            line.focus,
            fill.pressed,
            "aria-expanded:text-bright",
            "disabled:bg-transparent disabled:text-foreground/38",
            motion.fast,
          )}
        >
          <Text>{props.title}</Text>
          <Glyph name="chevron-down" class={cx(ink.secondary, "group-aria-expanded:rotate-180", motion.base)} />
        </AccordionTrigger>
      </AccordionHeader>
      <AccordionPanel reveal accessibilityLabel={props.title}>
        <View class={cx("px-2 pb-3 pt-1", text.md, ink.body)}>{props.children}</View>
      </AccordionPanel>
    </BaseAccordionItem>
  );
}

export interface PaginationProps {
  /** The current page, from 1 (controlled). */
  page: number;
  totalPages: number;
  /** The page asked for: write it back to `page`. */
  onChange?: (event: { page: number }) => void;
  /** The most pages shown at once (default 7). */
  visiblePages?: number;
  disabled?: boolean;
  /** What it pages through ("Results"). */
  accessibilityLabel?: string;
  class?: string;
}

/**
 * Square page cells, Previous and Next at the ends (disabled there); the
 * current page has the selected fill and bright text.
 */
export function Pagination(props: PaginationProps) {
  return (
    <BasePagination
      class={cx("flex-row self-start gap-1", props.class)}
      page={props.page}
      totalPages={props.totalPages}
      visiblePages={props.visiblePages ?? 7}
      disabled={props.disabled}
      accessibilityLabel={props.accessibilityLabel}
      previousAccessibilityLabel="Previous page"
      nextAccessibilityLabel="Next page"
      onChange={props.onChange}
      itemClass={cx(
        "size-7 items-center justify-center border border-transparent tabular-nums",
        text.md,
        ink.body,
        fill.hover,
        line.focus,
        fill.pressed,
        "selected:bg-foreground/18 selected:text-bright",
        "disabled:bg-transparent disabled:text-foreground/38",
        motion.fast,
      )}
      renderItem={(item) =>
        item.kind === "previous" ? (
          <Glyph name="chevron-left" />
        ) : item.kind === "next" ? (
          <Glyph name="chevron-right" />
        ) : (
          <Text>{item.kind === "ellipsis" ? "…" : String(item.page)}</Text>
        )
      }
    />
  );
}

/** A group of the sidebar. */
export interface SidebarSection {
  /** The group's heading. */
  title?: string;
  items: NavItem[];
}

export interface SidebarProps {
  sections: SidebarSection[];
  /** The current item's value (controlled). */
  value: string;
  /** The item asked for: write it back to `value`. */
  onChange?: (event: { value: string }) => void;
  /** What screen readers call it ("Components"). */
  accessibilityLabel?: string;
  /** Content above the groups (a title). */
  header?: JSX.Element;
  /** Content below the groups. */
  footer?: JSX.Element;
  class?: string;
}

/**
 * A navigation sidebar: grouped rows, one stop of Tab; Up / Down (and Home /
 * End) move to the previous / next item and select it. The current row has
 * the selected fill and bright text.
 */
export function Sidebar(props: SidebarProps) {
  return (
    <View class={cx("w-52 gap-2 border-r border-border bg-surface py-2", props.class)}>
      {props.header}
      <BaseTabs class="gap-0.5" accessibilityLabel={props.accessibilityLabel}>
        <Each of={props.sections} key={(section) => section.title ?? section.items[0]?.value}>
          {(section) => (
            <View class="gap-0.5 pb-2">
              <If when={section().title}>
                <Text class={cx("px-3 pt-1 pb-1", text.xs, ink.secondary)}>{section().title}</Text>
              </If>
              <Each of={section().items} key={(item) => item.value}>
                {(item) => (
                  <Tab
                    class={cx(
                      "h-7 flex-row items-center justify-between px-3 border border-transparent",
                      text.md,
                      ink.body,
                      fill.hover,
                      fill.focus,
                      line.focus,
                      fill.pressed,
                      "selected:bg-foreground/18 selected:text-bright",
                      "disabled:bg-transparent disabled:text-foreground/38",
                      motion.fast,
                    )}
                    selected={props.value === item().value}
                    disabled={item().disabled}
                    onPress={() => props.onChange?.({ value: item().value })}
                  >
                    <Text>{item().label}</Text>
                    <If when={item().badge}>
                      <Text class={cx(text.sm, ink.secondary, "tabular-nums")}>{item().badge}</Text>
                    </If>
                  </Tab>
                )}
              </Each>
            </View>
          )}
        </Each>
      </BaseTabs>
      <If when={props.footer}>
        <View class="mt-auto">{props.footer}</View>
      </If>
    </View>
  );
}

export interface LinkProps {
  label: string;
  /** A URL (opened by the system) or a path of the app's (with `onOpen`). */
  href: string;
  /** `app`: `onOpen` opens it; `system` (default) opens it outside the app. */
  openWith?: "system" | "app";
  onOpen?: (event: { href: string }) => void;
  disabled?: boolean;
  class?: string;
}

/** Underlined accent text that brightens under the pointer, with the focus border when the keyboard reaches it. */
export function Link(props: LinkProps) {
  return (
    <BaseLink
      class={cx(
        "self-start border border-transparent px-0.5",
        text.md,
        ink.accent,
        "underline hover:text-bright",
        fill.focus,
        line.focus,
        "disabled:no-underline disabled:text-foreground/38",
        motion.fast,
        props.class,
      )}
      href={props.href}
      openWith={props.openWith}
      onOpen={props.onOpen}
      disabled={props.disabled}
    >
      <Text>{props.label}</Text>
    </BaseLink>
  );
}
