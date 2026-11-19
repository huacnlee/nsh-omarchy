// What every gallery page is made of: sections of demos on cards, and the
// page list each family file exports.
import type { JSX } from "nsh";
import { Text, View } from "nsh";
import { Caption, Title } from "nsh-omarchy";

/** One gallery page: a component, its sidebar row and its demos. */
export type GalleryPage = {
  /** A stable id: the sidebar's value and the row's test id (`page-<value>`). */
  value: string;
  /** The sidebar label and the page heading: the component's name. */
  title: string;
  page: () => JSX.Element;
};

/** A titled group of pages in the sidebar. */
export type Family = { title: string; pages: GalleryPage[] };

/** A page's stack of sections. */
export function Demos(props: { children: JSX.Element }) {
  return <View class="gap-6">{props.children}</View>;
}

/** A gallery section: a title, a line on what it shows, and the demo on a card. */
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

/** A wrapping row of demos. */
export function Row(props: { children: JSX.Element }) {
  return <View class="flex-row flex-wrap items-center gap-2">{props.children}</View>;
}

/** What the last action did, for the reader (and the tests). */
export function Status(props: { text: string }) {
  return <Text class="text-[11px] text-text-secondary">{props.text}</Text>;
}
