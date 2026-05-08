#!/usr/bin/env bun

import fs from "node:fs"
import path from "node:path"
import YAML from "yaml"
import { installPlugins, parsePluginSource } from "./gitLoader.js"
import type { GitPluginSpec } from "./gitLoader.js"
import type { PluginSource, QuartzPluginsJson } from "./types"

const configCandidates = [
  "quartz.config.yaml",
  "quartz.plugins.json",
  "quartz.config.default.yaml",
  "quartz.plugins.default.json",
]

function resolveConfigPath(): string | undefined {
  return configCandidates
    .map((candidate) => path.resolve(process.cwd(), candidate))
    .find((candidate) => fs.existsSync(candidate))
}

function readPluginsConfig(): QuartzPluginsJson | undefined {
  const configPath = resolveConfigPath()
  if (!configPath) {
    return undefined
  }

  const raw = fs.readFileSync(configPath, "utf-8")
  if (configPath.endsWith(".yaml") || configPath.endsWith(".yml")) {
    return YAML.parse(raw) as QuartzPluginsJson
  }

  return JSON.parse(raw) as QuartzPluginsJson
}

function enabledPluginSources(config: QuartzPluginsJson | undefined): PluginSource[] {
  return (config?.plugins ?? []).filter((entry) => entry.enabled).map((entry) => entry.source)
}

function uniqueSpecs(sources: PluginSource[]): GitPluginSpec[] {
  const specs = new Map<string, GitPluginSpec>()
  for (const source of sources) {
    const spec = parsePluginSource(source)
    if (!specs.has(spec.name)) {
      specs.set(spec.name, spec)
    }
  }

  return [...specs.values()]
}

async function main() {
  const specs = uniqueSpecs(enabledPluginSources(readPluginsConfig()))

  if (specs.length === 0) {
    console.log("No enabled plugins to install.")
    return
  }

  console.log(`Installing ${specs.length} enabled plugin(s)...`)
  const installed = await installPlugins(specs, { verbose: true })

  if (installed.size === specs.length) {
    console.log("All plugins installed successfully")
  } else {
    console.error(`Only ${installed.size}/${specs.length} plugins installed`)
    process.exit(1)
  }
}

main().catch((err) => {
  console.error("Failed to install plugins:", err)
  process.exit(1)
})
