import { expect, test } from '@playwright/test';

for (const [path, heading] of [
  ['/', 'Le mouvement au service de vos ambitions.'],
  ['/transport-logistique', 'Le transport pensé pour le terrain.'],
  ['/contact', 'Parlons de votre projet.'],
  ['/secteurs', 'Des réponses pensées pour vos réalités métier.'],
  ['/realisations', 'Des opérations documentées, bientôt réunies ici.'],
  ['/implantations', 'Des contacts locaux pour vos demandes.'],
  ['/actualites', 'Les nouvelles de SAPTRANS.'],
] as const) {
  test(`${path} rend le contenu principal`, async ({ page }) => {
    await page.goto(path, { waitUntil: 'domcontentloaded' });
    await expect(page.getByRole('heading', { level: 1, name: heading })).toBeVisible();
    await expect(page.locator('main')).toHaveCount(1);
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth);
    expect(overflow).toBe(false);
  });
}

test('le thème sombre et le thème clair sont persistants', async ({ page }) => {
  await page.goto('/', { waitUntil: 'domcontentloaded' });
  const theme = page.locator('html');
  const toggle = page.locator('.theme-toggle');
  const darkLogo = page.locator('.header .logo-for-light');
  const whiteLogo = page.locator('.header .logo-for-dark');
  await expect(darkLogo).toBeVisible();
  await expect(darkLogo).toHaveJSProperty('naturalWidth', 225);
  await toggle.click();
  await expect(theme).toHaveAttribute('data-theme', 'dark');
  await expect(whiteLogo).toBeVisible();
  await expect(whiteLogo).toHaveJSProperty('naturalWidth', 225);
  await page.reload({ waitUntil: 'domcontentloaded' });
  await expect(theme).toHaveAttribute('data-theme', 'dark');
  await toggle.click();
  await expect(theme).toHaveAttribute('data-theme', 'light');
  await expect(darkLogo).toBeVisible();
});

test('la navigation anglaise affiche une vraie page en anglais', async ({ page }) => {
  await page.goto('/?lang=en', { waitUntil: 'domcontentloaded' });
  await expect(page.locator('html')).toHaveAttribute('lang', 'en');
  await expect(page.getByRole('heading', { level: 1, name: 'Moving your ambitions forward.' })).toBeVisible();
});

test('la recherche en anglais renvoie vers le service demandé', async ({ page }) => {
  await page.goto('/recherche?q=freight&lang=en', { waitUntil: 'domcontentloaded' });
  await expect(page.getByRole('heading', { level: 1, name: 'What are you looking for?' })).toBeVisible();
  await expect(page.locator('.search-results .card').first().getByRole('heading', { name: 'Freight & logistics' })).toBeVisible();
});

test('la page métier présente et relie toutes ses sections', async ({ page }) => {
  await page.goto('/transport-logistique', { waitUntil: 'domcontentloaded' });
  const navigation = page.locator('.page-anchor-nav');
  for (const id of ['apercu', 'prestations', 'situations', 'demarche', 'informations', 'questions', 'contact-service']) {
    await expect(page.locator(`#${id}`)).toHaveCount(1);
    await expect(navigation.locator(`a[href="#${id}"]`)).toHaveCount(1);
  }
  await navigation.locator('a[href="#situations"]').click();
  await expect(page.locator('#situations')).toBeInViewport();
  await expect(navigation.locator('a[href="#situations"]')).toHaveAttribute('aria-current', 'location');
});

test('les visuels africains des pages principales se chargent', async ({ page }) => {
  for (const path of ['/', '/transport-logistique', '/location-engins', '/controle-technique', '/immobilier', '/entreprise']) {
    await page.goto(path, { waitUntil: 'domcontentloaded' });
    const sources = await page.locator('main img[src^="/media/editorial/"]').evaluateAll((images) =>
      [...new Set(images.map((image) => (image as HTMLImageElement).getAttribute('src')))]
    );
    expect(sources.length, `${path} doit afficher des visuels éditoriaux`).toBeGreaterThan(0);
    for (const source of sources) {
      const response = await page.request.get(source!);
      expect(response.ok(), `${path} : ${source}`).toBe(true);
      expect(response.headers()['content-type'], `${path} : ${source}`).toMatch(/^image\/(jpeg|png)/);
    }
  }
});

test('les deux polices éditoriales locales sont chargées', async ({ page }) => {
  await page.goto('/', { waitUntil: 'domcontentloaded' });
  await page.evaluate(() => document.fonts.ready);
  const fonts = await page.evaluate(() => ({
    body: getComputedStyle(document.body).fontFamily,
    title: getComputedStyle(document.querySelector('h1')!).fontFamily,
    bodyLoaded: document.fonts.check('400 16px Manrope'),
    titleLoaded: document.fonts.check('500 72px "Barlow Condensed"'),
  }));
  expect(fonts.body).toContain('Manrope');
  expect(fonts.title).toContain('Barlow Condensed');
  expect(fonts.bodyLoaded).toBe(true);
  expect(fonts.titleLoaded).toBe(true);
});
