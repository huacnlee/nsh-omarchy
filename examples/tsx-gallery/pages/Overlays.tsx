// Dialogs, sheets, popovers, tooltips, hover cards and toasts.
import { signal, View } from "nsh";
import {
  AlertDialog,
  Avatar,
  Body,
  Button,
  Caption,
  confirm,
  Dialog,
  dismissToast,
  Field,
  HoverCard,
  Input,
  Popover,
  Sheet,
  Switch,
  Title,
  toast,
  Tooltip,
} from "nsh-omarchy";
import { Demos, type Family, Row, Section, Status } from "./Section";

function DialogPage() {
  const open = signal(false);
  const name = signal("Workspace 1");
  const last = signal("nothing");
  return (
    <Demos>
      <Section title="Dialog" description="A square card over the scrim; Escape or a press outside closes it.">
        <Row>
          <Button label="Rename workspace" onPress={() => open.set(true)} />
        </Row>
        <Status text={`Last: ${last()}`} />
        <Dialog
          open={open()}
          onOpenChange={({ open: next }) => open.set(next)}
          title="Rename workspace"
          description="Shown in the bar and the switcher."
          footer={
            <>
              <Button label="Cancel" onPress={() => open.set(false)} />
              <Button
                label="Save"
                variant="primary"
                onPress={() => {
                  last.set(`Renamed to ${name()}`);
                  open.set(false);
                }}
              />
            </>
          }
        >
          <Field label="Name">
            <Input accessibilityLabel="Workspace name" value={name()} onChange={({ value }) => name.set(value)} autoFocus />
          </Field>
        </Dialog>
      </Section>
    </Demos>
  );
}

function AlertDialogPage() {
  const open = signal(false);
  const last = signal("nothing");
  return (
    <Demos>
      <Section title="AlertDialog" description="Closes only by its buttons or Escape.">
        <Row>
          <Button label="Delete workspace" variant="danger" onPress={() => open.set(true)} />
        </Row>
        <Status text={`Last: ${last()}`} />
        <AlertDialog
          open={open()}
          onOpenChange={({ open: next }) => open.set(next)}
          title="Delete workspace?"
          description="Its windows move to workspace 1. This cannot be undone."
          actionLabel="Delete"
          tone="danger"
          onAction={() => last.set("Deleted")}
        />
      </Section>
      <Section title="confirm()" description="The same alert as an awaited call.">
        <Row>
          <Button
            label="Native confirm"
            variant="ghost"
            onPress={async () =>
              last.set((await confirm({ title: "Restart Hyprland?", actionLabel: "Restart" })) ? "Restarted" : "Kept")
            }
          />
          <Button
            label="Danger confirm"
            variant="ghost"
            onPress={async () =>
              last.set(
                (await confirm({ title: "Delete 3 files?", actionLabel: "Delete", tone: "danger" })) ? "Files deleted" : "Files kept",
              )
            }
          />
        </Row>
      </Section>
    </Demos>
  );
}

function SheetPage() {
  const open = signal(false);
  const dnd = signal(false);
  return (
    <Demos>
      <Section title="Sheet" description="A panel at the window's edge, with a header, a close button and actions.">
        <Row>
          <Button label="Open settings" onPress={() => open.set(true)} />
        </Row>
        <Sheet
          open={open()}
          onOpenChange={({ open: next }) => open.set(next)}
          title="Quick settings"
          description="Applies at once."
          footer={<Button label="Done" variant="primary" onPress={() => open.set(false)} />}
        >
          <Switch label="Do not disturb" checked={dnd()} onChange={({ checked }) => dnd.set(checked)} />
        </Sheet>
      </Section>
    </Demos>
  );
}

function PopoverPage() {
  return (
    <Demos>
      <Section title="Popover" description="A card under its trigger; Escape or a press outside closes it.">
        <Row>
          <Popover trigger={<Button label="Details" />} title="Battery">
            <Body>82%, about 5 hours left.</Body>
          </Popover>
        </Row>
      </Section>
    </Demos>
  );
}

function TooltipPage() {
  return (
    <Demos>
      <Section title="Tooltip" description="A label, and a shortcut in keycaps, after a hover.">
        <Row>
          <Button label="Hover me" variant="ghost" tooltip={<Tooltip label="Lock screen" shortcut="super-esc" />} />
        </Row>
      </Section>
    </Demos>
  );
}

function HoverCardPage() {
  return (
    <Demos>
      <Section title="HoverCard" description="A preview card that opens while the pointer rests on its trigger.">
        <Row>
          <HoverCard trigger={<Button label="@dhh" variant="ghost" />}>
            <View class="flex-row gap-3">
              <Avatar name="David Heinemeier Hansson" />
              <View class="flex-1 gap-1">
                <Title>DHH</Title>
                <Caption>Creator of Omarchy.</Caption>
              </View>
            </View>
          </HoverCard>
        </Row>
      </Section>
    </Demos>
  );
}

function ToastPage() {
  const last = signal("nothing");
  const lastToast = signal("");
  return (
    <Demos>
      <Section title="Toast" description="Bottom right, 6 s; errors stay until dismissed.">
        <Row>
          <Button label="Notify" onPress={() => toast({ title: "Screenshot saved", description: "~/Pictures/shot.png" })} />
          <Button
            label="With action"
            onPress={() =>
              lastToast.set(
                toast({ title: "Window closed", kind: "success", action: { label: "Undo", onAction: () => last.set("Undone") } }),
              )
            }
          />
          <Button label="Warning" onPress={() => toast({ title: "Battery low", kind: "warning" })} />
          <Button label="Error" variant="danger" onPress={() => toast({ title: "Update failed", description: "No network.", kind: "error" })} />
        </Row>
        <Status text={`Last: ${last()}`} />
      </Section>
      <Section title="By id" description="dismissToast() removes a toast; a toast with the same id replaces it.">
        <Row>
          <Button label="Dismiss last" variant="ghost" onPress={() => dismissToast(lastToast())} />
          <Button
            label="Progress"
            onPress={() => {
              toast({ id: "progress", title: "Step 1 of 2", timeout: null });
              toast({ id: "progress", title: "Step 2 of 2", kind: "success", timeout: 2000 });
            }}
          />
        </Row>
      </Section>
    </Demos>
  );
}

export const overlays: Family = {
  title: "Overlays",
  pages: [
    { value: "dialog", title: "Dialog", page: DialogPage },
    { value: "alert-dialog", title: "AlertDialog", page: AlertDialogPage },
    { value: "sheet", title: "Sheet", page: SheetPage },
    { value: "popover", title: "Popover", page: PopoverPage },
    { value: "tooltip", title: "Tooltip", page: TooltipPage },
    { value: "hover-card", title: "HoverCard", page: HoverCardPage },
    { value: "toast", title: "Toast", page: ToastPage },
  ],
};
