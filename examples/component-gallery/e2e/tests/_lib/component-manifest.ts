export type ComponentStatus = "ready" | "coming-soon";

export type ComponentLayer = {
  smoke: boolean;
  motion: boolean;
  visual: boolean;
};

export type ManifestEntry = {
  /** Display name as it appears in the gallery's `h4`. Must match `docs.rs`. */
  name: string;
  /** Stable slug used for snapshot directory naming. */
  slug: string;
  /** Coverage layers enabled for this component. */
  layers: ComponentLayer;
  /** Component status. Coming-soon entries are smoke-only by definition. */
  status: ComponentStatus;
};

export const COMPONENT_MANIFEST: ManifestEntry[] = [
  // Foundations
  { name: "Surface", slug: "surface", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "GlassSurface", slug: "glass-surface", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "GlassLayer", slug: "glass-layer", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "LiquidSurface", slug: "liquid-surface", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  // Actions
  { name: "Button", slug: "button", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "IconButton", slug: "icon-button", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "CommandMenu", slug: "command-menu", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "Toolbar", slug: "toolbar", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  // Inputs
  { name: "TextField", slug: "text-field", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "Checkbox", slug: "checkbox", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "Switch", slug: "switch", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  // Layout
  { name: "Stack", slug: "stack", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "Tabs", slug: "tabs", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "Sidebar", slug: "sidebar", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  // Surfaces / Feedback / Misc
  { name: "MetricCard", slug: "metric-card", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "EmptyState", slug: "empty-state", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "Dialog", slug: "dialog", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "Toast", slug: "toast", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "Tooltip", slug: "tooltip", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "Alert", slug: "alert", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "Progress", slug: "progress", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  { name: "Skeleton", slug: "skeleton", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  // Motion
  { name: "Presence", slug: "presence", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "PresenceGate", slug: "presence-gate", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "KineticBox", slug: "kinetic-box", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "Sequence", slug: "sequence", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "TimelineScope", slug: "timeline-scope", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "KineticText", slug: "kinetic-text", status: "ready", layers: { smoke: true, motion: false, visual: true } },
  // Composition
  { name: "FrameStage", slug: "frame-stage", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "SharedElement", slug: "shared-element", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  { name: "SharedLayout", slug: "shared-layout", status: "ready", layers: { smoke: true, motion: true, visual: true } },
  // Capture
  { name: "CaptureStage", slug: "capture-stage", status: "ready", layers: { smoke: true, motion: false, visual: true } },
];

export function readyComponents(): ManifestEntry[] {
  return COMPONENT_MANIFEST.filter((c) => c.status === "ready");
}

export function findByName(name: string): ManifestEntry | undefined {
  return COMPONENT_MANIFEST.find((c) => c.name === name);
}
