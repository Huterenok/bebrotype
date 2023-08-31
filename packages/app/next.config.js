/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    domains: ["lh3.googleusercontent.com"],
  },
  presets: ["next/babel"],
  plugins: [
    [
      "effector/babel-plugin",
      {
        factories: ["effector-forms"],
      },
    ],
  ],
};

module.exports = nextConfig;
