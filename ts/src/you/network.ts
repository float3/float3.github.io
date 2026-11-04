/**
 * The reader's public address, asked of three services the page's
 * `connect-src` already allows. Asking more than one is what shows split
 * routing: two answers in the same address family mean two exits.
 */

const IPV4_ONLY = "https://api.ipify.org?format=json"
const PREFERRED = "https://api64.ipify.org?format=json"
const GEOIP = "https://api.ip.sb/geoip"

/** What `api.ip.sb/geoip` answers, as far as the page reads it. */
export interface GeoIP {
  ip: string
  country?: string
  country_code?: string
  region?: string
  city?: string
  isp?: string
  organization?: string
  asn?: number
  asn_organization?: string
  timezone?: string
  offset?: number
}

export interface Network {
  ipv4?: string
  preferred?: string
  geo?: GeoIP
}

export enum Family {
  V4 = "IPv4",
  V6 = "IPv6",
}

/**
 * Networks that rent out machines rather than connect homes. An address in one
 * of them is a VPN, a proxy, a relay or a server, not a person's router.
 */
const HOSTING = [
  "akamai",
  "alibaba",
  "amazon",
  "cdn77",
  "choopa",
  "cloudflare",
  "contabo",
  "datacamp",
  "digitalocean",
  "google cloud",
  "hetzner",
  "leaseweb",
  "linode",
  "m247",
  "microsoft",
  "mullvad",
  "oracle",
  "ovh",
  "packethub",
  "proton",
  "tencent",
  "vultr",
  "zenlayer",
]

async function fetchJSON<T>(url: string): Promise<T | undefined> {
  try {
    const response = await fetch(url, { cache: "no-store" })
    if (!response.ok) return undefined

    return (await response.json()) as T
  } catch {
    return undefined
  }
}

export async function lookupNetwork(): Promise<Network> {
  const [ipv4, preferred, geo] = await Promise.all([
    fetchJSON<{ ip: string }>(IPV4_ONLY),
    fetchJSON<{ ip: string }>(PREFERRED),
    fetchJSON<GeoIP>(GEOIP),
  ])

  return { ipv4: ipv4?.ip, preferred: preferred?.ip, geo }
}

export function familyOf(address: string): Family {
  return address.includes(":") ? Family.V6 : Family.V4
}

/** Every distinct address the services saw, in the order asked. */
export function addressesOf(network: Network): string[] {
  const seen = [network.ipv4, network.preferred, network.geo?.ip]

  return [...new Set(seen.filter((address): address is string => !!address))]
}

/** How many exits share one address family; more than one is split routing. */
export function exitCount(network: Network): number {
  const addresses = addressesOf(network)
  const v4 = addresses.filter((address) => familyOf(address) === Family.V4).length
  const v6 = addresses.length - v4

  return Math.max(v4, v6)
}

export function ownerOf(geo: GeoIP): string | undefined {
  return geo.isp ?? geo.organization ?? geo.asn_organization
}

export function isHosting(geo: GeoIP): boolean {
  const names = [geo.isp, geo.organization, geo.asn_organization].join(" ").toLowerCase()

  return HOSTING.some((name) => names.includes(name))
}
