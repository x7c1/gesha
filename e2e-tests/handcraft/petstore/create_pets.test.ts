import { client } from "../../client.ts";
import { assertEquals } from "../../deps.ts";

const endpoint = "pets";

Deno.test("201", async () => {
  const response = await client.post(endpoint);
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.text(),
  };
  const expected = {
    status: 201,
    contentType: null,
    body: "",
  };
  assertEquals(actual, expected);
});
