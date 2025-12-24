import type { UseFetchOptions } from "nuxt/app";

export default function <T>(
  url: Parameters<typeof useFetch>[0],
  options?: UseFetchOptions<T>
) {
  return useFetch(url, {
    ...options,
    $fetch: $fetch.create({
      baseURL: useRuntimeConfig().public.defaultServer,
      headers: {
        Accept: "application/json",
      },
    }),
  });
}
