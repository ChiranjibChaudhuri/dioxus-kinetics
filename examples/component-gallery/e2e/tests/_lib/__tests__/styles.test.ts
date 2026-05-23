import { describe, expect, it } from "vitest";
import { parseInlineStyles } from "../styles.js";

describe("parseInlineStyles", () => {
  it("returns empty object when no inline styles match the requested props", () => {
    expect(parseInlineStyles("", ["opacity"])).toEqual({});
    expect(parseInlineStyles("color: red", ["opacity"])).toEqual({});
  });

  it("parses opacity as a float", () => {
    expect(parseInlineStyles("opacity: 0.5", ["opacity"])).toEqual({
      opacity: 0.5,
    });
    expect(parseInlineStyles("opacity:1", ["opacity"])).toEqual({ opacity: 1 });
  });

  it("returns transform verbatim", () => {
    expect(
      parseInlineStyles("transform: translateY(12px)", ["transform"])
    ).toEqual({ transform: "translateY(12px)" });
  });

  it("parses the --ui-presence-t custom property as a float", () => {
    expect(
      parseInlineStyles("--ui-presence-t: 0.75; opacity: 0.5", [
        "presenceT",
        "opacity",
      ])
    ).toEqual({ presenceT: 0.75, opacity: 0.5 });
  });

  it("ignores malformed declarations without breaking valid ones", () => {
    expect(
      parseInlineStyles("opacity:0.3; brokenline; transform: scale(1)", [
        "opacity",
        "transform",
      ])
    ).toEqual({ opacity: 0.3, transform: "scale(1)" });
  });
});
