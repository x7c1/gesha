import { assertEquals, streamFromMultipart } from "../../deps.ts";

const endpoint = "multipart_form_data";

Deno.test("201", async () => {
  const [stream, boundary] = streamFromMultipart(async (writer) => {
    const file = await Deno.open("LICENSE");
    await writer.writeFile("binary_field", "LICENSE", file);
    file.close();

    await writer.writeField("string_field", "abcde");
    await writer.writeField(
      "optional_object_field",
      JSON.stringify({
        field_a: "sample field_a",
        field_b: ["b1", "b2"],
      }),
    );
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
        length: 1063,
        file_name: "LICENSE",
      },
      optional_string_field: null,
      optional_object_field: {
        name: "optional_object_field",
        value: {
          field_a: "sample field_a",
          field_b: ["b1", "b2"],
        },
      },
    },
  };
  assertEquals(actual, expected);
});
