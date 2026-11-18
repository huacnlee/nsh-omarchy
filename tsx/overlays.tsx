// Omarchy overlays on `nsh/base`: Dialog, AlertDialog (and the native
// `confirm`), Sheet, Popover, Tooltip, HoverCard, and toasts (`Toaster` +
// `toast`). nsh/base owns focus, Escape, dismissal and the roles; this file
// draws them: square surfaces with a 1pt border, a scrim behind modals,
// popups that fade and rise 4pt over 140 ms, sheets that slide over 220 ms
// (base's motion), none of it under reduced motion.
import type { JSX } from "nsh";
import { Overlay, safeArea, signal, Text, View } from "nsh";
import {
  AlertDialog as BaseAlertDialog,
  Dialog as BaseDialog,
  HoverCard as BaseHoverCard,
  Popover as BasePopover,
  type Placement,
  type Align,
  Sheet as BaseSheet,
  Toast,
  type ToastEntry,
  ToastManager,
  ToastStack,
  Tooltip as BaseTooltip,
} from "nsh/base";
import { Button, IconButton } from "./button";
import { Glyph } from "./icons";
import { cx, ink, motion, shortcutLabel, surface, text } from "./theme";

/** A modal's title and description, as the dialog and the sheet draw them. */
function Heading(props: { title: string; description?: string }) {
  return (
    <View class="gap-1">
      <Text role="header" class={cx(text.lg, ink.bright, "font-semibold")}>
        {props.title}
      </Text>
      <If when={props.description}>
        <Text class={cx(text.md, ink.secondary)}>{props.description}</Text>
      </If>
    </View>
  );
}

export interface DialogProps {
  /** Shown (controlled). */
  open: boolean;
  /** The user closed it (Escape, back, the backdrop): write `open` back. */
  onOpenChange: (event: { open: boolean }) => void;
  title: string;
  description?: string;
  /** The body. */
  children?: JSX.Element;
  /** The actions, right-aligned under the body (`Button`s). */
  footer?: JSX.Element;
  /** A press on the scrim closes it (default true). */
  closeOnBackdropPress?: boolean;
  /** Escape closes it (default true), from a focused field in it too. */
  closeOnEscape?: boolean;
  /** `alertdialog` for one that asks the user to confirm. */
  role?: "dialog" | "alertdialog";
  testID?: string;
  class?: string;
}

/** A modal card over a scrim, square, 1pt border, outline actions. */
export function Dialog(props: DialogProps) {
  return (
    <BaseDialog
      open={props.open}
      onOpenChange={({ open }) => props.onOpenChange({ open })}
      animation="fade"
      role={props.role}
      accessibilityLabel={props.title}
      accessibilityDescription={props.description}
      closeOnBackdropPress={props.closeOnBackdropPress}
      closeOnEscape={props.closeOnEscape}
      testID={props.testID}
      class={cx("m-auto w-105 max-w-[90%] gap-4 p-4", surface.raised, props.class)}
      backdropClass={surface.scrim}
    >
      <Heading title={props.title} description={props.description} />
      {props.children}
      <If when={props.footer}>
        <View class="flex-row justify-end gap-2">{props.footer}</View>
      </If>
    </BaseDialog>
  );
}

export interface AlertDialogProps {
  open: boolean;
  onOpenChange: (event: { open: boolean }) => void;
  title: string;
  description?: string;
  /** The confirming action's label ("Delete"). */
  actionLabel: string;
  /** The cancelling action's label (default "Cancel"). */
  cancelLabel?: string;
  /** `danger` for a destructive action (default), `primary` otherwise. */
  tone?: "danger" | "primary";
  /** The user confirmed; the dialog closes. */
  onAction: () => void;
}

/**
 * A dialog that asks for a decision: only its buttons and Escape close it
 * (not the scrim), and screen readers announce it as an alert dialog.
 */
export function AlertDialog(props: AlertDialogProps) {
  const close = () => props.onOpenChange({ open: false });
  return (
    <Dialog
      open={props.open}
      onOpenChange={props.onOpenChange}
      title={props.title}
      description={props.description}
      role="alertdialog"
      closeOnBackdropPress={false}
      footer={
        <>
          <Button label={props.cancelLabel ?? "Cancel"} onPress={close} />
          <Button
            label={props.actionLabel}
            variant={props.tone === "primary" ? "primary" : "danger"}
            onPress={() => {
              close();
              props.onAction();
            }}
          />
        </>
      }
    />
  );
}

/**
 * Asks the platform's own alert (drawn by the system, not styled): resolves
 * `true` when the user chose `actionLabel`.
 */
export async function confirm(options: {
  title: string;
  description?: string;
  actionLabel: string;
  cancelLabel?: string;
  destructive?: boolean;
}): Promise<boolean> {
  const { action } = await BaseAlertDialog.show({
    title: options.title,
    description: options.description,
    actions: [
      { id: "cancel", label: options.cancelLabel ?? "Cancel", role: "cancel" },
      { id: "confirm", label: options.actionLabel, role: options.destructive ? "destructive" : "default" },
    ],
  });
  return action === "confirm";
}

const sheetEdge: Record<Placement, string> = {
  right: "w-80 max-w-full h-full border-l",
  left: "w-80 max-w-full h-full border-r",
  bottom: "w-full max-h-[80%] border-t",
  top: "w-full max-h-[80%] border-b",
};

export interface SheetProps {
  open: boolean;
  onOpenChange: (event: { open: boolean }) => void;
  title: string;
  description?: string;
  /** The edge it is on (default `right`). */
  side?: Placement;
  children?: JSX.Element;
  /** Actions at the bottom of the panel. */
  footer?: JSX.Element;
  /** The close button's label, for screen readers (default "Close"). */
  closeLabel?: string;
  testID?: string;
}

/** A panel at an edge of the window over a scrim, with a header and a close button. */
export function Sheet(props: SheetProps) {
  const side = () => props.side ?? "right";
  return (
    <BaseSheet
      open={props.open}
      onOpenChange={({ open }) => props.onOpenChange({ open })}
      side={side()}
      accessibilityLabel={props.title}
      testID={props.testID}
      popupClass={cx("bg-surface border-border gap-4 p-4", sheetEdge[side()])}
      backdropClass={surface.scrim}
    >
      <View class="flex-row items-start justify-between gap-4">
        <View class="flex-1">
          <Heading title={props.title} description={props.description} />
        </View>
        <IconButton accessibilityLabel={props.closeLabel ?? "Close"} size="sm" onPress={() => props.onOpenChange({ open: false })}>
          <Glyph name="cross" />
        </IconButton>
      </View>
      <View class="flex-1 gap-4">{props.children}</View>
      <If when={props.footer}>
        <View class="flex-row justify-end gap-2 border-t border-foreground/12 pt-4">{props.footer}</View>
      </If>
    </BaseSheet>
  );
}

/** A popup surface's classes. */
const popup = cx(surface.popup, motion.base);

export interface PopoverProps {
  /** What toggles it: a `Button` usually. */
  trigger: JSX.Element;
  /** The content, made on each opening. */
  children: JSX.Element;
  /** A bright heading above the content. */
  title?: string;
  open?: boolean;
  onOpenChange?: (event: { open: boolean }) => void;
  placement?: Placement;
  align?: Align;
  testID?: string;
  class?: string;
}

/** An anchored, non-modal surface: a press outside and Escape close it, the focus goes back to the trigger. */
export function Popover(props: PopoverProps) {
  return (
    <BasePopover
      trigger={props.trigger}
      open={props.open}
      onOpenChange={props.onOpenChange}
      placement={props.placement}
      align={props.align}
      offset={4}
      testID={props.testID}
      popupClass={cx(popup, "w-64 gap-2 p-3", text.md, ink.body, props.class)}
      popupStartingClass={motion.popupStarting}
      popupEndingClass={motion.popupEnding}
    >
      <If when={props.title}>
        <Text class={cx(text.md, ink.bright, "font-semibold")}>{props.title}</Text>
      </If>
      {props.children}
    </BasePopover>
  );
}

export interface TooltipProps {
  label: string;
  /** A shortcut after the label, in monospace. */
  shortcut?: string;
  placement?: Placement;
}

/**
 * A small square label next to a control, given to its `tooltip` prop:
 * `<IconButton tooltip={<Tooltip label="Save" shortcut="ctrl-s" />} …>`. It
 * shows after the pointer rests, at once with the keyboard focus, and hides
 * on Escape. Keep an accessible label on the control: a tooltip only adds.
 */
export function Tooltip(props: TooltipProps) {
  return (
    <BaseTooltip
      placement={props.placement}
      delay={400}
      class={cx("flex-row items-center gap-2 px-2 py-1 bg-popover border border-foreground/40", text.sm, ink.body, motion.fast)}
      startingClass="opacity-0"
      endingClass="opacity-0"
    >
      <Text>{props.label}</Text>
      <If when={props.shortcut}>
        <Text class={cx("font-mono", ink.secondary)}>{shortcutLabel(props.shortcut ?? "")}</Text>
      </If>
    </BaseTooltip>
  );
}

export interface HoverCardProps {
  /** What it previews; it keeps its own role and presses. */
  trigger: JSX.Element;
  children: JSX.Element;
  placement?: Placement;
  class?: string;
}

/**
 * A preview of what the trigger points at: it opens after the pointer rests
 * 400 ms and closes 200 ms after it left both. The pointer alone opens it, so
 * keep what a keyboard user needs reachable elsewhere.
 */
export function HoverCard(props: HoverCardProps) {
  return (
    <BaseHoverCard
      trigger={props.trigger}
      placement={props.placement}
      openDelay={400}
      closeDelay={200}
      offset={4}
      popupClass={cx(popup, "w-72 gap-2 p-3", text.md, ink.body, props.class)}
      popupStartingClass={motion.popupStarting}
      popupEndingClass={motion.popupEnding}
    >
      {props.children}
    </BaseHoverCard>
  );
}

// ---- Toasts ---------------------------------------------------------------

/** What a toast is about: its marker's color; an error stays until dismissed. */
export type ToastKind = "info" | "success" | "warning" | "error";

export interface ToastContent {
  title: string;
  description?: string;
  kind?: ToastKind;
  /** A button that runs `onAction`, then dismisses the toast. */
  action?: { label: string; onAction: () => void };
  /** Milliseconds it stays (default 6000; an error stays until dismissed). */
  duration?: number;
  /** Names it: a toast with the same id replaces it. */
  id?: string;
}

/** How many show at once. */
const LIMIT = 3;
/** How often the lifecycle advances while a toast is mounted. */
const TICK = 50;
/** A notice's time on screen. */
export const TOAST_TIMEOUT = 6000;

const manager = new ToastManager({ exitDuration: 140 });
const shown = signal<ToastEntry[]>([]);
const contents = signal<Record<string, ToastContent>>({});
const expanded = signal(false);
let ticking: ReturnType<typeof setInterval> | undefined;
let count = 0;

function tick() {
  const advanced = manager.advance(expanded());
  if (advanced.removed.length) {
    const next = { ...contents() };
    for (const gone of advanced.removed) delete next[gone.id];
    contents.set(next);
  }
  if (advanced.changed) shown.set(manager.visible(LIMIT));
  if (manager.isEmpty() && ticking !== undefined) {
    clearInterval(ticking);
    ticking = undefined;
  }
}

function run() {
  shown.set(manager.visible(LIMIT));
  ticking ??= setInterval(tick, TICK);
}

/**
 * Shows a notice in the app's `Toaster`, from anywhere: `toast({ title:
 * "Saved" })`. Returns its id, for `dismissToast`.
 */
export function toast(content: ToastContent): string {
  const id = content.id ?? `toast-${++count}`;
  contents.set({ ...contents(), [id]: content });
  const timeout = content.kind === "error" ? null : (content.duration ?? TOAST_TIMEOUT);
  manager.push(id, id, { timeout });
  run();
  return id;
}

/** Dismisses the toast `id`, or every toast without one. */
export function dismissToast(id?: string) {
  if (id === undefined) manager.dismissAll();
  else manager.dismiss(id);
  run();
}

const marker: Record<ToastKind, string> = {
  info: "bg-omarchy-accent",
  success: "bg-success",
  warning: "bg-warning",
  error: "bg-danger",
};

function ToastItem(props: { entry: ToastEntry }) {
  const content = () => contents()[props.entry.id] ?? { title: "" };
  return (
    <Toast
      transitionStatus={props.entry.status}
      accessibilityLabel={[content().title, content().description].filter(Boolean).join(". ")}
      onDismissRequest={() => dismissToast(props.entry.id)}
      class={cx("w-full flex-row items-start gap-3 py-3 pl-4 pr-2", surface.raised, motion.base)}
      startingClass="opacity-0 translate-y-1"
      endingClass="opacity-0"
      swipingClass="transition-none"
    >
      <View class={cx("absolute left-0 top-0 bottom-0 w-0.5", marker[content().kind ?? "info"])} />
      <View class="flex-1 gap-0.5">
        <Text class={cx(text.md, ink.bright, "font-semibold")}>{content().title}</Text>
        <If when={content().description}>
          <Text class={cx(text.sm, ink.secondary)}>{content().description}</Text>
        </If>
      </View>
      <If when={content().action}>
        <Button
          label={content().action?.label}
          variant="primary"
          size="sm"
          onPress={() => {
            content().action?.onAction();
            dismissToast(props.entry.id);
          }}
        />
      </If>
      <IconButton accessibilityLabel="Dismiss" size="sm" onPress={() => dismissToast(props.entry.id)}>
        <Glyph name="cross" />
      </IconButton>
    </Toast>
  );
}

/**
 * Where toasts show: the window's bottom right. Place it once, at the root,
 * after the content. The pointer on the stack expands it and pauses every
 * timeout; Escape dismisses the newest, a swipe right or its close button
 * one; screen readers read each as it shows.
 */
export function Toaster() {
  return (
    <Overlay
      style={{ pointerEvents: "box-none", left: 0, right: 0, bottom: 0, padding: 16, paddingBottom: safeArea().bottom + 16, alignItems: "flex-end" }}
    >
      <ToastStack
        placement="bottomRight"
        class="w-85 max-w-full"
        motion={{ duration: 220, exitDuration: 140, expandedGap: 8 }}
        onExpandedChange={({ expanded: open }) => expanded.set(open)}
        onCancel={() => {
          const newest = shown().filter((e) => e.status !== "ending").at(-1);
          if (newest) dismissToast(newest.id);
        }}
      >
        <Each of={shown()} key={(e: ToastEntry) => e.id}>
          {(entry) => <ToastItem entry={entry()} />}
        </Each>
      </ToastStack>
    </Overlay>
  );
}
