export default function (assetPath: string) {
  const runtimeConfig = useRuntimeConfig();
  const url = new URL(assetPath, runtimeConfig.public.defaultServer);
  return url.toString();
}
