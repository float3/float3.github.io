---
title: von Neumann–Morgenstern axioms
tags:
  - math
  - economics
  - statistics
  - teaching
---

### completeness

$$
\begin{equation}
\textcolor{#7be9ff}{\forall}\,
\textcolor{#0466e7}{A},
\textcolor{#ffbc3f}{B}:\;
\textcolor{#0466e7}{A}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#ffbc3f}{B}
\;\lor\;
\textcolor{#ffbc3f}{B}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#0466e7}{A}
\end{equation}
$$

for <span style="color:#7be9ff">any</span> two lotteries <span style="color:#0466e7">A</span> and <span style="color:#ffbc3f">B</span>, either <span style="color:#0466e7">A</span> is <span style="color:#fe7fb3">at least as good as</span> <span style="color:#ffbc3f">B</span>, or the other way around (or both). the agent always has an opinion.

### transitivity

$$
\begin{equation}
\textcolor{#0466e7}{A}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#ffbc3f}{B},\;
\textcolor{#ffbc3f}{B}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#dd6fff}{C}
\;\Rightarrow\;
\textcolor{#0466e7}{A}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#dd6fff}{C}
\end{equation}
$$

if <span style="color:#0466e7">A</span> <span style="color:#fe7fb3">beats</span> <span style="color:#ffbc3f">B</span> and <span style="color:#ffbc3f">B</span> <span style="color:#fe7fb3">beats</span> <span style="color:#dd6fff">C</span>, then <span style="color:#0466e7">A</span> <span style="color:#fe7fb3">beats</span> <span style="color:#dd6fff">C</span>. without it, an agent with cyclic preferences can be money-pumped: it pays to swap <span style="color:#dd6fff">C</span>→<span style="color:#ffbc3f">B</span>→<span style="color:#0466e7">A</span>→<span style="color:#dd6fff">C</span> forever.

### continuity

$$
\begin{equation}
\textcolor{#0466e7}{A}
\textcolor{#fe7fb3}{\succ}
\textcolor{#ffbc3f}{B}
\textcolor{#fe7fb3}{\succ}
\textcolor{#dd6fff}{C}
\;\Rightarrow\;
\textcolor{#7be9ff}{\exists}\,
\textcolor{#9dc141}{p} \in [0,1]:\;
\textcolor{#9dc141}{p}\textcolor{#0466e7}{A}
+
(1-\textcolor{#9dc141}{p})\textcolor{#dd6fff}{C}
\textcolor{#fe7fb3}{\sim}
\textcolor{#ffbc3f}{B}
\end{equation}
$$

if <span style="color:#ffbc3f">B</span> sits <span style="color:#fe7fb3">strictly</span> between <span style="color:#0466e7">A</span> and <span style="color:#dd6fff">C</span>, then <span style="color:#7be9ff">there is</span> <span style="color:#9dc141">a probability</span> at which a gamble between <span style="color:#0466e7">the best</span> and <span style="color:#dd6fff">the worst</span> feels <span style="color:#fe7fb3">exactly as good as</span> <span style="color:#ffbc3f">the middle</span> for sure. no outcome is infinitely good or infinitely bad.

### independence

$$
\begin{equation}
\textcolor{#0466e7}{A}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#ffbc3f}{B}
\;\Rightarrow\;
\textcolor{#9dc141}{p}\textcolor{#0466e7}{A}
+
(1-\textcolor{#9dc141}{p})\textcolor{#dd6fff}{C}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#9dc141}{p}\textcolor{#ffbc3f}{B}
+
(1-\textcolor{#9dc141}{p})\textcolor{#dd6fff}{C}
\end{equation}
$$

mixing <span style="color:#dd6fff">a third option</span> into both sides with <span style="color:#9dc141">the same probability</span> does not flip the preference. <span style="color:#dd6fff">C</span> is irrelevant; only the part where the lotteries differ matters. this is the axiom humans most often violate (see the Allais paradox<sup><span style="color:#4d8dc3">1</span></sup>).

### the theorem

if preferences satisfy all four, there is a utility function $\textcolor{#0466e7}{u}$ such that

$$
\begin{equation}
\textcolor{#0466e7}{A}
\textcolor{#fe7fb3}{\succeq}
\textcolor{#ffbc3f}{B}
\iff
\mathbb{E}[\textcolor{#0466e7}{u}(\textcolor{#0466e7}{A})]
\ge
\mathbb{E}[\textcolor{#0466e7}{u}(\textcolor{#ffbc3f}{B})]
\end{equation}
$$

a rational agent acts as if it maximizes expected utility. $\textcolor{#0466e7}{u}$ is unique up to positive affine transformation $a\,\textcolor{#0466e7}{u} + b,\; a > 0$.<sup><span style="color:#4d8dc3">2</span></sup>

### legend

- $\textcolor{#0466e7}{A},\textcolor{#ffbc3f}{B},\textcolor{#dd6fff}{C}$ - lotteries (probability distributions over outcomes).
- $\textcolor{#fe7fb3}{\succeq}$ - weakly preferred (at least as good as).
- $\textcolor{#fe7fb3}{\succ}$ - strictly preferred.
- $\textcolor{#fe7fb3}{\sim}$ - indifferent.
- $\textcolor{#9dc141}{p}$ - probability, $\textcolor{#9dc141}{p} \in [0,1]$.
- $\textcolor{#9dc141}{p}\textcolor{#0466e7}{A} + (1-\textcolor{#9dc141}{p})\textcolor{#dd6fff}{C}$ - compound lottery: $\textcolor{#0466e7}{A}$ with probability $\textcolor{#9dc141}{p}$, else $\textcolor{#dd6fff}{C}$.
- $\textcolor{#7be9ff}{\forall}$, $\textcolor{#7be9ff}{\exists}$ - for all, there exists.

<span style="color:#4d8dc3">
1. <a href="https://en.wikipedia.org/wiki/Allais_paradox">Allais paradox</a> <br>
2. <a href="https://en.wikipedia.org/wiki/Von_Neumann%E2%80%93Morgenstern_utility_theorem">Von Neumann–Morgenstern utility theorem</a>
</span>
