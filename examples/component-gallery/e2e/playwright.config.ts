import { defineConfig, devices } from "@playwright/test";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = resolve(__filename, "..");

// Resolve KINETICS_E2E_MODE from --project=... so defineConfig (below) and
// global-setup.ts see a consistent value. Must run BEFORE defineConfig.
const projectArg = process.argv.find((arg) => arg.startsWith("--project="));
if (projectArg) {
  const name = projectArg.slice("--project=".length);
  process.env.KINETICS_E2E_MODE = name === "dev-loop" ? "dev-loop" : "static";
} else if (!process.env.KINETICS_E2E_MODE) {
  process.env.KINETICS_E2E_MODE = "static";
}

const PROJECT_ROOT = resolve(__dirname, "..");
const DIST_DIR = resolve(PROJECT_ROOT, "dist");
const STATIC_PORT = 4173;
const DEV_LOOP_URL = process.env.KINETICS_DEV_LOOP_URL ?? "http://localhost:9173";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [
    ["list"],
    ["html", { open: "never", outputFolder: "playwright-report" }],
    ["./reporters/audit-report.ts"],
  ],
  globalSetup: "./global-setup.ts",
  expect: {
    toHaveScreenshot: {
      maxDiffPixelRatio: 0.05,
      animations: "disabled",
    },
  },
  use: {
    actionTimeout: 10_000,
    navigationTimeout: 30_000,
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  projects: [
    {
      name: "static",
      use: {
        ...devices["Desktop Chrome"],
        baseURL: `http://localhost:${STATIC_PORT}`,
      },
      metadata: { mode: "static" },
    },
    {
      name: "static-webkit",
      use: {
        ...devices["Desktop Safari"],
        baseURL: `http://localhost:${STATIC_PORT}`,
      },
      metadata: { mode: "static" },
    },
    {
      name: "dev-loop",
      use: {
        ...devices["Desktop Chrome"],
        baseURL: DEV_LOOP_URL,
      },
      metadata: { mode: "dev-loop" },
    },
  ],
  webServer: process.env.KINETICS_E2E_MODE === "dev-loop"
    ? undefined
    : {
        command: `npx http-server ${DIST_DIR} -p ${STATIC_PORT} --silent`,
        port: STATIC_PORT,
        reuseExistingServer: !process.env.CI,
        timeout: 60_000,
      },
});

