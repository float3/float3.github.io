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
  entry: {
    glsl2hlsl: "./dist/glsl.js",
    aoc: "./dist/aoc.js",
    tuningplayground: "./dist/tuningplayground.js",
    textprocessing: "./dist/textprocessing.js",
    polyrhythm: "./dist/polyrhythm.js",
    therenderingequation: "./dist/therenderingequation.js",
    movies: "./dist/movies.js",
    trolley: "./dist/trolley.js",
    chars: "./dist/chars.js",
    pokemon: "./dist/pokemon.js",
  },
  output: {
    path: path.resolve(path.dirname(""), "../content/js"),
    filename: "[name].js",
    publicPath: "/js/",
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
