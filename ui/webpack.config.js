const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "..", "public");
module.exports = (env, argv) => {
  return {
    experiments: {
      asyncWebAssembly: true,
    },
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "bundle.js",
      webassemblyModuleFilename: "bundle.wasm"
    },
    module: {
      rules: [
        {
          test: /\.(scss)$/i,
          include: [
            path.resolve(__dirname, "stylesheets"),
          ],
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
            'postcss-loader'
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};