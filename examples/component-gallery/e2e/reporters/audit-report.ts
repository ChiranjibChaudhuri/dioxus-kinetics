import type {
  FullConfig,
  FullResult,
  Reporter,
  TestCase,
  TestError,
  TestResult,
} from "@playwright/test/reporter";
import { writeFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import {
  COMPONENT_MANIFEST,
  type ManifestEntry,
} from "../tests/_lib/component-manifest.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

type Layer = "smoke" | "motion" | "visual";
type Variant = "default" | "dark" | "reduced-motion" | "solid-glass";

type RowKey = `${string}::${Layer}::${Variant}`;

type Outcome = "pass" | "fail" | "flaky" | "skipped";

type Cell = {
  outcome: Outcome;
  notes: string[];
};

type Run = {
  startedAt: string;
  rows: Map<RowKey, Cell>;
};

function rowKey(name: string, layer: Layer, variant: Variant): RowKey {
  return `${name}::${layer}::${variant}` as RowKey;
}

function classifyLayer(testFile: string): Layer {
  if (testFile.includes("visual.spec.")) return "visual";
  if (testFile.includes("smoke.spec.")) return "smoke";
  return "motion";
}

function classifyVariant(title: string): Variant {
  const m = title.match(/@(default|dark|reduced-motion|solid-glass)/);
  if (m) return m[1] as Variant;
  return "default";
}

function classifyComponent(test: TestCase): string | undefined {
  // For smoke + visual, the per-test title starts with the component name.
  const direct = test.title.match(/^([A-Z][A-Za-z]+)\b/);
  if (direct) return direct[1];
  // For bespoke specs, the describe block carries the component name.
  for (const parent of test.parent.titlePath()) {
    const match = parent.match(/^([A-Z][A-Za-z]+)$/);
    if (match) return match[1];
  }
  return undefined;
}

function outcomeOf(result: TestResult): Outcome {
  if (result.status === "skipped") return "skipped";
  if (result.status === "passed") {
    // A test that passed only after retries is considered flaky.
    return result.retry > 0 ? "flaky" : "pass";
  }
  return "fail";
}

export function renderTable(
  manifest: ManifestEntry[],
  rows: Map<RowKey, Cell>
): string {
  const lines: string[] = [];
  lines.push("| Component | Smoke | Motion | Visual | Status | Notes |");
  lines.push("|---|---|---|---|---|---|");
  for (const entry of manifest) {
    if (entry.status !== "ready") continue;
    const cells: Record<Layer, string> = { smoke: "n/a", motion: "n/a", visual: "n/a" };
    const notes: string[] = [];

    for (const layer of ["smoke", "motion", "visual"] as Layer[]) {
      if (!entry.layers[layer]) continue;
      let worst: Outcome | undefined;
      for (const variant of ["default", "dark", "reduced-motion", "solid-glass"] as Variant[]) {
        const cell = rows.get(rowKey(entry.name, layer, variant));
        if (!cell) continue;
        if (cell.notes.length > 0) notes.push(`${layer}@${variant}: ${cell.notes.join("; ")}`);
        worst = worseOutcome(worst, cell.outcome);
      }
      cells[layer] = outcomeLabel(worst);
    }

    const status = computeStatus(entry, cells);
    lines.push(
      `| ${entry.name} | ${cells.smoke} | ${cells.motion} | ${cells.visual} | ${status} | ${notes.join(" / ")} |`
    );
  }
  return lines.join("\n");
}

function outcomeLabel(o: Outcome | undefined): string {
  if (!o) return "n/a";
  if (o === "pass") return "pass";
  if (o === "fail") return "fail";
  if (o === "flaky") return "flaky";
  return "skipped";
}

function worseOutcome(a: Outcome | undefined, b: Outcome): Outcome {
  if (!a) return b;
  const order: Outcome[] = ["pass", "skipped", "flaky", "fail"];
  return order.indexOf(b) > order.indexOf(a) ? b : a;
}

function computeStatus(entry: ManifestEntry, cells: Record<Layer, string>): string {
  const layers = (["smoke", "motion", "visual"] as Layer[]).filter((l) => entry.layers[l]);
  const allPass = layers.every((l) => cells[l] === "pass");
  if (allPass) return "ready";
  const anyFail = layers.some((l) => cells[l] === "fail");
  if (anyFail) return "regression";
  return "partial";
}

class AuditReportReporter implements Reporter {
  private run: Run = { startedAt: new Date().toISOString(), rows: new Map() };

  onTestEnd(test: TestCase, result: TestResult): void {
    const component = classifyComponent(test);
    if (!component) return;
    const layer = classifyLayer(test.location?.file ?? "");
    const variant = classifyVariant(test.titlePath().join(" "));
    const key = rowKey(component, layer, variant);
    const outcome = outcomeOf(result);
    const notes: string[] = [];
    if (outcome === "fail" && result.errors[0]?.message) {
      notes.push(result.errors[0].message.split("\n")[0].slice(0, 120));
    }
    this.run.rows.set(key, { outcome, notes });
  }

  async onEnd(_result: FullResult): Promise<void> {
    const body = [
      `# Component Gallery Audit Report`,
      ``,
      `- Started: ${this.run.startedAt}`,
      `- Finished: ${new Date().toISOString()}`,
      ``,
      `## Summary`,
      ``,
      renderTable(COMPONENT_MANIFEST, this.run.rows),
      ``,
      `## Coming Soon`,
      ``,
      ...COMPONENT_MANIFEST.filter((c) => c.status === "coming-soon").map(
        (c) => `- ${c.name}`
      ),
      ``,
    ].join("\n");

    const path = resolve(__dirname, "..", "audit-report.md");
    try {
      writeFileSync(path, body, "utf8");
    } catch (err) {
      // eslint-disable-next-line no-console
      console.warn(`[audit-report] failed to write ${path}: ${(err as Error).message}`);
    }
  }

  // Required no-op handlers from the Reporter interface.
  onBegin(_config: FullConfig): void {}
  onError(_error: TestError): void {}
}

export default AuditReportReporter;
