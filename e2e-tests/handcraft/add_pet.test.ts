import { client } from "../client.ts";
import { assertEquals } from "../deps.ts";

Deno.test("200", async () => {
  const response = await client.post("petstore-expanded/pet");
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };

  const expected = {
    status: 200,
    contentType: "application/json",
    body: { id: 123, name: "sample name", tag: null },
  };
  assertEquals(actual, expected);
});
