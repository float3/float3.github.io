import { QuartzComponent, QuartzComponentConstructor, QuartzComponentProps } from "./types"
import type { StringResource } from "../util/resources"

const desktopQuery = "(min-width: 801px)"

function gateScript(script: string): string {
  return `
const media = window.matchMedia(${JSON.stringify(desktopQuery)})
let hasRun = false
const runIfMatched = () => {
  if (hasRun || !media.matches) return
  hasRun = true
  ${script}
}
runIfMatched()
if (typeof media.addEventListener === "function") {
  media.addEventListener("change", runIfMatched)
} else if (typeof media.addListener === "function") {
  media.addListener(runIfMatched)
}
`
}

function gateResources(resource: StringResource): StringResource {
  if (!resource) return resource
  if (Array.isArray(resource)) return resource.map(gateScript)
  return gateScript(resource)
}

export default ((component: QuartzComponent) => {
  const Component = component
  const DesktopOnly: QuartzComponent = (props: QuartzComponentProps) => {
    return (
      <div class="desktop-only">
        <Component {...props} />
      </div>
    )
  }

  DesktopOnly.displayName = component.displayName
  DesktopOnly.afterDOMLoaded = gateResources(component?.afterDOMLoaded)
  DesktopOnly.beforeDOMLoaded = gateResources(component?.beforeDOMLoaded)
  DesktopOnly.css = component?.css
  return DesktopOnly
}) satisfies QuartzComponentConstructor<QuartzComponent>
