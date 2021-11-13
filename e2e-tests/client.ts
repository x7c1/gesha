export interface ApiClient {
  get: (path: string) => Promise<Response>;
  post: (path: string) => Promise<Response>;
}

export const client: ApiClient = {
  get: (path: string): Promise<Response> => {
    const url = toUrl(path);
    return fetch(url);
  },
  post: (path: string): Promise<Response> => {
    const url = toUrl(path);
    return fetch(url, {
      method: "POST",
    });
  },
};

function toUrl(path: string): string {
  return `http://localhost:8080/${path}`;
}
