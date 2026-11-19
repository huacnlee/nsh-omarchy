import type { APIContext } from 'astro';
import { publicUrl } from '../lib/urls';

export function GET({ site }: APIContext) {
  const repo = 'https://github.com/huacnlee/nsh-omarchy/blob/main';
  return new Response(`# NSH Omarchy

> Omarchy-styled components for Native Shell (NSH) apps, in Rust and TSX: one nshm crate, nsh-omarchy.

## Documentation

- [API reference](${publicUrl(site, 'api.md')}): Every component's constructor, props, shorthands and events, in Rust and TSX. Frozen from 0.1.0. Read it before writing code.
- [README](${repo}/README.md): Installing with \`nsh add nsh-omarchy\`, a Rust app and a TSX app, theme classes.

## Source and runnable examples

- [Public exports](${repo}/src/lib.rs): The exact exported names.
- [TSX gallery](${repo}/examples/tsx-gallery/App.tsx): Every component, imported from "nsh-omarchy".
- [Rust gallery](${repo}/examples/gallery/src/lib.rs): Every component, from Rust.

## Framework reference

- [NSH documentation](https://nsh.dev/llms.txt): Components, reactivity, styling and tooling of Native Shell.
`, { headers: { 'Content-Type': 'text/plain; charset=utf-8' } });
}
