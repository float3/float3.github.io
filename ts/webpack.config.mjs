import path from "node:path"
import { fileURLToPath } from "node:url"
import TerserPlugin from "terser-webpack-plugin"

const tsDir = fileURLToPath(new URL(".", import.meta.url))
const contentJsDir = fileURLToPath(new URL("../content/js", import.meta.url))

const config = {
  context: tsDir,
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
    adventofcode: "./dist/aoc.js",
    tuningplayground: "./dist/tuningplayground.js",
    textprocessing: "./dist/textprocessing.js",
    polyrhythm: "./dist/polyrhythm.js",
    therenderingequation: "./dist/therenderingequation.js",
    movies: "./dist/movies.js",
    trolley: "./dist/trolley.js",
    chars: "./dist/chars.js",
    pokemon: "./dist/pokemon.js",
    bayestheorem: "./dist/bayestheorem.js",
    photography: "./dist/photography.js",
    audiooscilloscope: "./dist/audiooscilloscope.js",
    abcnotation: "./dist/abcnotation.js",
  },
  output: {
    path: path.resolve(contentJsDir),
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

export default config
