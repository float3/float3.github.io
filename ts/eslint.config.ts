import globals from "globals"
import pluginJs from "@eslint/js"
import tseslint from "typescript-eslint"
import prettierPlugin from "eslint-plugin-prettier"
import prettierConfig from "eslint-config-prettier"
import type { Linter } from "eslint"

const config: Linter.Config[] = [
  { files: ["**/*.ts"] },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  {
    plugins: {
      prettier: prettierPlugin,
    },
    rules: {
      "prettier/prettier": "error",
    },
  },
  prettierConfig,
]

export default config
