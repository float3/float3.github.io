/**
 * The bundle that turns each compiled wasm package into something a page can
 * load.
 *
 * TypeScript rather than JavaScript because everything here is; webpack-cli
 * loads it through jiti, which is why that is a dependency. It sits outside
 * `src`, so `tsc` and eslint do not see it — jiti strips the types at load
 * time and webpack validates the shape.
 */

import path from "node:path"
import { fileURLToPath } from "node:url"
import TerserPlugin from "terser-webpack-plugin"
import type { Configuration } from "webpack"

const tsDir = fileURLToPath(new URL(".", import.meta.url))
const contentJsDir = fileURLToPath(new URL("../content/js", import.meta.url))

const config: Configuration = {
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
    gallery: "./dist/gallery.js",
    chars: "./dist/chars.js",
    bayestheorem: "./dist/bayestheorem.js",
    photography: "./dist/photography.js",
    audiooscilloscope: "./dist/audiooscilloscope.js",
    background: "./dist/background.js",
    you: "./dist/you.js",
    comments: "./dist/comments.js",
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
