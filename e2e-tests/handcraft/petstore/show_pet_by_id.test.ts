import { client } from "../../client.ts";
import { assertEquals } from "../../deps.ts";

const endpoint = "pets";

Deno.test("200", async () => {
  const response = await client.get(`${endpoint}/111`);
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

Deno.test("500", async () => {
  const response = await client.get(`${endpoint}/invalid_id`);
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
