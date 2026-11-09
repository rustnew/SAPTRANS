import { expect, test } from '@playwright/test';

for (const [path, heading] of [
  ['/', 'Des solutions pour vous faire avancer.'],
  ['/transport-logistique', 'Faire circuler vos marchandises'],
  ['/contact', 'Échangeons sur votre besoin'],
] as const) {
  test(`${path} rend le contenu principal`, async ({ page }) => {
    await page.goto(path);
    await expect(page.getByRole('heading', { level: 1, name: heading })).toBeVisible();
    await expect(page.locator('main')).toHaveCount(1);
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth > innerWidth);
    expect(overflow).toBe(false);
  });
}
