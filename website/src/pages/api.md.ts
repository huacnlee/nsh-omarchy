import source from '../../../docs/api.md?raw';

// The API reference as plain Markdown, for the Download link and for tools.
export function GET() {
  return new Response(source, { headers: { 'Content-Type': 'text/markdown; charset=utf-8' } });
}
