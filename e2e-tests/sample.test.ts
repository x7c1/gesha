import { assertEquals } from "./deps.ts";
import { client } from "./client.ts";

Deno.test("index.html", async () => {
  const response = await client.call("123/foooo/index.html");
  const text = await response.text();
  assertEquals(text, "Hello foooo! id:123");
});

Deno.test("show_pet_by_id", async () => {
  const response = await client.call("pets/1234");
  const text = await response.text();
  assertEquals(text, `{"id":0,"name":"sample_name","tag":null}`);
});
