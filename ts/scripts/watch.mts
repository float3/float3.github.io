const bun = process.execPath

const commands = [
  { label: "TypeScript", args: ["run", "watch:ts"] },
  { label: "webpack", args: ["run", "watch:webpack"] },
]

const children = commands.map(({ args }) =>
  Bun.spawn([bun, ...args], {
    cwd: process.cwd(),
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  }),
)

let stopping = false
let exitCode = 0

function stopAll(code) {
  if (stopping) {
    return
  }

  stopping = true
  exitCode = code

  for (const child of children) {
    child.kill()
  }
}

process.on("SIGINT", () => stopAll(130))
process.on("SIGTERM", () => stopAll(143))

await Promise.race(
  children.map(async (child, index) => {
    const code = await child.exited
    if (!stopping) {
      if (code !== 0) {
        console.error(`${commands[index].label} exited with code ${code}`)
      }
      stopAll(code)
    }
  }),
)

await Promise.allSettled(children.map((child) => child.exited))
process.exit(exitCode)
