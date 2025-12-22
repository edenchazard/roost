import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: false },
  ssr: false,

  modules: ["@nuxt/eslint", "@nuxt/icon", "@nuxt/test-utils", "@vueuse/nuxt"],

  runtimeConfig: {
    public: {
      defaultServer:
        process.env.CLIENT_DEFAULT_SERVER ?? "http://localhost:3001",
    },
  },

  vite: {
    plugins: [tailwindcss()],
  },

  css: ["~/assets/css/tailwind.css"],
});
