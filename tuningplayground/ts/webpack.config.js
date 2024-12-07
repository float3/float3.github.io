const CopyPlugin = require("copy-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const JsonMinimizerPlugin = require("json-minimizer-webpack-plugin");
const HTMLMinimizerPlugin = require("html-minimizer-webpack-plugin");
const path = require("path");

module.exports = {
  module: {
    rules: [
      {
        test: /\.json$/i,
        type: "asset/resource",
      },
      {
        test: /\.txt$/i,
        type: "asset/resource",
      },
      {
        test: /\.html$/i,
        type: "asset/resource",
      },
      {
        test: /\.wav$/,
        type: "asset/resource",
      },
      {
        test: /\.midi?$/,
        type: "asset/resource",
      },
    ],
  },
  entry: "./dist/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "../www/"),
    filename: "bootstrap.js",
  },
  target: "web",
  plugins: [
    new CopyPlugin({
      patterns: ["./src/chords.json", "./src/chords.txt"],
      options: {
        concurrency: 100,
      },
    }),
  ],
  optimization: {
    minimizer: [
      new TerserPlugin({
        terserOptions: {
          compress: {
            drop_console: true,
            pure_funcs: [
              "console.log",
              "console.info",
              "console.debug",
              "console.error",
              "console.warn",
              "console.assert",
            ],
          },
          mangle: true,
        },
      }),
      new JsonMinimizerPlugin(),
      new HTMLMinimizerPlugin(),
    ],
  },
  experiments: {
    syncWebAssembly: true,
  },
};
