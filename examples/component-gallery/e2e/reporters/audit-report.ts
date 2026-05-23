// Stub — full implementation in Task 22.
import type { Reporter } from "@playwright/test/reporter";

const AuditReporter: new () => Reporter = class implements Reporter {
  onBegin() {}
  onEnd() {}
};

export default AuditReporter;
