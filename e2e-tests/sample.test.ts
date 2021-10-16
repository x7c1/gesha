import { assertEquals } from "./deps.ts";
import { client } from "./client.ts";

Deno.test("index.html", async () => {
  const response = await client.call("123/foooo/index.html");
  const text = await response.text();
  assertEquals(text, "Hello foooo! id:123");
});

Deno.test("show_pet_by_id - OK", async () => {
  const response = await client.call("pets/111");
  const text = await response.text();
  assertEquals(text, `{"id":111,"name":"sample_name","tag":null}`);
});

Deno.test("show_pet_by_id - InternalServerError", async () => {
  const response = await client.call("pets/invalid_id");
  const text = await response.text();
  assertEquals(text, `{"code":1,"message":"invalid digit found in string"}`);
});
