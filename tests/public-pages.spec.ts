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

test('le menu Solutions ouvre un grand panneau et se ferme avec Échap', async ({ page, isMobile }) => {
  test.skip(isMobile, 'Le menu mobile utilise sa propre navigation.');
  await page.setViewportSize({ width: 1600, height: 900 });
  await page.goto('/transport-logistique', { waitUntil: 'domcontentloaded' });
  const menu = page.locator('.solutions-menu');
  const panel = menu.locator('.solutions-panel');
  await menu.locator('summary').click();
  await expect(panel).toBeVisible();
  await panel.getByRole('button', { name: /Location d’engins/ }).click();
  await expect(panel.getByRole('link', { name: /Découvrir les engins/ })).toBeVisible();
  await panel.getByRole('button', { name: /Contrôle technique/ }).click();
  await expect(panel.getByRole('link', { name: /Découvrir AutoRelax/ })).toBeVisible();
  const box = await panel.boundingBox();
  expect(box!.width).toBeGreaterThan(900);
  expect(box!.height).toBeGreaterThan(350);
  await page.keyboard.press('Escape');
  await expect(panel).toBeHidden();
  await page.setViewportSize({ width: 1280, height: 900 });
  await menu.locator('summary').click();
  await expect(panel).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth)).toBe(false);
  await page.setViewportSize({ width: 1025, height: 900 });
  await expect(menu.locator('summary')).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth)).toBe(false);
  await page.setViewportSize({ width: 1024, height: 900 });
  await expect(menu.locator('summary')).toBeHidden();
  await expect(page.locator('.mobile-menu summary')).toBeVisible();
});

test('les liens des solutions restent accessibles sur mobile', async ({ page, isMobile }) => {
  test.skip(!isMobile, 'Vérification du menu mobile.');
  await page.goto('/location-engins', { waitUntil: 'domcontentloaded' });
  const menu = page.locator('.mobile-menu');
  await menu.locator('summary').click();
  await expect(menu.getByRole('link', { name: 'Transport & logistique' })).toBeVisible();
  await expect(menu.getByRole('link', { name: 'Location d’engins' })).toBeVisible();
  await expect(menu.getByRole('link', { name: 'AutoRelax' })).toBeVisible();
});

test('le pied de page regroupe les liens SAPTRANS sur ordinateur et mobile', async ({ page, isMobile }) => {
  await page.goto('/', { waitUntil: 'domcontentloaded' });
  const footer = page.locator('footer');
  const logo = footer.locator('.footer-logo');
  await expect(logo).toBeVisible();
  await expect(logo).toHaveJSProperty('naturalWidth', 225);
  await expect(footer.locator('.footer-links > div')).toHaveCount(5);
  const social = footer.getByRole('navigation', { name: 'Réseaux sociaux' });
  for (const [name, href] of [
    ['Facebook', 'http://www.facebook.com'],
    ['X / Twitter', 'http://www.twitter.com'],
    ['Instagram', 'http://www.instagram.com'],
    ['LinkedIn', 'http://www.linkedin.com'],
  ]) {
    const link = social.getByRole('link', { name });
    await expect(link).toHaveAttribute('href', href);
    await expect(link).toHaveAttribute('rel', 'noopener noreferrer');
  }
  await expect(footer.getByRole('link', { name: 'Partenaires' })).toHaveAttribute('href', /#partenaires$/);
  await expect(page.locator('#partenaires')).toHaveCount(1);
  const sitemap = await page.request.get('/sitemap.xml');
  expect(sitemap.ok()).toBe(true);
  const columns = await footer.locator('.footer-links').evaluate((element) => getComputedStyle(element).gridTemplateColumns.split(' ').length);
  expect(columns).toBe(isMobile ? 2 : 5);
  expect(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth)).toBe(false);
});

test('les pages de service présentent une ouverture éditoriale adaptée à l’écran', async ({ page, isMobile }) => {
  await page.goto('/transport-logistique', { waitUntil: 'domcontentloaded' });
  const hero = page.locator('.activity-hero');
  const breadcrumb = hero.getByRole('navigation', { name: 'Fil d’Ariane' });
  await expect(breadcrumb.getByRole('link', { name: 'Solutions' })).toHaveAttribute('href', /#services$/);
  await expect(hero.getByRole('link', { name: /Faire une demande/ })).toBeVisible();
  const imageBox = await hero.locator('.activity-hero-media').boundingBox();
  const textBox = await hero.locator('.activity-hero-content').boundingBox();
  expect(imageBox).not.toBeNull();
  expect(textBox).not.toBeNull();
  if (isMobile) {
    expect(imageBox!.y).toBeGreaterThanOrEqual(textBox!.y + textBox!.height - 1);
  } else {
    expect(imageBox!.x).toBeGreaterThanOrEqual(page.viewportSize()!.width * 0.48);
    expect(imageBox!.height).toBeGreaterThan(600);
  }
  expect(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth)).toBe(false);
});

test('toutes les rubriques ouvrent le même grand panneau sur les pages principales', async ({ page, isMobile }) => {
  test.skip(isMobile, 'Le menu mobile utilise sa propre navigation.');
  await page.setViewportSize({ width: 1600, height: 900 });
  for (const path of ['/', '/transport-logistique', '/location-engins', '/entreprise']) {
    await page.goto(path, { waitUntil: 'domcontentloaded' });
    for (const label of ['Solutions', 'Services', 'Secteurs & lieux', 'À découvrir']) {
      const menu = page.locator('.desktop-nav details').filter({ has: page.locator('summary', { hasText: label }) });
      await menu.locator('summary').click();
      const panel = menu.locator('.mega-panel');
      await expect(panel).toBeVisible();
      const box = await panel.boundingBox();
      expect(box!.width).toBeGreaterThan(1500);
      expect(box!.height).toBeGreaterThan(350);
    }
  }
  await page.setViewportSize({ width: 1025, height: 900 });
  for (const menu of await page.locator('.desktop-nav details.mega-menu').all()) {
    await menu.locator('summary').click();
    await expect(menu.locator('.mega-panel')).toBeVisible();
    expect(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth)).toBe(false);
  }
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
  await page.goto('/entreprise', { waitUntil: 'domcontentloaded' });
  await page.evaluate(() => document.fonts.ready);
  const fonts = await page.evaluate(() => ({
    body: getComputedStyle(document.body).fontFamily,
    title: getComputedStyle(document.querySelector('h1')!).fontFamily,
    accent: getComputedStyle(document.querySelector('.company-divisions a strong')!).fontFamily,
    bodyLoaded: document.fonts.check('400 16px Manrope'),
    titleLoaded: document.fonts.check('800 64px Manrope'),
    accentLoaded: document.fonts.check('500 28px "Barlow Condensed"'),
  }));
  expect(fonts.body).toContain('Manrope');
  expect(fonts.title).toContain('Manrope');
  expect(fonts.accent).toContain('Barlow Condensed');
  expect(fonts.bodyLoaded).toBe(true);
  expect(fonts.titleLoaded).toBe(true);
  expect(fonts.accentLoaded).toBe(true);
});
