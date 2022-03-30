/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  webpack(config, { isServer }) {
    if (isServer) {
      config.output.webassemblyModuleFilename = './../static/wasm/[modulehash].wasm';
    } else {
      config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm';
    }
    config.experiments = { asyncWebAssembly: true };
    config.optimization.moduleIds = 'named';

    return config;
  },
};

module.exports = nextConfig;
