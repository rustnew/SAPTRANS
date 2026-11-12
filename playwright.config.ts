import { defineConfig, devices } from '@playwright/test';

const previewPort = process.env.PREVIEW_PORT || '3000';
const localChrome = process.env.PLAYWRIGHT_CHROME_EXECUTABLE;

export default defineConfig({
  testDir: './tests',
  use: {
    baseURL: `http://127.0.0.1:${previewPort}`,
    ...(localChrome ? { launchOptions: { executablePath: localChrome } } : {}),
  },
  projects: [
    { name: 'desktop', use: { ...devices['Desktop Chrome'] } },
    { name: 'mobile', use: { ...devices['iPhone 13'], browserName: 'chromium' } },
  ],
  webServer: {
    command: 'cargo run --manifest-path preview/Cargo.toml --bin saptrans-preview',
    env: { PREVIEW_LISTEN_ADDR: `127.0.0.1:${previewPort}` },
    url: `http://127.0.0.1:${previewPort}`,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
