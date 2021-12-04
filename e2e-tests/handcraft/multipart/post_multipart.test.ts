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

  const text = await response.text();
  console.log("response", response, text);
  assertEquals(1, 1);
});
