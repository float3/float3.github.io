/**
 * The offset a page has to clear to stick something to the top of the window.
 *
 * Below Quartz's `$mobile` breakpoint the left sidebar is laid out as a row
 * above the page rather than a column beside it, and the explorer plugin makes
 * it `position: sticky` — with a transparent background and `z-index: 200`, so
 * anything a page pins at `top: 0` ends up underneath the site's own header
 * with both legible through each other.
 *
 * `custom.scss` carries a guess at that header's height as `--sticky-top`, for
 * a reader with no scripts. This replaces the guess with the measurement,
 * because the header holds a title in a webfont and a search box and is not a
 * fixed number of pixels tall. Above the breakpoint the header is a column
 * beside the page and the offset goes back to nothing.
 */

/** Quartz's own `$mobile`, from `quartz/styles/variables.scss`. */
const MOBILE = "(max-width: 800px)"

const HEADER = ".sidebar.left"

const PROPERTY = "--sticky-top"

/**
 * Keeps `--sticky-top` on the document in step with the header, until the page
 * goes away. Safe to call from more than one page script: the last writer wins
 * and they all measure the same element.
 */
export function followHeader(): void {
  const header = document.querySelector<HTMLElement>(HEADER)
  if (header === null) return

  const narrow = window.matchMedia(MOBILE)
  const measure = (): void => {
    const height = narrow.matches ? header.getBoundingClientRect().height : 0
    document.documentElement.style.setProperty(PROPERTY, `${Math.round(height)}px`)
  }

  measure()
  // The webfonts land after the first measurement and change the height.
  void document.fonts.ready.then(measure)
  window.addEventListener("resize", measure)
  narrow.addEventListener("change", measure)
}
