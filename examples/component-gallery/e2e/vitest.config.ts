import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    include: ["tests/_lib/__tests__/**/*.test.ts"],
    environment: "node",
  },
});
