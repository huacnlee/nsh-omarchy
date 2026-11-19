import type { APIContext } from 'astro';
import { publicUrl } from '../lib/urls';

export function GET({ site }: APIContext) {
  const urls = ['', 'api/'].map((path) => publicUrl(site, path));
  return new Response(`<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls.map((url) => `  <url><loc>${url}</loc></url>`).join('\n')}\n</urlset>\n`, {
    headers: { 'Content-Type': 'application/xml; charset=utf-8' },
  });
}
