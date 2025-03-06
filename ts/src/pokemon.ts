console.time("importTime")
import("./pokemon/index.js")
  .then(() => console.timeEnd("importTime"))
  .catch((e) => console.error("Error importing `.js`:", e))
