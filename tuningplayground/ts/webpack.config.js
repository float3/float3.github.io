import CopyPlugin from "copy-webpack-plugin"
import TerserPlugin from "terser-webpack-plugin/dist/index.js"
import JsonMinimizerPlugin from "json-minimizer-webpack-plugin"
import HTMLMinimizerPlugin from "html-minimizer-webpack-plugin"
import path from "path"

export default {
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
    ],
  },
  entry: "./dist/bootstrap.js",
  output: {
    path: path.resolve(path.dirname(""), "../www/"),
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
}
