import { assertEquals } from "./deps.ts";
import { client } from "./client.ts";

Deno.test("index.html", async () => {
  const response = await client.call("123/foooo/index.html");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.text(),
  };
  const expected = {
    status: 200,
    contentType: "text/plain; charset=utf-8",
    body: "Hello foooo! id:123",
  };
  assertEquals(actual, expected);
});

Deno.test("show_pet_by_id - OK", async () => {
  const response = await client.call("pets/111");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };
  const expected = {
    status: 200,
    contentType: "application/json",
    body: {
      id: 111,
      name: "handcraft_name",
      tag: null,
    },
  };
  assertEquals(actual, expected);
});

Deno.test("show_pet_by_id - InternalServerError", async () => {
  const response = await client.call("pets/invalid_id");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };
  const expected = {
    status: 500,
    contentType: "application/json",
    body: {
      code: 1,
      message: "invalid digit found in string",
    },
  };
  assertEquals(actual, expected);
});
