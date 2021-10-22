import { client } from "../client.ts";
import { assertEquals } from "../deps.ts";

Deno.test("200 empty", async () => {
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

Deno.test("200 non-empty", async () => {
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

Deno.test("400 invalid query parameter", async () => {
  const response = await client.call("pets?limit=invalid");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    x_next: response.headers.get("x-next"),
    body: await response.text(),
  };
  const expected = {
    status: 400,
    contentType: "text/plain; charset=utf-8",
    x_next: null,
    body: "Query deserialize error: invalid digit found in string",
  };
  assertEquals(actual, expected);
});

Deno.test("500", async () => {
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
