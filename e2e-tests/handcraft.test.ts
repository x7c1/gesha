import { assertEquals } from "./deps.ts";
import { client } from "./client.ts";

Deno.test("index.html", async () => {
  const response = await client.call("123/foooo/index.html");
  assertEquals(response.status, 200);
  assertEquals(
    response.headers.get("content-type"),
    "text/plain; charset=utf-8",
  );
  assertEquals(
    await response.text(),
    "Hello foooo! id:123",
  );
});

Deno.test("show_pet_by_id - OK", async () => {
  const response = await client.call("pets/111");
  assertEquals(response.status, 200);
  assertEquals(response.headers.get("content-type"), "application/json");
  assertEquals(await response.json(), {
    id: 111,
    name: "handcraft_name",
    tag: null,
  });
});

Deno.test("show_pet_by_id - InternalServerError", async () => {
  const response = await client.call("pets/invalid_id");
  assertEquals(response.status, 500);
  assertEquals(response.headers.get("content-type"), "application/json");
  assertEquals(await response.json(), {
    code: 1,
    message: "invalid digit found in string",
  });
});
