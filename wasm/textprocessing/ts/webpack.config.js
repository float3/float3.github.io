import TerserPlugin from "terser-webpack-plugin/dist/index.js"
import path from "path"

export default {
  module: {
    rules: [
      {
        type: "webassembly/async",
        test: /\.wasm$/,
      },
    ],
  },
  entry: "./dist/bootstrap.js",
  output: {
    path: path.resolve(path.dirname(""), "../../../content/tools/textprocessing"),
    filename: "bootstrap.js",
  },
  target: "web",
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
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
}
