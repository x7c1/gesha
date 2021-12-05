import { assertEquals, streamFromMultipart } from "../../deps.ts";

const endpoint = "multipart_form_data";

Deno.test("201", async () => {
  const [stream, boundary] = streamFromMultipart(async (writer) => {
    const file = await Deno.open("README.md");
    await writer.writeFile("binary_field", "README.md", file);
    file.close();

    await writer.writeField("string_field", "abcde");
  });

  const response = await fetch(`http://localhost:8080/${endpoint}`, {
    headers: {
      "Content-Type": `multipart/form-data; boundary=${boundary}`,
    },
    body: stream,
    method: "POST",
  });

  const actual = {
    status: response.status,
    contentType: response.headers.get("content-type"),
    body: await response.json(),
  };
  const expected = {
    status: 201,
    contentType: "application/json",
    body: {
      string_field: {
        name: "string_field",
        value: "abcde",
      },
      binary_field: {
        name: "binary_field",
        length: 8,
        file_name: "README.md",
      },
      optional_string_field: null,
    },
  };
  assertEquals(actual, expected);
});
