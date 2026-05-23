import type { Locator } from "@playwright/test";

export type StyleSnapshot = Partial<{
  opacity: number;
  transform: string;
  presenceT: number;
}>;

export function parseInlineStyles(
  raw: string,
  props: Array<keyof StyleSnapshot>
): StyleSnapshot {
  const decls = new Map<string, string>();
  for (const part of raw.split(";")) {
    const idx = part.indexOf(":");
    if (idx < 0) continue;
    const key = part.slice(0, idx).trim();
    const value = part.slice(idx + 1).trim();
    if (key) decls.set(key, value);
  }

  const out: StyleSnapshot = {};
  for (const prop of props) {
    switch (prop) {
      case "opacity": {
        const v = decls.get("opacity");
        if (v !== undefined) {
          const parsed = Number.parseFloat(v);
          if (!Number.isNaN(parsed)) out.opacity = parsed;
        }
        break;
      }
      case "transform": {
        const v = decls.get("transform");
        if (v !== undefined) out.transform = v;
        break;
      }
      case "presenceT": {
        const v = decls.get("--ui-presence-t");
        if (v !== undefined) {
          const parsed = Number.parseFloat(v);
          if (!Number.isNaN(parsed)) out.presenceT = parsed;
        }
        break;
      }
    }
  }
  return out;
}

export async function readStyles(
  locator: Locator,
  props: Array<keyof StyleSnapshot>
): Promise<StyleSnapshot> {
  const raw = (await locator.getAttribute("style")) ?? "";
  return parseInlineStyles(raw, props);
}
