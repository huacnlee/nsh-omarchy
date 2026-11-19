// The type ramp, cards, panels, badges, keycaps, separators, avatars, icons,
// empty states and Markdown.
import { signal } from "nsh";
import {
  Avatar,
  Badge,
  Body,
  Button,
  Caption,
  Card,
  Code,
  EmptyState,
  Heading,
  Icon,
  Keycap,
  Markdown,
  Omarchy,
  Panel,
  Separator,
  Shortcut,
  Title,
} from "nsh-omarchy";
import { Demos, type Family, Row, Section, Status } from "./Section";

const notes = `# Release notes

Omarchy's **Markdown** view sets documents in the compact type scale, with \`inline code\`, *emphasis* and [links](https://omarchy.org).

- Square code blocks on the inset surface
- [x] Theme tokens

\`\`\`sh
omarchy-theme-set tokyo-night
\`\`\`
`;

function TypographyPage() {
  return (
    <Demos>
      <Section title="The type ramp" description="Heading, Title, Body and Caption: 20 / 14 / 12 / 11 points.">
        <Heading>Heading</Heading>
        <Title>Title</Title>
        <Body>Body text for paragraphs and labels.</Body>
        <Caption>Caption for hints and metadata.</Caption>
      </Section>
      <Section title="Code" description="Monospace on the inset surface.">
        <Row>
          <Body>Run</Body>
          <Code>omarchy-theme-set tokyo-night</Code>
        </Row>
      </Section>
    </Demos>
  );
}

function CardPage() {
  return (
    <Demos>
      <Section title="Card" description="A title, a description, content and a footer of actions.">
        <Card
          title="Updates"
          description="3 packages can be upgraded."
          class="w-72"
          footer={<Button label="Update" variant="primary" size="small" />}
        >
          <Caption>Last checked 5 minutes ago.</Caption>
        </Card>
      </Section>
    </Demos>
  );
}

function PanelPage() {
  return (
    <Demos>
      <Section title="Panel" description="A titled region, filled or outlined.">
        <Row>
          <Panel title="Workspace" class="w-48">
            <Body>Filled panel</Body>
          </Panel>
          <Panel title="Outline" variant="outline" class="w-48">
            <Body>Outlined panel</Body>
          </Panel>
        </Row>
      </Section>
    </Demos>
  );
}

function BadgePage() {
  return (
    <Demos>
      <Section title="Badge" description="A short uppercase label; filled, outlined, small.">
        <Row>
          <Badge label="Neutral" />
          <Badge label="New" variant="primary" />
          <Badge label="Online" variant="success" />
          <Badge label="Beta" variant="warning" />
          <Badge label="Failed" variant="danger" />
          <Badge label="Outline" variant="primary" outline />
          <Badge label="Small" variant="success" outline size="small" class="self-center" />
        </Row>
      </Section>
    </Demos>
  );
}

function KeycapPage() {
  return (
    <Demos>
      <Section title="Keycap and Shortcut" description="Keys in monospace caps; a shortcut is a row of them.">
        <Row>
          <Keycap key="esc" />
          <Shortcut keys="super-shift-enter" />
          <Caption>Open a terminal</Caption>
        </Row>
        <Caption>{`Save is ${Omarchy.shortcutLabel("mod-s")} here`}</Caption>
      </Section>
    </Demos>
  );
}

function SeparatorPage() {
  return (
    <Demos>
      <Section title="Separator" description="A hairline between groups, across or along.">
        <Body>Above the line</Body>
        <Separator />
        <Row>
          <Body>Left</Body>
          <Separator layout="vertical" class="h-5" />
          <Body>Right</Body>
        </Row>
      </Section>
    </Demos>
  );
}

function AvatarPage() {
  return (
    <Demos>
      <Section title="Avatar" description="Initials on a square; small, medium and large.">
        <Row>
          <Avatar name="Ada Lovelace" size="small" />
          <Avatar name="Grace Hopper" />
          <Avatar name="Linus Torvalds" size="large" />
        </Row>
      </Section>
    </Demos>
  );
}

function IconPage() {
  return (
    <Demos>
      <Section title="Icon" description="Lucide glyphs by name, in the text color; a label makes one an image.">
        <Row>
          <Icon name="folder" class="text-text-secondary" />
          <Icon name="circle-check" class="text-success" />
          <Icon name="lock" label="Encrypted" size={24} class="text-foreground" />
        </Row>
      </Section>
    </Demos>
  );
}

function EmptyStatePage() {
  return (
    <Demos>
      <Section title="EmptyState" description="A glyph, a title, a hint and an action, centered.">
        <EmptyState title="No windows" description="Open an app with super-space." icon="inbox">
          <Button label="Open launcher" variant="primary" />
        </EmptyState>
      </Section>
    </Demos>
  );
}

function MarkdownPage() {
  const link = signal("");
  return (
    <Demos>
      <Section title="Markdown" description="A document in the type ramp; links are reported, not opened.">
        <Markdown source={notes} onLinkPress={({ value }) => link.set(value)} />
        <Status text={link() ? `Link pressed: ${link()}` : "Press a link"} />
      </Section>
    </Demos>
  );
}

export const display: Family = {
  title: "Display",
  pages: [
    { value: "typography", title: "Typography", page: TypographyPage },
    { value: "card", title: "Card", page: CardPage },
    { value: "panel", title: "Panel", page: PanelPage },
    { value: "badge", title: "Badge", page: BadgePage },
    { value: "keycap", title: "Keycap", page: KeycapPage },
    { value: "separator", title: "Separator", page: SeparatorPage },
    { value: "avatar", title: "Avatar", page: AvatarPage },
    { value: "icon", title: "Icon", page: IconPage },
    { value: "empty-state", title: "EmptyState", page: EmptyStatePage },
    { value: "markdown", title: "Markdown", page: MarkdownPage },
  ],
};
