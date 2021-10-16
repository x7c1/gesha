import { assertEquals } from "./deps.ts";
import { client } from "./client.ts";

Deno.test("index.html", async () => {
  const response = await client.call("123/foooo/index.html");
  const text = await response.text();
  assertEquals(text, "Hello foooo! id:123");
});
