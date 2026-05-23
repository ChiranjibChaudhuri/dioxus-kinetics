import { execSync } from "node:child_process";
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = resolve(__filename, "..");

const PROJECT_ROOT = resolve(__dirname, "..");
const DIST_DIR = resolve(PROJECT_ROOT, "dist");
const INDEX_HTML = resolve(DIST_DIR, "index.html");

export default async function globalSetup() {
  // Only the `static` project needs a build. `dev-loop` runs against a
  // user-managed `dx serve`. Distinguish via env var set by the project
  // config below.
  if (process.env.KINETICS_E2E_MODE !== "static") {
    return;
  }

  // eslint-disable-next-line no-console
  console.log("[e2e] running `dx build --release --package component-gallery`...");
  try {
    execSync("dx build --release --package component-gallery", {
      cwd: resolve(PROJECT_ROOT, "..", ".."),
      stdio: "inherit",
    });
  } catch (err) {
    throw new Error(
      "`dx build` failed. Ensure the Dioxus CLI is installed and on PATH " +
        "(see README.md). Original error: " +
        (err as Error).message
    );
  }

  if (!existsSync(INDEX_HTML)) {
    throw new Error(
      `[e2e] dx build succeeded but ${INDEX_HTML} is missing. ` +
        "Check Dioxus.toml `out_dir` setting."
    );
  }

  // eslint-disable-next-line no-console
  console.log(`[e2e] static artifact ready at ${DIST_DIR}`);
}
