import { expect, test } from '@playwright/test';

for (const [path, heading] of [
  ['/', 'Le mouvement au service de vos ambitions.'],
  ['/transport-logistique', 'Faire circuler vos marchandises'],
  ['/contact', 'Échangeons sur votre besoin'],
  ['/secteurs', 'Des réponses pensées pour vos réalités métier.'],
  ['/realisations', 'Des opérations documentées, bientôt réunies ici.'],
  ['/implantations', 'Des contacts locaux pour vos demandes.'],
  ['/actualites', 'Les nouvelles de SAPTRANS.'],
] as const) {
  test(`${path} rend le contenu principal`, async ({ page }) => {
    await page.goto(path);
    await expect(page.getByRole('heading', { level: 1, name: heading })).toBeVisible();
    await expect(page.locator('main')).toHaveCount(1);
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth > innerWidth);
    expect(overflow).toBe(false);
  });
}

test('le thème sombre et le thème clair sont persistants', async ({ page }) => {
  await page.goto('/');
  const theme = page.locator('html');
  const toggle = page.locator('.theme-toggle');
  await toggle.click();
  await expect(theme).toHaveAttribute('data-theme', 'dark');
  await page.reload();
  await expect(theme).toHaveAttribute('data-theme', 'dark');
  await toggle.click();
  await expect(theme).toHaveAttribute('data-theme', 'light');
});

test('la navigation anglaise affiche une vraie page en anglais', async ({ page }) => {
  await page.goto('/?lang=en');
  await expect(page.locator('html')).toHaveAttribute('lang', 'en');
  await expect(page.getByRole('heading', { level: 1, name: 'Moving your ambitions forward.' })).toBeVisible();
});

test('la recherche en anglais renvoie vers le service demandé', async ({ page }) => {
  await page.goto('/recherche?q=freight&lang=en');
  await expect(page.getByRole('heading', { level: 1, name: 'What are you looking for?' })).toBeVisible();
  await expect(page.locator('.search-results').getByRole('link', { name: /Freight & logistics/ })).toBeVisible();
});
