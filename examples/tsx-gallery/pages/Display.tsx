// Cards, panels, badges, keycaps, empty states, avatars, icons, the type
// ramp, Markdown and the charts.
import { signal, View } from "nsh";
import {
  AreaChart,
  Avatar,
  Badge,
  BarChart,
  Body,
  Button,
  Caption,
  Card,
  Code,
  EmptyState,
  Heading,
  Icon,
  Keycap,
  LineChart,
  Markdown,
  Omarchy,
  Panel,
  PieChart,
  Separator,
  Shortcut,
  Title,
} from "nsh-omarchy";
import { Row, Section, Status } from "./Section";

const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const desktop = [186, 305, 237, 73, 209, 214];
const mobile = [80, 200, 120, 190, 130, 140];
const visits = months.map((label, i) => ({ label, value: desktop[i] }));
const browsers = [
  { label: "Chrome", value: 275 },
  { label: "Safari", value: 200 },
  { label: "Firefox", value: 187 },
  { label: "Edge", value: 173 },
  { label: "Other", value: 90 },
];

const notes = `# Release notes

Omarchy's **Markdown** view sets documents in the compact type scale, with \`inline code\`, *emphasis* and [links](https://omarchy.org).

- Square code blocks on the inset surface
- [x] Theme tokens

\`\`\`sh
omarchy-theme-set tokyo-night
\`\`\`
`;

export function Display() {
  const link = signal("");
  return (
    <View class="gap-6">
      <Section title="Typography" description="20 / 14 / 12 / 11 points; monospace for code.">
        <Heading>Heading</Heading>
        <Title>Title</Title>
        <Body>Body text for paragraphs and labels.</Body>
        <Caption>Caption for hints and metadata.</Caption>
        <Row>
          <Body>Run</Body>
          <Code>omarchy-theme-set tokyo-night</Code>
        </Row>
      </Section>
      <Section title="Card and Panel">
        <Row>
          <Card
            title="Updates"
            description="3 packages can be upgraded."
            class="w-72"
            footer={<Button label="Update" variant="primary" size="small" />}
          >
            <Caption>Last checked 5 minutes ago.</Caption>
          </Card>
          <Panel title="Workspace" class="w-48">
            <Body>Filled panel</Body>
          </Panel>
          <Panel title="Outline" variant="outline" class="w-48">
            <Body>Outlined panel</Body>
          </Panel>
        </Row>
      </Section>
      <Section title="Badge, Keycap and Separator">
        <Row>
          <Badge label="Neutral" />
          <Badge label="New" variant="primary" />
          <Badge label="Online" variant="success" />
          <Badge label="Beta" variant="warning" />
          <Badge label="Failed" variant="danger" />
          <Badge label="Outline" variant="primary" outline />
          <Badge label="Small" variant="success" outline size="small" class="self-center" />
        </Row>
        <Separator />
        <Row>
          <Keycap key="esc" />
          <Shortcut keys="super-shift-enter" />
          <Separator layout="vertical" class="h-5" />
          <Caption>Open a terminal</Caption>
        </Row>
        <Caption>{`Save is ${Omarchy.shortcutLabel("mod-s")} here`}</Caption>
      </Section>
      <Section title="Avatar and Icon">
        <Row>
          <Avatar name="Ada Lovelace" size="small" />
          <Avatar name="Grace Hopper" />
          <Avatar name="Linus Torvalds" size="large" />
          <Icon name="folder" class="text-text-secondary" />
          <Icon name="circle-check" class="text-success" />
          <Icon name="lock" label="Encrypted" size={24} class="text-foreground" />
        </Row>
      </Section>
      <Section title="EmptyState">
        <EmptyState title="No windows" description="Open an app with super-space." icon="inbox">
          <Button label="Open launcher" variant="primary" />
        </EmptyState>
      </Section>
      <Section title="Markdown" description="A document in the type ramp; links are reported, not opened.">
        <Markdown source={notes} onLinkPress={({ value }) => link.set(value)} />
        <Status text={link() ? `Link pressed: ${link()}` : "Press a link"} />
      </Section>
      <Section title="Charts" description="Tab to a chart; Left and Right step the points.">
        <LineChart
          data={visits}
          name="Desktop"
          series={[{ name: "Mobile", values: mobile }]}
          curve="natural"
          height={160}
          accessibilityLabel="Visitors, January to June"
        />
        <Row>
          <View class="w-[360px]">
            <BarChart data={visits} name="Desktop" series={[{ name: "Mobile", values: mobile }]} stacked height={140} />
          </View>
          <View class="w-[360px]">
            <AreaChart data={visits} name="Desktop" height={140} unit=" visits" />
          </View>
        </Row>
        <PieChart data={browsers} donut={0.6} size={140} accessibilityLabel="Visitors by browser" />
      </Section>
    </View>
  );
}
