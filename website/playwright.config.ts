import { defineConfig } from '@playwright/test';

// The site under test: the dev server, or with PLAYWRIGHT_PRODUCTION=1 the
// production build at the GitHub Pages base path.
const production = process.env.PLAYWRIGHT_PRODUCTION === '1';
const localURL = production ? 'http://127.0.0.1:4323/nsh-omarchy/' : 'http://127.0.0.1:4321/';
export default defineConfig({
  testDir: './tests',
  use: { baseURL: process.env.PLAYWRIGHT_BASE_URL || localURL, screenshot: 'only-on-failure', trace: 'retain-on-failure' },
  webServer: process.env.PLAYWRIGHT_BASE_URL ? undefined : {
    command: production ? 'BASE_PATH=/nsh-omarchy bun run preview --port 4323' : 'bun run dev',
    url: localURL,
    reuseExistingServer: !process.env.CI,
  },
});
