interface RequestParameter<A> {
  body?: A;
}

export interface ApiClient {
  get: (path: string) => Promise<Response>;
  post: <A>(path: string, params?: RequestParameter<A>) => Promise<Response>;
}

export const client: ApiClient = {
  get: (path: string): Promise<Response> => {
    const url = toUrl(path);
    return fetch(url);
  },
  post: <A>(path: string, params?: RequestParameter<A>): Promise<Response> => {
    const url = toUrl(path);

    const body = (params?.body === undefined)
      ? null
      : JSON.stringify(params.body);

    return fetch(url, {
      method: "POST",
      body,
    });
  },
};

function toUrl(path: string): string {
  return `http://localhost:8080/${path}`;
}
