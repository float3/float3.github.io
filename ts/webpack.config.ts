/**
 * The bundle that turns each compiled wasm package into something a page can
 * load.
 *
 * TypeScript rather than JavaScript because everything here is; webpack-cli
 * loads it through jiti, which is why that is a dependency. It sits outside
 * `src`, so `tsc` and eslint do not see it — jiti strips the types at load
 * time and webpack validates the shape.
 */

import fs from "node:fs"
import path from "node:path"
import { fileURLToPath } from "node:url"
import TerserPlugin from "terser-webpack-plugin"
import type { Configuration } from "webpack"

const tsDir = fileURLToPath(new URL(".", import.meta.url))
const contentJsDir = fileURLToPath(new URL("../content/js", import.meta.url))
const srcDir = fileURLToPath(new URL("./src", import.meta.url))

/**
 * Every module directly under `src` is a page script, bundled as
 * `/js/<its name>.js`; everything a page script imports lives in a
 * subdirectory. There is no list to keep in step: `cargo test -p site` checks
 * that every bundle is loaded by some page and every page's bundle exists,
 * reading the directory by the same rule.
 */
function pageScripts(): Record<string, string> {
  return Object.fromEntries(
    fs
      .readdirSync(srcDir)
      .filter((name) => name.endsWith(".ts") && !name.endsWith(".d.ts"))
      .map((name) => name.slice(0, -".ts".length))
      .sort()
      .map((name) => [name, `./dist/${name}.js`]),
  )
}

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
  entry: pageScripts(),
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
