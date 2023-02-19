import { assertStringIncludes } from "https://deno.land/std@0.156.0/testing/asserts.ts";
import "../src/index.ts";

Deno.test({
  name: "Integration",
  fn: async () => {
    const response = await fetch("http://localhost:8000", { method: "GET", });
    assertStringIncludes(await response.text(), "This domain hosts a URL shortener");
  },
  sanitizeResources: false,
  sanitizeOps: false,
});
