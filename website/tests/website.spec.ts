import { test, expect, type Page } from '@playwright/test';

const galleryTitle = 'NSH Omarchy component gallery running in the browser';

async function pickSiteTheme(page: Page, name: string) {
  await page.getByRole('button', { name: /Theme:/ }).click();
  await page.getByRole('menuitemradio', { name }).click();
}

test('themes persist and every layout fits narrow screens', async ({ page }) => {
  await page.goto('./');
  await page.evaluate(() => localStorage.setItem('nsh-omarchy-theme', 'catppuccin'));
  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'tokyo-night');
  await expect(page.locator('astro-island[ssr]')).toHaveCount(0);
  for (const [name, id] of [['Flexoki Light', 'flexoki-light'], ['Tokyo Night', 'tokyo-night']]) {
    await pickSiteTheme(page, name);
    await expect(page.locator('html')).toHaveAttribute('data-theme', id);
    await page.reload();
    await expect(page.locator('html')).toHaveAttribute('data-theme', id);
  }
  for (const path of ['./', './api/']) {
    await page.goto(path);
    for (const width of [1440, 768, 390, 320]) {
      await page.setViewportSize({ width, height: 900 });
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), `${path} at ${width}`).toBe(true);
    }
  }
});

test('the homepage shows the README examples and copies the install command', async ({ page, context }) => {
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  await page.goto('./');
  await expect(page.getByRole('heading', { level: 1 })).toContainText('NSH / OMARCHY');
  // TSX is the default tab; Rust is one press or one arrow key away.
  const tsx = page.getByRole('tab', { name: 'TSX' });
  const rust = page.getByRole('tab', { name: 'Rust' });
  await expect(tsx).toHaveAttribute('aria-selected', 'true');
  await expect(page.getByRole('tabpanel')).toContainText('<Button label="Save" variant="primary"');
  await expect(page.getByText('om::button("Save").primary()')).toBeHidden();
  await rust.click();
  await expect(rust).toHaveAttribute('aria-selected', 'true');
  await expect(page.getByRole('tabpanel')).toContainText('om::button("Save").primary()');
  await rust.press('ArrowLeft');
  await expect(tsx).toBeFocused();
  await expect(tsx).toHaveAttribute('aria-selected', 'true');
  await expect(page.getByRole('heading', { name: 'Overlays' })).toBeVisible();
  await page.getByRole('button', { name: 'Copy to clipboard' }).click();
  await expect(page.getByRole('button', { name: 'Copy to clipboard' })).toHaveText('Copied');
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('nsh add nsh-omarchy');
  expect(errors).toEqual([]);
});

test('the homepage runs the TSX gallery', async ({ page }) => {
  test.setTimeout(120_000);
  await page.goto('./');
  await page.locator(`iframe[title="${galleryTitle}"]`).scrollIntoViewIfNeeded();
  const gallery = page.frameLocator(`iframe[title="${galleryTitle}"]`);
  await expect(gallery.getByRole('button', { name: 'Choices' })).toBeVisible({ timeout: 90_000 });
  await gallery.getByRole('button', { name: 'Choices' }).click();
  await expect(gallery.getByText('Choices', { exact: true }).first()).toBeVisible();
  await expect(gallery.getByRole('switch').first()).toBeVisible();
});

test('the gallery follows the site theme at startup and as it changes', async ({ page }) => {
  test.setTimeout(120_000);
  await page.addInitScript(() => localStorage.setItem('nsh-omarchy-theme', 'flexoki-light'));
  await page.goto('./');
  await page.locator(`iframe[title="${galleryTitle}"]`).scrollIntoViewIfNeeded();
  const gallery = page.frameLocator(`iframe[title="${galleryTitle}"]`);
  const light = gallery.getByRole('tab', { name: 'Light' });
  const dark = gallery.getByRole('tab', { name: 'Dark' });
  await expect(light).toHaveAttribute('aria-selected', 'true', { timeout: 90_000 });
  await pickSiteTheme(page, 'Tokyo Night');
  await expect(dark).toHaveAttribute('aria-selected', 'true');
  await expect(light).not.toHaveAttribute('aria-selected', 'true');
});

test('the API page renders docs/api.md with its chapters', async ({ page }) => {
  await page.goto('./api/');
  await expect(page.getByRole('heading', { level: 1 })).toContainText('nsh-omarchy API');
  const chapters = page.getByRole('navigation', { name: 'API chapters' }).getByRole('link');
  await expect(chapters).toHaveCount(5);
  await chapters.filter({ hasText: 'Components' }).click();
  await expect(page).toHaveURL(/#2-components$/);
  const markdown = await page.request.get('./api.md');
  expect(markdown.ok()).toBe(true);
  expect(await markdown.text()).toContain('# nsh-omarchy API');
});
