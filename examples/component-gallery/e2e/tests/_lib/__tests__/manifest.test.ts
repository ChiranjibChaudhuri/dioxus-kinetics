import { describe, expect, it } from "vitest";
import {
  COMPONENT_MANIFEST,
  findByName,
  readyComponents,
} from "../component-manifest.js";

describe("component-manifest", () => {
  it("contains at least one ready component", () => {
    expect(readyComponents().length).toBeGreaterThan(0);
  });

  it("slugs are unique", () => {
    const slugs = COMPONENT_MANIFEST.map((c) => c.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("names are unique", () => {
    const names = COMPONENT_MANIFEST.map((c) => c.name);
    expect(new Set(names).size).toBe(names.length);
  });

  it("findByName returns the entry for a known component", () => {
    const entry = findByName("Button");
    expect(entry).toBeDefined();
    expect(entry?.slug).toBe("button");
  });

  it("findByName returns undefined for unknown names", () => {
    expect(findByName("NotARealComponent")).toBeUndefined();
  });

  it("coming-soon components do not advertise motion or visual coverage", () => {
    for (const c of COMPONENT_MANIFEST) {
      if (c.status !== "coming-soon") continue;
      expect(c.layers.motion).toBe(false);
      expect(c.layers.visual).toBe(false);
    }
  });
});
