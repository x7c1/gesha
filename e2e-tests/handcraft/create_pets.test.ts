import { client } from "../client.ts";
import { assertEquals } from "../deps.ts";

Deno.test("201", async () => {
  const response = await client.post("pets");
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
