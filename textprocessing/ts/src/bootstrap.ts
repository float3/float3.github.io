import("./index.js").catch((e) => console.error("Error importing `index.js`:", e))
import { transformLeftToRight, transformRightToLeft } from "./index.js"

declare global {
    interface Window {
        transformLeftToRight: typeof transformLeftToRight;
        transformRightToLeft: typeof transformRightToLeft;
    }
}

window.transformLeftToRight = transformLeftToRight
window.transformRightToLeft = transformRightToLeft
