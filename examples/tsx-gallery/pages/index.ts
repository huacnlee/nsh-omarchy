// Every gallery page, by family, in sidebar order: the README's families.
import { actions } from "./Actions";
import { charts } from "./Charts";
import { choices } from "./Choices";
import { data } from "./Data";
import { display } from "./Display";
import { fields } from "./Fields";
import { navigation } from "./Navigation";
import { overlays } from "./Overlays";
import type { Family, GalleryPage } from "./Section";

export const families: Family[] = [actions, choices, fields, navigation, overlays, data, display, charts];

/** Every page, flat, in sidebar order. */
export const pages: GalleryPage[] = families.flatMap((family) => family.pages);
