const blankValues = new Set(["", "-"])

function parseSortDate(value: string): number | null {
  const match = value.match(/^([A-Za-z]+)\.?\s+(\d{1,2}),\s+(\d{4})$/)
  if (!match) return null

  const months: Record<string, number> = {
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

function cellText(cell: HTMLTableCellElement | undefined): string {
  return (cell?.textContent ?? "").replace(/\s+/g, " ").trim()
}

type SortType = "checkbox" | "date" | "number" | "text"
type SortValue = string | number | null

function sortValue(cell: HTMLTableCellElement | undefined, type: SortType): SortValue {
  const checkbox = cell?.querySelector("input[type='checkbox']")
  if (type === "checkbox") return checkbox instanceof HTMLInputElement && checkbox.checked ? 1 : 0

  const text = cellText(cell)
  if (blankValues.has(text)) return null

  if (type === "number") return Number(text.replace(/,/g, ""))
  if (type === "date") return parseSortDate(text)
  return text.toLocaleLowerCase()
}

function inferColumnType(rows: HTMLTableRowElement[], columnIndex: number): SortType {
  if (rows.some((row) => row.cells[columnIndex]?.querySelector("input[type='checkbox']"))) {
    return "checkbox"
  }

  const values = rows
    .map((row) => cellText(row.cells[columnIndex]))
    .filter((value) => !blankValues.has(value))

  if (values.length === 0) return "text"
  if (values.every((value) => /^-?\d+(?:\.\d+)?$/.test(value.replace(/,/g, "")))) {
    return "number"
  }
  if (values.every((value) => parseSortDate(value) !== null)) {
    return "date"
  }

  return "text"
}

function compareValues(left: SortValue, right: SortValue, direction: "asc" | "desc"): number {
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

function sortTable(table: HTMLTableElement, columnIndex: number): void {
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

function initializeSortableTables(): void {
  document.querySelectorAll<HTMLTableElement>(".table-container > table").forEach((table) => {
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
      button.setAttribute("aria-label", `Sort by ${cellText(header) || `column ${columnIndex + 1}`}`)

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
