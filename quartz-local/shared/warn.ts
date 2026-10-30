import { styleText } from "util"

/** A build warning, in the colour Quartz uses for its own. */
export function warn(message: string): void {
  console.log(styleText("yellow", `\nWarning: ${message}`))
}
