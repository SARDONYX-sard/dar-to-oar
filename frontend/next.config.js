/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  experimental: {
    appDir: true,
  },
  distDir: "../out",
  // output: "export",
  swcMinify: true,
};

module.exports = nextConfig;
