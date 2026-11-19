// Checkboxes, switches, radios, sliders and progress bars.
import type { JSX } from "nsh";
import { signal, Text, View } from "nsh";
import { Checkbox, Progress, Radio, RadioGroup, Slider, Switch } from "nsh-omarchy";

/** A demo block: a title, a line on what it shows, and the demo on a card. */
function Block(props: { title: string; description: string; children: JSX.Element }) {
  return (
    <View class="gap-2">
      <View class="gap-0.5">
        <Text class="text-[14px] font-semibold text-bright">{props.title}</Text>
        <Text class="text-[11px] text-text-secondary">{props.description}</Text>
      </View>
      <View class="gap-3 border border-border bg-surface p-4">{props.children}</View>
    </View>
  );
}

/** What the demos report, for the reader (and the tests). */
function Status(props: { text: string }) {
  return <Text class="text-[11px] text-text-secondary">{props.text}</Text>;
}

export function Choices() {
  const notify = signal(true);
  const sync = signal(false);
  const all = signal<boolean | "mixed">("mixed");
  const wifi = signal(true);
  const bluetooth = signal(false);
  const plan = signal<string>("pro");
  const volume = signal(40);
  const range = signal<[number, number]>([20, 80]);
  const progress = signal(64);
  const kept = signal("untouched");
  return (
    <View class="gap-6">
      <Block title="Checkbox" description="A square box; a drawn check, a dash while indeterminate.">
        <Checkbox label="Send notifications" checked={notify()} onChange={({ checked }) => notify.set(checked)} />
        <Checkbox label="Sync on metered networks" checked={sync()} onChange={({ checked }) => sync.set(checked)} />
        <Checkbox
          label="Select all"
          checked={all() === true}
          indeterminate={all() === "mixed"}
          onChange={({ checked }) => all.set(checked)}
        />
        <Checkbox label="Keeps its own value" onChange={({ checked }) => kept.set(checked ? "on" : "off")} />
        <Checkbox label="Managed by policy" checked disabled />
        <Status text={`Notifications ${notify() ? "on" : "off"} · sync ${sync() ? "on" : "off"}`} />
        <Status text={`Kept: ${kept()}`} />
        <Status text={`All: ${all() === "mixed" ? "mixed" : all() ? "checked" : "unchecked"}`} />
      </Block>
      <Block title="Switch" description="A square track; the thumb slides over 140 ms.">
        <Switch label="Wi-Fi" checked={wifi()} onChange={({ checked }) => wifi.set(checked)} />
        <Switch label="Bluetooth" checked={bluetooth()} onChange={({ checked }) => bluetooth.set(checked)} />
        <Switch label="Airplane mode" checked={false} disabled />
        <Status text={`Wi-Fi ${wifi() ? "on" : "off"} · Bluetooth ${bluetooth() ? "on" : "off"}`} />
      </Block>
      <Block title="RadioGroup" description="One Tab stop; the arrows move and check.">
        <RadioGroup accessibilityLabel="Plan" value={plan()} onChange={({ value }) => plan.set(value)}>
          <Radio value="free" label="Free" />
          <Radio value="pro" label="Pro" />
          <Radio value="team" label="Team" />
          <Radio value="enterprise" label="Enterprise" disabled />
        </RadioGroup>
        <RadioGroup accessibilityLabel="Density" layout="horizontal">
          <Radio value="compact" label="Compact" />
          <Radio value="roomy" label="Roomy" />
        </RadioGroup>
        <Status text={`Plan: ${plan()}`} />
      </Block>
      <Block title="Slider" description="An inset rail, an accent range, a square thumb.">
        <Slider
          accessibilityLabel="Volume"
          value={volume()}
          showValue
          onChange={({ value }) => volume.set(value as number)}
          class="w-80"
        />
        <Slider
          accessibilityLabel="Price range"
          value={range()}
          showValue
          onChange={({ value }) => range.set(value as [number, number])}
          class="w-80"
        />
        <Slider accessibilityLabel="Locked" value={30} disabled class="w-80" />
      </Block>
      <Block title="Progress" description="A thin inset track with a toned fill.">
        <Progress label="Downloading" value={progress()} showValue class="w-80" />
        <Progress label="Installing" indeterminate class="w-80" />
        <Progress label="Disk" value={92} tone="warning" showValue class="w-80" />
        <Progress label="Backup" value={100} tone="success" showValue class="w-80" />
        <Slider
          accessibilityLabel="Download progress"
          value={progress()}
          onChange={({ value }) => progress.set(value as number)}
          class="w-80"
        />
      </Block>
    </View>
  );
}
