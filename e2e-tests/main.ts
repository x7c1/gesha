import { assertEquals } from "https://deno.land/std@0.92.0/testing/asserts.ts";

Deno.test("sample index.html", async () => {
    const res = await fetch("http://localhost:8080/123/fuga/index.html");
    const text = await res.text();
    assertEquals(text, "Hello fuga! id:123")
})
