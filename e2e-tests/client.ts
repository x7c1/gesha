export interface ApiClient {
  call: (path: string) => Promise<Response>;
}

export const client: ApiClient = {
  call: (path: string): Promise<Response> => {
    const url = toUrl(path);
    return fetch(url);
  },
};

function toUrl(path: string): string {
  return `http://localhost:8080/${path}`;
}
