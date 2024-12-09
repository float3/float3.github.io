import CopyPlugin from "copy-webpack-plugin";
import TerserPlugin from "terser-webpack-plugin/dist/index.js";
import JsonMinimizerPlugin from "json-minimizer-webpack-plugin";
import HTMLMinimizerPlugin from "html-minimizer-webpack-plugin";
import path from "path";

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
      new JsonMinimizerPlugin(),
      new HTMLMinimizerPlugin(),
    ],
  },
  experiments: {
    syncWebAssembly: true,
  },
};
