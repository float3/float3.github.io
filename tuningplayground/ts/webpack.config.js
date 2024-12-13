import TerserPlugin from "terser-webpack-plugin/dist/index.js"
import path from "path"

export default {
  entry: "./dist/bootstrap.js",
  output: {
    path: path.resolve(path.dirname(""), "../www/"),
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
    syncWebAssembly: true,
  },
}
