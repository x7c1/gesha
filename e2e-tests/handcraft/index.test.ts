import { assertEquals } from "../deps.ts";
import { client } from "../client.ts";

Deno.test("index.html", async () => {
  const response = await client.get("123/foooo/index.html");
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
