---
parent: "_index.md"
date: "2026-08-24T13:10:00.000Z"
author: "float3"
authorId: 86748455
history:
  - date: "2026-08-24T13:10:00.000Z"
---

Comments here can carry a `<script>`, so here is one carrying a whole game. Press **run this** — it opens in a sandboxed frame with no way back to this page.

A note on flagging mistakes, since it is the only interesting decision in it: a square turns red when its digit **conflicts with another digit** in its row, column or box — not when it disagrees with a stored solution. Checking against a solution punishes you for finding a different valid one, and it is wrong the moment a puzzle has more than one. Conflict-checking is right whether or not the puzzle is unique. This generator does produce unique puzzles — it only removes a digit while one solution remains — but the rule holds either way.

<div id="game"><div id="board"></div><div id="bar"><button id="again" type="button">new puzzle</button><span id="status"></span></div></div>
<style>
  #game { font: 16px/1.4 system-ui, sans-serif; max-width: 30rem; }
  #board { display: grid; grid-template-columns: repeat(9, 1fr); gap: 0; border: 2px solid currentColor; width: min(100%, 26rem); aspect-ratio: 1; }
  #board input { -moz-appearance: textfield; appearance: none; background: transparent; border: 1px solid rgba(128,128,128,.45); color: inherit; font: inherit; font-size: clamp(14px, 4.4vw, 22px); min-width: 0; padding: 0; text-align: center; width: 100%; height: 100%; }
  #board input:focus { outline: 2px solid #4a8fd6; outline-offset: -2px; }
  #board input.given { font-weight: 700; background: rgba(128,128,128,.16); }
  #board input.bad { background: #d1362f; color: #fff; }
  #board input.edge-r { border-right-width: 2px; border-right-color: currentColor; }
  #board input.edge-b { border-bottom-width: 2px; border-bottom-color: currentColor; }
  #bar { align-items: center; display: flex; gap: .75rem; margin-top: .75rem; }
  #bar button { font: inherit; padding: .3rem .7rem; }
  #status { font-weight: 600; }
</style>
<script>
  const N = 9;
  const peersOf = (i) => { const r = (i / N) | 0, c = i % N, br = r - (r % 3), bc = c - (c % 3), out = new Set(); for (let k = 0; k < N; k++) { out.add(r * N + k); out.add(k * N + c); } for (let dr = 0; dr < 3; dr++) for (let dc = 0; dc < 3; dc++) out.add((br + dr) * N + bc + dc); out.delete(i); return [...out]; };
  const PEERS = Array.from({ length: 81 }, (_, i) => peersOf(i));
  const shuffled = (a) => { for (let i = a.length - 1; i > 0; i--) { const j = (Math.random() * (i + 1)) | 0; [a[i], a[j]] = [a[j], a[i]]; } return a; };
  const fits = (g, i, v) => !PEERS[i].some((p) => g[p] === v);
  // Randomised backtracking, which is enough to produce a full grid quickly.
  function fill(g, i = 0) { if (i === 81) return true; if (g[i]) return fill(g, i + 1); for (const v of shuffled([1,2,3,4,5,6,7,8,9])) { if (!fits(g, i, v)) continue; g[i] = v; if (fill(g, i + 1)) return true; g[i] = 0; } g[i] = 0; return false; }
  // Counts up to `cap` solutions, so uniqueness costs no more than finding two.
  function count(g, cap = 2, i = 0) { if (i === 81) return 1; if (g[i]) return count(g, cap, i + 1); let total = 0; for (let v = 1; v <= 9 && total < cap; v++) { if (!fits(g, i, v)) continue; g[i] = v; total += count(g, cap - total, i + 1); g[i] = 0; } return total; }
  // Dig holes, keeping only the removals that leave the puzzle with one answer.
  function puzzle(givens = 32) { const g = new Array(81).fill(0); fill(g); const p = g.slice(); let left = 81; for (const i of shuffled([...Array(81).keys()])) { if (left <= givens) break; const held = p[i]; p[i] = 0; if (count(p.slice()) === 1) left--; else p[i] = held; } return p; }
  const board = document.getElementById("board");
  const status = document.getElementById("status");
  const cells = [];
  for (let i = 0; i < 81; i++) { const input = document.createElement("input"); input.type = "text"; input.inputMode = "numeric"; input.maxLength = 1; input.setAttribute("aria-label", "row " + (((i / N) | 0) + 1) + " column " + ((i % N) + 1)); if ((i % N) % 3 === 2 && i % N !== 8) input.classList.add("edge-r"); if ((((i / N) | 0) % 3) === 2 && ((i / N) | 0) !== 8) input.classList.add("edge-b"); board.appendChild(input); cells.push(input); }
  const valueAt = (i) => (cells[i].value.trim() === "" ? 0 : Number(cells[i].value));
  // A digit is wrong when it fights a peer, never because it differs from some
  // stored answer — that would be a lie on any puzzle with more than one.
  function review() { const values = cells.map((_, i) => valueAt(i)); let bad = 0, filled = 0; cells.forEach((cell, i) => { const v = values[i]; if (v) filled++; const clash = v !== 0 && PEERS[i].some((p) => values[p] === v); cell.classList.toggle("bad", clash); if (clash) bad++; }); status.textContent = bad ? bad + " square" + (bad === 1 ? "" : "s") + " in conflict" : filled === 81 ? "solved" : filled + " of 81"; }
  function deal() { const p = puzzle(); cells.forEach((cell, i) => { const given = p[i] !== 0; cell.value = given ? String(p[i]) : ""; cell.readOnly = given; cell.classList.toggle("given", given); cell.classList.remove("bad"); }); review(); }
  board.addEventListener("input", (event) => { const cell = event.target; cell.value = /^[1-9]$/.test(cell.value) ? cell.value : ""; review(); });
  board.addEventListener("keydown", (event) => { const i = cells.indexOf(event.target); if (i < 0) return; const step = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: -N, ArrowDown: N }[event.key]; if (!step) return; const next = i + step; if (next >= 0 && next < 81) { cells[next].focus(); event.preventDefault(); } });
  document.getElementById("again").addEventListener("click", deal);
  deal();
</script>
