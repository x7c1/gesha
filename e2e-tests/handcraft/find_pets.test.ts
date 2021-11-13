import { client } from "../client.ts";
import { assertEquals } from "../deps.ts";

const endpoint = "petstore-expanded/pets";

Deno.test("200-empty", async () => {
  const response = await client.get(endpoint);
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };
  const expected = {
    status: 200,
    contentType: "application/json",
    body: [],
  };
  assertEquals(actual, expected);
});

Deno.test("200-multiple", async () => {
  const response = await client.get(`${endpoint}?tags=t0&tags=t1`);
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };
  const expected = {
    status: 200,
    contentType: "application/json",
    body: [
      {
        id: 0,
        name: "name-0",
        tag: "t0",
      },
      {
        id: 1,
        name: "name-1",
        tag: "t1",
      },
    ],
  };
  assertEquals(actual, expected);
});
