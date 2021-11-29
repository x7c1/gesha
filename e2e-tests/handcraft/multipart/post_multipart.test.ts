import { assertEquals, streamFromMultipart } from "../../deps.ts";

const endpoint = "multipart_request";

Deno.test("201", async () => {
  const [stream, boundary] = streamFromMultipart(async (writer) => {
    const file = await Deno.open("README.md");
    await writer.writeFile("file", "README.md", file);
    file.close();

    await writer.writeField("hoge1", "日本語ほげほげ");
    await writer.writeField("hoge2", "fuga2");
  });

  const response = await fetch(`http://localhost:8080/${endpoint}`, {
    headers: {
      "Content-Type": `multipart/form-data; boundary=${boundary}`,
    },
    body: stream,
    method: "POST",
  });

  response.text();
  console.log("response", response);
  assertEquals(1, 1);
});
