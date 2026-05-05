import { QuartzComponent, QuartzComponentConstructor, QuartzComponentProps } from "./types"
import type { StringResource } from "../util/resources"

const mobileQuery = "(max-width: 800px)"

function gateScript(script: string): string {
  return `
const media = window.matchMedia(${JSON.stringify(mobileQuery)})
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
  const MobileOnly: QuartzComponent = (props: QuartzComponentProps) => {
    return (
      <div class="mobile-only">
        <Component {...props} />
      </div>
    )
  }

  MobileOnly.displayName = component.displayName
  MobileOnly.afterDOMLoaded = gateResources(component?.afterDOMLoaded)
  MobileOnly.beforeDOMLoaded = gateResources(component?.beforeDOMLoaded)
  MobileOnly.css = component?.css
  return MobileOnly
}) satisfies QuartzComponentConstructor<QuartzComponent>
