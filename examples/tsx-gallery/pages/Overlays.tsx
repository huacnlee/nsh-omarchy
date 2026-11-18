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
  Field,
  HoverCard,
  Input,
  Popover,
  Sheet,
  Switch,
  Title,
  toast,
  Tooltip,
} from "../../../tsx";
import { Row, Section, Status } from "./Section";

export function Overlays() {
  const dialog = signal(false);
  const alert = signal(false);
  const sheet = signal(false);
  const name = signal("Workspace 1");
  const last = signal("nothing");
  const dnd = signal(false);
  return (
    <View class="gap-6">
      <Section title="Dialog and AlertDialog" description="A square card over the scrim; the alert only closes by its buttons or Escape.">
        <Row>
          <Button label="Rename workspace" onPress={() => dialog.set(true)} />
          <Button label="Delete workspace" variant="danger" onPress={() => alert.set(true)} />
          <Button
            label="Native confirm"
            variant="ghost"
            onPress={async () => last.set((await confirm({ title: "Restart Hyprland?", actionLabel: "Restart" })) ? "Restarted" : "Kept")}
          />
        </Row>
        <Status text={`Last: ${last()}`} />
        <Dialog
          open={dialog()}
          onOpenChange={({ open }) => dialog.set(open)}
          title="Rename workspace"
          description="Shown in the bar and the switcher."
          footer={
            <>
              <Button label="Cancel" onPress={() => dialog.set(false)} />
              <Button
                label="Save"
                variant="primary"
                onPress={() => {
                  last.set(`Renamed to ${name()}`);
                  dialog.set(false);
                }}
              />
            </>
          }
        >
          <Field label="Name">
            <Input accessibilityLabel="Workspace name" value={name()} onChange={({ value }) => name.set(value)} autoFocus />
          </Field>
        </Dialog>
        <AlertDialog
          open={alert()}
          onOpenChange={({ open }) => alert.set(open)}
          title="Delete workspace?"
          description="Its windows move to workspace 1. This cannot be undone."
          actionLabel="Delete"
          onAction={() => last.set("Deleted")}
        />
      </Section>
      <Section title="Sheet" description="A panel at the window's edge, with a header, a close button and actions.">
        <Button label="Open settings" onPress={() => sheet.set(true)} />
        <Sheet
          open={sheet()}
          onOpenChange={({ open }) => sheet.set(open)}
          title="Quick settings"
          description="Applies at once."
          footer={<Button label="Done" variant="primary" onPress={() => sheet.set(false)} />}
        >
          <Switch label="Do not disturb" checked={dnd()} onChange={({ checked }) => dnd.set(checked)} />
        </Sheet>
      </Section>
      <Section title="Popover, Tooltip and HoverCard">
        <Row>
          <Popover trigger={<Button label="Details" />} title="Battery">
            <Body>82%, about 5 hours left.</Body>
          </Popover>
          <Button label="Hover me" variant="ghost" tooltip={<Tooltip label="Lock screen" shortcut="super-esc" />} />
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
      <Section title="Toast" description="Bottom right, 6 s; errors stay until dismissed.">
        <Row>
          <Button label="Notify" onPress={() => toast({ title: "Screenshot saved", description: "~/Pictures/shot.png" })} />
          <Button
            label="With action"
            onPress={() =>
              toast({ title: "Window closed", kind: "success", action: { label: "Undo", onAction: () => last.set("Undone") } })
            }
          />
          <Button label="Warning" onPress={() => toast({ title: "Battery low", kind: "warning" })} />
          <Button label="Error" variant="danger" onPress={() => toast({ title: "Update failed", description: "No network.", kind: "error" })} />
        </Row>
      </Section>
    </View>
  );
}
