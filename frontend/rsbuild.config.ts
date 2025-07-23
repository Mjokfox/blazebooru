import { defineConfig } from "@rsbuild/core";
import { pluginSass } from "@rsbuild/plugin-sass";
import { pluginVue } from "@rsbuild/plugin-vue";

export default defineConfig({
  plugins: [pluginSass(), pluginVue()],
  html: {
    template: "./index.html",
    favicon: "./favicon.ico",
  },
  server: {
    port: 5173,
    proxy: {
      "/api/": {
        target: "http://localhost:3000",
        xfwd: true,
      },
      "/f/": {
        target: "http://localhost:3000",
      },
    },
  },
});
