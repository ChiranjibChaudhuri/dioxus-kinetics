import { defineConfig, devices } from "@playwright/test";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = resolve(__filename, "..");

const projectArg = process.argv.find((arg) => arg.startsWith("--project="));
if (projectArg) {
  const name = projectArg.slice("--project=".length);
  process.env.KINETICS_E2E_MODE = name === "dev-loop" ? "dev-loop" : "static";
} else if (!process.env.KINETICS_E2E_MODE) {
  process.env.KINETICS_E2E_MODE = "static";
}

const PROJECT_ROOT = resolve(__dirname, "..");
const WORKSPACE_ROOT = resolve(PROJECT_ROOT, "..", "..");
const DIST_DIR = resolve(
  WORKSPACE_ROOT,
  "target",
  "dx",
  "flagship",
  "release",
  "web",
  "public"
);
const STATIC_PORT = 4174;
const DEV_LOOP_URL = process.env.KINETICS_FLAGSHIP_URL ?? "http://localhost:9174";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["list"], ["html", { open: "never", outputFolder: "playwright-report" }]],
  expect: {
    toHaveScreenshot: { maxDiffPixelRatio: 0.05, animations: "disabled" },
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
    },
    {
      name: "dev-loop",
      use: {
        ...devices["Desktop Chrome"],
        baseURL: DEV_LOOP_URL,
      },
    },
  ],
  webServer:
    process.env.KINETICS_E2E_MODE === "dev-loop"
      ? undefined
      : {
          command: `npx http-server "${DIST_DIR}" -p ${STATIC_PORT} --silent`,
          port: STATIC_PORT,
          reuseExistingServer: !process.env.CI,
          timeout: 60_000,
        },
});
