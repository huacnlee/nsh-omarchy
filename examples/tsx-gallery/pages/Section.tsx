// A gallery section: a title, a line on what it shows, and the demo on a card.
import type { JSX } from "nsh";
import { Text, View } from "nsh";
import { Caption, Title } from "../../../tsx";

export function Section(props: { title: string; description?: string; children: JSX.Element }) {
  return (
    <View class="gap-2">
      <View class="gap-0.5">
        <Title>{props.title}</Title>
        <If when={props.description}>
          <Caption>{props.description}</Caption>
        </If>
      </View>
      <View class="gap-3 border border-border bg-surface p-4">{props.children}</View>
    </View>
  );
}

/** A labelled row of demos. */
export function Row(props: { children: JSX.Element }) {
  return <View class="flex-row flex-wrap items-center gap-2">{props.children}</View>;
}

/** What the last action did, for the reader (and the tests). */
export function Status(props: { text: string }) {
  return <Text class="text-[11px] text-text-secondary">{props.text}</Text>;
}
