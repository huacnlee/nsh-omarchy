// The homepage shows the README's examples and component table, so the site
// and the README (whose Rust example is compiled and tested by
// tests/readme.rs) cannot drift apart.
import readme from '../../../README.md?raw';

/** The first fenced block of `lang` in the README. */
export function readmeCode(lang: string): string {
  const match = readme.match(new RegExp('```' + lang + '\\n([\\s\\S]*?)\\n```'));
  if (!match) throw new Error(`README.md has no \`\`\`${lang} block`);
  return match[1];
}

/** The rows of the README's component table: a family and its components. */
export function readmeFamilies(): { family: string; components: string[] }[] {
  const rows = readme.split('\n').filter((line) => /^\| [A-Z]/.test(line) && !line.startsWith('| Family') && !line.startsWith('| Path'));
  const families = rows
    .map((line) => line.split('|').map((cell) => cell.trim()).filter(Boolean))
    .filter((cells) => cells.length === 2 && !cells[0].startsWith('`'))
    .map(([family, components]) => ({
      family,
      components: components.split(/,\s*(?![^()]*\))/).map((name) => name.replace(/`/g, '')),
    }));
  if (families.length < 8) throw new Error('README.md component table not found');
  return families;
}
