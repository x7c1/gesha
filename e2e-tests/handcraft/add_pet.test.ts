import { client } from "../client.ts";
import { assertEquals } from "../deps.ts";

Deno.test("200", async () => {
  const response = await client.post("petstore-expanded/pet", {
    body: {
      name: "sample-pet",
      tag: "sample-tag",
    },
  });
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };

  const expected = {
    status: 200,
    contentType: "application/json",
    body: { id: 123, name: "created:sample-pet", tag: "created:sample-tag" },
  };
  assertEquals(actual, expected);
});

Deno.test("400 missing-field", async () => {
  const response = await client.post("petstore-expanded/pet", {
    body: {
      name_asdf: "sample-pet",
      tag_asdf: "sample-tag",
    },
  });
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };

  const expected = {
    status: 400,
    contentType: "application/json",
    body: { code: 4005, message: "missing field `name` at line 1 column 50" },
  };
  assertEquals(actual, expected);
});

Deno.test("400 invalid-type", async () => {
  const response = await client.post("petstore-expanded/pet", {
    body: {
      name: 12345,
      tag: "sample-tag",
    },
  });
  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };

  const expected = {
    status: 400,
    contentType: "application/json",
    body: {
      code: 4005,
      message:
        "invalid type: integer `12345`, expected a string at line 1 column 13",
    },
  };
  assertEquals(actual, expected);
});
