// Cards, panels, badges, keycaps, empty states, avatars and the type scale.
import { View } from "nsh";
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
  Keycap,
  Panel,
  Separator,
  Shortcut,
  Title,
} from "../../../tsx";
import { Row, Section } from "./Section";

export function Display() {
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
            footer={<Button label="Update" variant="primary" size="sm" />}
          >
            <Caption>Last checked 5 minutes ago.</Caption>
          </Card>
          <Panel class="w-48">
            <Body>Raised panel</Body>
          </Panel>
          <Panel variant="inset" class="w-48">
            <Body>Inset panel</Body>
          </Panel>
        </Row>
      </Section>
      <Section title="Badge, Keycap and Separator">
        <Row>
          <Badge label="Neutral" />
          <Badge label="New" tone="accent" />
          <Badge label="Online" tone="success" />
          <Badge label="Beta" tone="warning" />
          <Badge label="Failed" tone="danger" />
          <Badge label="Solid" tone="accent" solid />
        </Row>
        <Separator />
        <Row>
          <Keycap label="esc" />
          <Shortcut keys={["super", "shift", "enter"]} />
          <Separator orientation="vertical" class="h-5" />
          <Caption>Open a terminal</Caption>
        </Row>
      </Section>
      <Section title="Avatar">
        <Row>
          <Avatar name="Ada Lovelace" size="sm" />
          <Avatar name="Grace Hopper" />
          <Avatar name="Linus Torvalds" size="lg" />
        </Row>
      </Section>
      <Section title="EmptyState">
        <EmptyState
          title="No windows"
          description="Open an app with super-space."
          action={<Button label="Open launcher" variant="primary" />}
        />
      </Section>
    </View>
  );
}
