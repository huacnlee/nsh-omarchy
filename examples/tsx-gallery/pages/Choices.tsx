// Checkboxes, switches, radios, sliders and progress bars.
import { signal } from "nsh";
import { Checkbox, Progress, Radio, RadioGroup, Slider, Switch } from "nsh-omarchy";
import { Demos, type Family, Section, Status } from "./Section";

function CheckboxPage() {
  const notify = signal(true);
  const sync = signal(false);
  const all = signal<boolean | "mixed">("mixed");
  const kept = signal("untouched");
  return (
    <Demos>
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
      <Section title="Uncontrolled" description="Left without `checked`, the box keeps its own value.">
        <Checkbox label="Keeps its own value" onChange={({ checked }) => kept.set(checked ? "on" : "off")} />
        <Status text={`Kept: ${kept()}`} />
      </Section>
    </Demos>
  );
}

function SwitchPage() {
  const wifi = signal(true);
  const bluetooth = signal(false);
  return (
    <Demos>
      <Section title="Switch" description="A square track; the thumb slides over 140 ms.">
        <Switch label="Wi-Fi" checked={wifi()} onChange={({ checked }) => wifi.set(checked)} />
        <Switch label="Bluetooth" checked={bluetooth()} onChange={({ checked }) => bluetooth.set(checked)} />
        <Switch label="Airplane mode" checked={false} disabled />
        <Status text={`Wi-Fi ${wifi() ? "on" : "off"} · Bluetooth ${bluetooth() ? "on" : "off"}`} />
      </Section>
    </Demos>
  );
}

function RadioGroupPage() {
  const plan = signal<string>("pro");
  return (
    <Demos>
      <Section title="Vertical" description="One Tab stop; the arrows move and check.">
        <RadioGroup accessibilityLabel="Plan" value={plan()} onChange={({ value }) => plan.set(value)}>
          <Radio value="free" label="Free" />
          <Radio value="pro" label="Pro" />
          <Radio value="team" label="Team" />
          <Radio value="enterprise" label="Enterprise" disabled />
        </RadioGroup>
        <Status text={`Plan: ${plan()}`} />
      </Section>
      <Section title="Horizontal">
        <RadioGroup accessibilityLabel="Density" layout="horizontal">
          <Radio value="compact" label="Compact" />
          <Radio value="roomy" label="Roomy" />
        </RadioGroup>
      </Section>
    </Demos>
  );
}

function SliderPage() {
  const volume = signal(40);
  const range = signal<[number, number]>([20, 80]);
  return (
    <Demos>
      <Section title="Slider" description="An inset rail, an accent range, a square thumb.">
        <Slider
          accessibilityLabel="Volume"
          value={volume()}
          showValue
          onChange={({ value }) => volume.set(value as number)}
          class="w-80"
        />
        <Slider accessibilityLabel="Locked" value={30} disabled class="w-80" />
      </Section>
      <Section title="Range" description="Two thumbs for a start and an end.">
        <Slider
          accessibilityLabel="Price range"
          value={range()}
          showValue
          onChange={({ value }) => range.set(value as [number, number])}
          class="w-80"
        />
      </Section>
    </Demos>
  );
}

function ProgressPage() {
  const progress = signal(64);
  return (
    <Demos>
      <Section title="Progress" description="A thin inset track with a toned fill.">
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
      </Section>
    </Demos>
  );
}

export const choices: Family = {
  title: "Choices",
  pages: [
    { value: "checkbox", title: "Checkbox", page: CheckboxPage },
    { value: "switch", title: "Switch", page: SwitchPage },
    { value: "radio-group", title: "RadioGroup", page: RadioGroupPage },
    { value: "slider", title: "Slider", page: SliderPage },
    { value: "progress", title: "Progress", page: ProgressPage },
  ],
};
