import { client } from "../client.ts";
import { assertEquals } from "../deps.ts";

Deno.test("list_pets - OK - empty", async () => {
  const response = await client.call("pets");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    x_next: response.headers.get("x-next"),
    body: await response.json(),
  };
  const expected = {
    status: 200,
    contentType: "application/json",
    x_next: null,
    body: [],
  };
  assertEquals(actual, expected);
});

Deno.test("list_pets - OK - query parameter", async () => {
  const response = await client.call("pets?limit=123");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    x_next: response.headers.get("x-next"),
    body: await response.json(),
  };
  const expected = {
    status: 200,
    contentType: "application/json",
    x_next: "456",
    body: [
      {
        id: 111,
        name: "name-111",
        tag: null,
      },
      {
        id: 222,
        name: "name-222",
        tag: null,
      },
    ],
  };
  assertEquals(actual, expected);
});

Deno.test("list_pets - InternalServerError", async () => {
  const response = await client.call("pets?limit=666");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    x_next: response.headers.get("x-next"),
    body: await response.json(),
  };
  const expected = {
    status: 500,
    contentType: "application/json",
    x_next: null,
    body: {
      code: 333,
      message: "sample error message",
    },
  };
  assertEquals(actual, expected);
});
