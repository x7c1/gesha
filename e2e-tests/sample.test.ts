import { assertEquals } from "./deps.ts";

Deno.test("index.html", async () => {
  const res = await fetch("http://localhost:8080/123/foooo/index.html");
  const text = await res.text();
  assertEquals(text, "Hello foooo! id:123");
});
