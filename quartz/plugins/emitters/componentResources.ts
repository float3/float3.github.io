import { FullSlug, joinSegments } from "../../util/path"
import { QuartzEmitterPlugin } from "../types"

// @ts-ignore
import spaRouterScript from "../../components/scripts/spa.inline"
// @ts-ignore
import popoverScript from "../../components/scripts/popover.inline"
import baseStyles from "../../styles/base.scss"
import customStyles from "../../styles/custom.scss"
import popoverStyle from "../../components/styles/popover.scss"
import { BuildCtx } from "../../util/ctx"
import { QuartzComponent } from "../../components/types"
import {
  googleFontHref,
  googleFontSubsetHref,
  joinStyles,
  processGoogleFonts,
} from "../../util/theme"
import { Features, transform } from "lightningcss"
import { transform as transpile } from "esbuild"
import { write } from "./helpers"

type ComponentResources = {
  css: string[]
  beforeDOMLoaded: string[]
  afterDOMLoaded: string[]
}

const sortableTablesScript = `
const blankValues = new Set(["", "-"])

const parseSortDate = (value) => {
  const match = value.match(/^([A-Za-z]+)\\.?\\s+(\\d{1,2}),\\s+(\\d{4})$/)
  if (!match) return null

  const months = {
    jan: 0,
    january: 0,
    feb: 1,
    february: 1,
    mar: 2,
    march: 2,
    apr: 3,
    april: 3,
    may: 4,
    jun: 5,
    june: 5,
    jul: 6,
    july: 6,
    aug: 7,
    august: 7,
    sep: 8,
    sept: 8,
    september: 8,
    oct: 9,
    october: 9,
    nov: 10,
    november: 10,
    dec: 11,
    december: 11,
  }

  const month = months[match[1].toLowerCase()]
  if (month === undefined) return null

  return Date.UTC(Number(match[3]), month, Number(match[2]))
}

const cellText = (cell) => (cell?.textContent ?? "").replace(/\\s+/g, " ").trim()

const sortValue = (cell, type) => {
  const checkbox = cell?.querySelector("input[type='checkbox']")
  if (type === "checkbox") return checkbox?.checked ? 1 : 0

  const text = cellText(cell)
  if (blankValues.has(text)) return null

  if (type === "number") return Number(text.replace(/,/g, ""))
  if (type === "date") return parseSortDate(text)
  return text.toLocaleLowerCase()
}

const inferColumnType = (rows, columnIndex) => {
  if (rows.some((row) => row.cells[columnIndex]?.querySelector("input[type='checkbox']"))) {
    return "checkbox"
  }

  const values = rows
    .map((row) => cellText(row.cells[columnIndex]))
    .filter((value) => !blankValues.has(value))

  if (values.length === 0) return "text"
  if (values.every((value) => /^-?\\d+(?:\\.\\d+)?$/.test(value.replace(/,/g, "")))) {
    return "number"
  }
  if (values.every((value) => parseSortDate(value) !== null)) {
    return "date"
  }

  return "text"
}

const compareValues = (left, right, direction) => {
  if (left === null && right === null) return 0
  if (left === null) return 1
  if (right === null) return -1

  const result =
    typeof left === "string" && typeof right === "string"
      ? left.localeCompare(right, undefined, { numeric: true, sensitivity: "base" })
      : left < right
        ? -1
        : left > right
          ? 1
          : 0

  return direction === "asc" ? result : -result
}

const sortTable = (table, columnIndex) => {
  const tbody = table.tBodies[0]
  if (!tbody) return

  const headers = Array.from(table.tHead?.rows[0]?.cells ?? [])
  const rows = Array.from(tbody.rows)
  const currentColumn = table.dataset.sortColumn
  const direction =
    currentColumn === String(columnIndex) && table.dataset.sortDirection === "asc" ? "desc" : "asc"
  const type = inferColumnType(rows, columnIndex)

  rows
    .map((row, index) => {
      if (!row.dataset.sortOriginalIndex) {
        row.dataset.sortOriginalIndex = String(index)
      }

      return {
        row,
        value: sortValue(row.cells[columnIndex], type),
        originalIndex: Number(row.dataset.sortOriginalIndex),
      }
    })
    .sort((left, right) => {
      const result = compareValues(left.value, right.value, direction)
      return result || left.originalIndex - right.originalIndex
    })
    .forEach(({ row }) => tbody.appendChild(row))

  table.dataset.sortColumn = String(columnIndex)
  table.dataset.sortDirection = direction

  headers.forEach((header, index) => {
    header.setAttribute(
      "aria-sort",
      index === columnIndex ? (direction === "asc" ? "ascending" : "descending") : "none",
    )
  })
}

const initializeSortableTables = () => {
  document.querySelectorAll(".table-container > table").forEach((table) => {
    if (table.dataset.sortableInitialized === "true") return

    const headerRow = table.tHead?.rows[0]
    const tbody = table.tBodies[0]
    if (!headerRow || !tbody || tbody.rows.length === 0) return

    table.dataset.sortableInitialized = "true"
    table.classList.add("sortable-table")

    Array.from(tbody.rows).forEach((row, index) => {
      row.dataset.sortOriginalIndex = String(index)
    })

    Array.from(headerRow.cells).forEach((header, columnIndex) => {
      header.setAttribute("aria-sort", "none")

      const button = document.createElement("button")
      button.type = "button"
      button.className = "sortable-table-header"
      button.setAttribute("aria-label", \`Sort by \${cellText(header) || \`column \${columnIndex + 1}\`}\`)

      while (header.firstChild) {
        button.appendChild(header.firstChild)
      }

      const indicator = document.createElement("span")
      indicator.className = "sortable-table-indicator"
      indicator.setAttribute("aria-hidden", "true")
      button.appendChild(indicator)

      const listener = () => sortTable(table, columnIndex)
      button.addEventListener("click", listener)
      window.addCleanup?.(() => button.removeEventListener("click", listener))

      header.appendChild(button)
    })
  })
}
document.addEventListener("nav", initializeSortableTables)
document.addEventListener("render", initializeSortableTables)
initializeSortableTables()
`

function getComponentResources(ctx: BuildCtx): ComponentResources {
  const allComponents: Set<QuartzComponent> = new Set()

  for (const emitter of ctx.cfg.plugins.emitters) {
    const components = emitter.getQuartzComponents?.(ctx) ?? []
    for (const component of components) {
      allComponents.add(component)
    }
  }

  const componentResources = {
    css: new Set<string>(),
    beforeDOMLoaded: new Set<string>(),
    afterDOMLoaded: new Set<string>(),
  }

  function normalizeResource(resource: string | string[] | undefined): string[] {
    if (!resource) return []
    if (Array.isArray(resource)) return resource
    return [resource]
  }

  for (const component of allComponents) {
    if (!component) continue
    const { css, beforeDOMLoaded, afterDOMLoaded } = component
    const normalizedCss = normalizeResource(css)
    const normalizedBeforeDOMLoaded = normalizeResource(beforeDOMLoaded)
    const normalizedAfterDOMLoaded = normalizeResource(afterDOMLoaded)

    normalizedCss.forEach((c) => componentResources.css.add(c))
    normalizedBeforeDOMLoaded.forEach((b) => componentResources.beforeDOMLoaded.add(b))
    normalizedAfterDOMLoaded.forEach((a) => componentResources.afterDOMLoaded.add(a))
  }

  return {
    css: [...componentResources.css],
    beforeDOMLoaded: [...componentResources.beforeDOMLoaded],
    afterDOMLoaded: [...componentResources.afterDOMLoaded],
  }
}

async function joinScripts(scripts: string[]): Promise<string> {
  // wrap with iife to prevent scope collision
  const script = scripts.map((script) => `(function () {${script}})();`).join("\n")

  // minify with esbuild
  const res = await transpile(script, {
    minify: true,
  })

  return res.code
}

function addGlobalPageResources(ctx: BuildCtx, componentResources: ComponentResources) {
  const cfg = ctx.cfg.configuration

  // popovers
  if (cfg.enablePopovers) {
    componentResources.afterDOMLoaded.push(popoverScript)
    componentResources.css.push(popoverStyle)
  }

  if (cfg.analytics?.provider === "google") {
    const tagId = cfg.analytics.tagId
    componentResources.afterDOMLoaded.push(`
      const gtagScript = document.createElement('script');
      gtagScript.src = 'https://www.googletagmanager.com/gtag/js?id=${tagId}';
      gtagScript.defer = true;
      gtagScript.onload = () => {
        window.dataLayer = window.dataLayer || [];
        function gtag() {
          dataLayer.push(arguments);
        }
        gtag('js', new Date());
        gtag('config', '${tagId}', { send_page_view: false });
        gtag('event', 'page_view', { page_title: document.title, page_location: location.href });
        document.addEventListener('nav', () => {
          gtag('event', 'page_view', { page_title: document.title, page_location: location.href });
        });
      };

      document.head.appendChild(gtagScript);
    `)
  } else if (cfg.analytics?.provider === "plausible") {
    const plausibleHost = cfg.analytics.host ?? "https://plausible.io"
    componentResources.afterDOMLoaded.push(`
      const plausibleScript = document.createElement('script');
      plausibleScript.src = '${plausibleHost}/js/script.manual.js';
      plausibleScript.setAttribute('data-domain', location.hostname);
      plausibleScript.defer = true;
      plausibleScript.onload = () => {
        window.plausible = window.plausible || function () { (window.plausible.q = window.plausible.q || []).push(arguments); };
        plausible('pageview');
        document.addEventListener('nav', () => {
          plausible('pageview');
        });
      };

      document.head.appendChild(plausibleScript);
    `)
  } else if (cfg.analytics?.provider === "umami") {
    componentResources.afterDOMLoaded.push(`
      const umamiScript = document.createElement("script");
      umamiScript.src = "${cfg.analytics.host ?? "https://analytics.umami.is"}/script.js";
      umamiScript.setAttribute("data-website-id", "${cfg.analytics.websiteId}");
      umamiScript.setAttribute("data-auto-track", "true");
      umamiScript.defer = true;

      document.head.appendChild(umamiScript);
    `)
  } else if (cfg.analytics?.provider === "goatcounter") {
    componentResources.afterDOMLoaded.push(`
      const goatcounterScriptPre = document.createElement('script');
      goatcounterScriptPre.textContent = \`
        window.goatcounter = { no_onload: true };
      \`;
      document.head.appendChild(goatcounterScriptPre);

      const endpoint = "https://${cfg.analytics.websiteId}.${cfg.analytics.host ?? "goatcounter.com"}/count";
      const goatcounterScript = document.createElement('script');
      goatcounterScript.src = "${cfg.analytics.scriptSrc ?? "https://gc.zgo.at/count.js"}";
      goatcounterScript.defer = true;
      goatcounterScript.setAttribute('data-goatcounter', endpoint);
      goatcounterScript.onload = () => {
        window.goatcounter.endpoint = endpoint;
        goatcounter.count({ path: location.pathname });
        document.addEventListener('nav', () => {
          goatcounter.count({ path: location.pathname });
        });
      };

      document.head.appendChild(goatcounterScript);
    `)
  } else if (cfg.analytics?.provider === "posthog") {
    componentResources.afterDOMLoaded.push(`
      const posthogScript = document.createElement("script");
      posthogScript.innerHTML= \`!function(t,e){var o,n,p,r;e.__SV||(window.posthog=e,e._i=[],e.init=function(i,s,a){function g(t,e){var o=e.split(".");2==o.length&&(t=t[o[0]],e=o[1]),t[e]=function(){t.push([e].concat(Array.prototype.slice.call(arguments,0)))}}(p=t.createElement("script")).type="text/javascript",p.async=!0,p.src=s.api_host+"/static/array.js",(r=t.getElementsByTagName("script")[0]).parentNode.insertBefore(p,r);var u=e;for(void 0!==a?u=e[a]=[]:a="posthog",u.people=u.people||[],u.toString=function(t){var e="posthog";return"posthog"!==a&&(e+="."+a),t||(e+=" (stub)"),e},u.people.toString=function(){return u.toString(1)+".people (stub)"},o="capture identify alias people.set people.set_once set_config register register_once unregister opt_out_capturing has_opted_out_capturing opt_in_capturing reset isFeatureEnabled onFeatureFlags getFeatureFlag getFeatureFlagPayload reloadFeatureFlags group updateEarlyAccessFeatureEnrollment getEarlyAccessFeatures getActiveMatchingSurveys getSurveys onSessionId".split(" "),n=0;n<o.length;n++)g(u,o[n]);e._i.push([i,s,a])},e.__SV=1)}(document,window.posthog||[]);
      posthog.init('${cfg.analytics.apiKey}', {
        api_host: '${cfg.analytics.host ?? "https://app.posthog.com"}',
        capture_pageview: false,
      });
      document.addEventListener('nav', () => {
        posthog.capture('$pageview', { path: location.pathname });
      })\`

      document.head.appendChild(posthogScript);
    `)
  } else if (cfg.analytics?.provider === "tinylytics") {
    const siteId = cfg.analytics.siteId
    componentResources.afterDOMLoaded.push(`
      const tinylyticsScript = document.createElement('script');
      tinylyticsScript.src = 'https://tinylytics.app/embed/${siteId}.js?spa';
      tinylyticsScript.defer = true;
      tinylyticsScript.onload = () => {
        window.tinylytics.triggerUpdate();
        document.addEventListener('nav', () => {
          window.tinylytics.triggerUpdate();
        });
      };

      document.head.appendChild(tinylyticsScript);
    `)
  } else if (cfg.analytics?.provider === "cabin") {
    componentResources.afterDOMLoaded.push(`
      const cabinScript = document.createElement("script")
      cabinScript.src = "${cfg.analytics.host ?? "https://scripts.withcabin.com"}/hello.js"
      cabinScript.defer = true
      document.head.appendChild(cabinScript)
    `)
  } else if (cfg.analytics?.provider === "clarity") {
    componentResources.afterDOMLoaded.push(`
      const clarityScript = document.createElement("script")
      clarityScript.innerHTML= \`(function(c,l,a,r,i,t,y){c[a]=c[a]||function(){(c[a].q=c[a].q||[]).push(arguments)};
      t=l.createElement(r);t.defer=1;t.src="https://www.clarity.ms/tag/"+i;
      y=l.getElementsByTagName(r)[0];y.parentNode.insertBefore(t,y);
      })(window, document, "clarity", "script", "${cfg.analytics.projectId}");\`
      document.head.appendChild(clarityScript)
    `)
  } else if (cfg.analytics?.provider === "matomo") {
    componentResources.afterDOMLoaded.push(`
      const matomoScript = document.createElement("script");
      matomoScript.innerHTML = \`
      let _paq = window._paq = window._paq || [];

      // Track SPA navigation
      // https://developer.matomo.org/guides/spa-tracking
      document.addEventListener("nav", () => {
        _paq.push(['setCustomUrl', location.pathname]);
        _paq.push(['setDocumentTitle', document.title]);
        _paq.push(['trackPageView']);
      });

      _paq.push(['trackPageView']);
      _paq.push(['enableLinkTracking']);
      (function() {
        const u="//${cfg.analytics.host}/";
        _paq.push(['setTrackerUrl', u+'matomo.php']);
        _paq.push(['setSiteId', ${cfg.analytics.siteId}]);
        const d=document, g=d.createElement('script'), s=d.getElementsByTagName
('script')[0];
        g.type='text/javascript'; g.async=true; g.src=u+'matomo.js'; s.parentNode.insertBefore(g,s);
      })();
      \`
      document.head.appendChild(matomoScript);
    `)
  } else if (cfg.analytics?.provider === "vercel") {
    /**
     * script from {@link https://vercel.com/docs/analytics/quickstart?framework=html#add-the-script-tag-to-your-site|Vercel Docs}
     */
    componentResources.beforeDOMLoaded.push(`
      window.va = window.va || function () { (window.vaq = window.vaq || []).push(arguments); };
    `)
    componentResources.afterDOMLoaded.push(`
      const vercelInsightsScript = document.createElement("script")
      vercelInsightsScript.src = "/_vercel/insights/script.js"
      vercelInsightsScript.defer = true
      document.head.appendChild(vercelInsightsScript)
    `)
  } else if (cfg.analytics?.provider === "rybbit") {
    componentResources.afterDOMLoaded.push(`
      const rybbitScript = document.createElement("script");
      rybbitScript.src = "${cfg.analytics.host ?? "https://app.rybbit.io"}/api/script.js";
      rybbitScript.setAttribute("data-site-id", "${cfg.analytics.siteId}");
      rybbitScript.async = true;
      rybbitScript.defer = true;

      document.head.appendChild(rybbitScript);
    `)
  }

  if (cfg.enableSPA) {
    componentResources.afterDOMLoaded.push(sortableTablesScript)
    componentResources.afterDOMLoaded.push(spaRouterScript)
  } else {
    componentResources.afterDOMLoaded.push(sortableTablesScript)
    componentResources.afterDOMLoaded.push(`
      window.spaNavigate = (url, _) => window.location.assign(url)
      window.addCleanup = () => {}
      const event = new CustomEvent("nav", { detail: { url: document.body.dataset.slug } })
      document.dispatchEvent(event)
    `)
  }
}

// This emitter should not update the `resources` parameter. If it does, partial
// rebuilds may not work as expected.
export const ComponentResources: QuartzEmitterPlugin = () => {
  return {
    name: "ComponentResources",
    async *emit(ctx, _content, _resources) {
      const cfg = ctx.cfg.configuration
      // component specific scripts and styles
      const componentResources = getComponentResources(ctx)
      let googleFontsStyleSheet = ""
      if (cfg.theme.fontOrigin === "local") {
        // let the user do it themselves in css
      } else if (cfg.theme.fontOrigin === "googleFonts" && !cfg.theme.cdnCaching) {
        // when cdnCaching is true, we link to google fonts in Head.tsx
        const theme = ctx.cfg.configuration.theme
        const response = await fetch(googleFontHref(theme))
        googleFontsStyleSheet = await response.text()

        if (theme.typography.title) {
          const title = ctx.cfg.configuration.pageTitle
          const response = await fetch(googleFontSubsetHref(theme, title))
          googleFontsStyleSheet += `\n${await response.text()}`
        }

        if (!cfg.baseUrl) {
          throw new Error(
            "baseUrl must be defined when using Google Fonts without cfg.theme.cdnCaching",
          )
        }

        const { processedStylesheet, fontFiles } = await processGoogleFonts(
          googleFontsStyleSheet,
          cfg.baseUrl,
        )
        googleFontsStyleSheet = processedStylesheet

        // Download and save font files
        for (const fontFile of fontFiles) {
          const res = await fetch(fontFile.url)
          if (!res.ok) {
            throw new Error(`Failed to fetch font ${fontFile.filename}`)
          }

          const buf = await res.arrayBuffer()
          yield write({
            ctx,
            slug: joinSegments("static", "fonts", fontFile.filename) as FullSlug,
            ext: `.${fontFile.extension}`,
            content: Buffer.from(buf),
          })
        }
      }

      // important that this goes *after* component scripts
      // as the "nav" event gets triggered here and we should make sure
      // that everyone else had the chance to register a listener for it
      addGlobalPageResources(ctx, componentResources)

      const quartzBase = joinStyles(
        ctx.cfg.configuration.theme,
        googleFontsStyleSheet,
        ...componentResources.css,
        baseStyles,
      )
      const stylesheet = `@layer quartz-base {\n${quartzBase}\n}\n${customStyles}`

      const [prescript, postscript] = await Promise.all([
        joinScripts(componentResources.beforeDOMLoaded),
        joinScripts(componentResources.afterDOMLoaded),
      ])

      yield write({
        ctx,
        slug: "index" as FullSlug,
        ext: ".css",
        content: transform({
          filename: "index.css",
          code: Buffer.from(stylesheet),
          minify: true,
          targets: {
            safari: (15 << 16) | (6 << 8), // 15.6
            ios_saf: (15 << 16) | (6 << 8), // 15.6
            edge: 115 << 16,
            firefox: 102 << 16,
            chrome: 109 << 16,
          },
          include: Features.MediaQueries,
        }).code.toString(),
      })

      yield write({
        ctx,
        slug: "prescript" as FullSlug,
        ext: ".js",
        content: prescript,
      })

      yield write({
        ctx,
        slug: "postscript" as FullSlug,
        ext: ".js",
        content: postscript,
      })
    },
    async *partialEmit() {},
  }
}
