---
title: bayes theorem
date: 2024-11-29
tags:
  - math
  - statistics
  - teaching
---

<link href="./math_stuff.css" rel="stylesheet" type="text/css">
<div class="container">

$$
\begin{equation}
\textcolor{#0466e7}{P(A \mid B)} =
\frac{\textcolor{#ffbc3f}{P(B \mid A)} \,
\textcolor{#fe7fb3}{P(A)}}{\textcolor{#dd6fff}{P(B)}}
\end{equation}
$$

</div>

To find
<span style="color:#0466e7">the probability of event \(A\) occurring given event \(B\) has occurred</span>, we multiply
<span style="color:#ffbc3f">the likelihood of event \(B\) given event \(A\)</span> by
<span style="color:#fe7fb3">the prior probability of event \(A\)</span> and divide by
<span style="color:#dd6fff">the marginal probability of event \(B\)</span><sup><span style="color:#4d8dc3">1</span></sup>.
<br></br>
Note that
<span style="color:#dd6fff">the marginal probability \(P(B)\)</span> acts as a normalization constant, ensuring that the posterior probabilities sum to 1, and it is computed as the sum of probabilities across all possible causes of \(B\).

<span style="color:#4d8dc3">
1. <a href="https://en.wikipedia.org/wiki/Bayes%27_theorem">Bayes' theorem</a> <br>
</span>
