export interface TabConfig {
  tabCount: number
  subTabCount: number
}

export function createTabs(container: HTMLElement, config: TabConfig) {
  const { tabCount, subTabCount } = config
  const tabsWrapper = document.createElement("div")
  tabsWrapper.className = "tabs"

  for (let i = 0; i <= tabCount; i++) {
    const tabButton = document.createElement("button")
    if (i === 1) tabButton.classList.add("active")
    tabButton.textContent = `${2015 + i}`
    tabButton.dataset.tab = `tab${i}`
    tabsWrapper.appendChild(tabButton)
  }

  container.appendChild(tabsWrapper)

  for (let i = 1; i <= tabCount; i++) {
    const tabContent = document.createElement("div")
    tabContent.id = `tab${i}`
    tabContent.className = i === 1 ? "tab-content active" : "tab-content"

    const subtabsWrapper = document.createElement("div")
    subtabsWrapper.className = "subtabs"
    for (let j = 1; j <= subTabCount; j++) {
      const subtabButton = document.createElement("button")
      if (j === 1) subtabButton.classList.add("active")
      subtabButton.textContent = `day ${j}`
      subtabButton.dataset.subtab = `subtab${i}-${j}`
      subtabsWrapper.appendChild(subtabButton)
    }

    tabContent.appendChild(subtabsWrapper)

    for (let j = 1; j <= subTabCount; j++) {
      const subtabContent = document.createElement("div")
      subtabContent.id = `subtab${i}-${j}`
      subtabContent.className = j === 1 ? "subtab-content active" : "subtab-content"

      const fields = document.createElement("div")
      fields.className = "fields"

      const leftCol = document.createElement("div")
      leftCol.className = "left-col"
      const leftInput = document.createElement("input")
      leftInput.type = "text"
      leftInput.placeholder = "Field 1"
      leftCol.appendChild(leftInput)

      const rightCol = document.createElement("div")
      rightCol.className = "right-col"
      for (let k = 2; k <= 3; k++) {
        const rightInput = document.createElement("input")
        rightInput.type = "text"
        rightInput.placeholder = `Field ${k}`
        rightCol.appendChild(rightInput)
      }

      fields.appendChild(leftCol)
      fields.appendChild(rightCol)
      subtabContent.appendChild(fields)
      tabContent.appendChild(subtabContent)
    }

    container.appendChild(tabContent)
  }

  tabsWrapper.querySelectorAll("button").forEach((btn) => {
    btn.addEventListener("click", () => {
      tabsWrapper.querySelectorAll("button").forEach((b) => b.classList.remove("active"))
      btn.classList.add("active")
      const target = btn.dataset.tab
      container.querySelectorAll(".tab-content").forEach((tc) => {
        tc.classList.remove("active")
        if (tc.id === target) tc.classList.add("active")
      })
    })
  })

  container.querySelectorAll(".subtabs").forEach((stWrapper) => {
    stWrapper.querySelectorAll("button").forEach((btn) => {
      btn.addEventListener("click", () => {
        stWrapper.querySelectorAll("button").forEach((b) => b.classList.remove("active"))
        btn.classList.add("active")
        const parentContent = btn.closest(".tab-content") as HTMLElement
        const target = btn.dataset.subtab
        parentContent.querySelectorAll(".subtab-content").forEach((stc) => {
          stc.classList.remove("active")
          if (stc.id === target) stc.classList.add("active")
        })
      })
    })
  })
}
