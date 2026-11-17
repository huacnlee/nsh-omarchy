// Checkboxes, switches, radios, sliders and progress bars.
import { signal, View } from "nsh";
import { Checkbox, Progress, Radio, RadioGroup, Slider, Switch } from "../../../tsx";
import { Row, Section, Status } from "./Section";

export function Choices() {
  const notify = signal(true);
  const sync = signal(false);
  const all = signal<boolean | "mixed">("mixed");
  const wifi = signal(true);
  const bluetooth = signal(false);
  const plan = signal<string | null>("pro");
  const volume = signal(40);
  const range = signal<[number, number]>([20, 80]);
  const progress = signal(64);
  return (
    <View class="gap-6">
      <Section title="Checkbox" description="A square box; a drawn check, a dash while indeterminate.">
        <Checkbox label="Send notifications" checked={notify()} onChange={({ checked }) => notify.set(checked)} />
        <Checkbox label="Sync on metered networks" checked={sync()} onChange={({ checked }) => sync.set(checked)} />
        <Checkbox
          label="Select all"
          checked={all() === true}
          indeterminate={all() === "mixed"}
          onChange={({ checked }) => all.set(checked)}
        />
        <Checkbox label="Managed by policy" checked disabled />
        <Status text={`Notifications ${notify() ? "on" : "off"} · sync ${sync() ? "on" : "off"}`} />
        <Status text={`All: ${all() === "mixed" ? "mixed" : all() ? "checked" : "unchecked"}`} />
      </Section>
      <Section title="Switch" description="A square track; the thumb slides over 140 ms.">
        <Switch label="Wi-Fi" checked={wifi()} onChange={({ checked }) => wifi.set(checked)} />
        <Switch label="Bluetooth" checked={bluetooth()} onChange={({ checked }) => bluetooth.set(checked)} />
        <Switch label="Airplane mode" checked={false} disabled />
        <Status text={`Wi-Fi ${wifi() ? "on" : "off"} · Bluetooth ${bluetooth() ? "on" : "off"}`} />
      </Section>
      <Section title="RadioGroup" description="One Tab stop; the arrows move and check.">
        <RadioGroup accessibilityLabel="Plan" value={plan()} onChange={({ value }) => plan.set(value)}>
          <Radio value="free" label="Free" />
          <Radio value="pro" label="Pro" />
          <Radio value="team" label="Team" />
          <Radio value="enterprise" label="Enterprise" disabled />
        </RadioGroup>
        <Status text={`Plan: ${plan() ?? "none"}`} />
      </Section>
      <Section title="Slider" description="An inset rail, an accent range, a square thumb.">
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
          format={(v) => `$${v}`}
          onChange={({ value }) => range.set(value as [number, number])}
          class="w-80"
        />
        <Slider accessibilityLabel="Locked" value={30} disabled class="w-80" />
      </Section>
      <Section title="Progress" description="A thin inset track with an accent fill.">
        <Progress label="Downloading" value={progress()} showValue class="w-80" />
        <Row>
          <Progress label="Installing" indeterminate class="w-80" />
        </Row>
        <Progress label="Disk" value={92} tone="warning" showValue class="w-80" />
        <Row>
          <Slider
            accessibilityLabel="Download progress"
            value={progress()}
            onChange={({ value }) => progress.set(value as number)}
            class="w-80"
          />
        </Row>
      </Section>
    </View>
  );
}
