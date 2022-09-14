/* eslint-disable import/order */
/* eslint-disable @typescript-eslint/no-var-requires */

const withBundleAnalyzer = require("@next/bundle-analyzer")({
  enabled: process.env.ANALYZE === "true",
});

// const runtimeCaching = require("next-pwa/cache");
// const withPWA = require("next-pwa")({
//   pwa: {
//     dest: "public",
//     disable: process.env.NODE_ENV === "development",
//     runtimeCaching,
//   },
// });

/** @type {import('next').NextConfig} */
const config = {
  swcMinify: true,
  i18n: { locales: ["ja"], defaultLocale: "ja" },
  reactStrictMode: true,
  poweredByHeader: false,
  images: {
    formats: ["image/avif", "image/webp"],
  },
};

const withPlugins = (plugins, config) => {
  return plugins.reduce((acc, plugin) => plugin(acc), { ...config });
};

module.exports = withPlugins([withBundleAnalyzer], config);
