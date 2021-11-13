import { client } from "../client.ts";
import { assertEquals } from "../deps.ts";

Deno.test("201", async () => {
  const response = await client.get("petstore-expanded/pets");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.text(),
  };
  const expected = {
    status: 200,
    contentType: "application/json",
    body: "[]",
  };
  assertEquals(actual, expected);
});
