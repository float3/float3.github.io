---
title: textprocessing
date: 2024-12-09
tags:
  - text
  - languages
  - wasm
---

made with love and rust (compiled to wasm)

<script src="./textprocessing/bootstrap.js"></script>
<link href="./textprocessing.css" rel="stylesheet" type="text/css">
<table>
   <thead>
      <tr>
         <th>Left Input</th>
         <th>Right Input</th>
      </tr>
   </thead>
   <tbody>
      <tr>
         <td>Pinyin<input id="left0" oninput="transformLeftToRight(0)"></td>
         <td>Zhuyin<input id="right0" oninput="transformRightToLeft(0)"></td>
      </tr>
      <tr>
         <td>Traditional<input id="left1" oninput="transformLeftToRight(1)"></td>
         <td>Simplified<input id="right1" oninput="transformRightToLeft(1)"></td>
      </tr>
      <tr>
         <td>Hiragana<input id="left2" oninput="transformLeftToRight(2)"></td>
         <td>Katakana<input id="right2" oninput="transformRightToLeft(2)"></td>
      </tr>
      <tr>
         <td>Hanja<input id="left3" oninput="transformLeftToRight(3)"></td>
         <td>Hangeul<input id="right3" oninput="transformRightToLeft(3)"></td>
      </tr>
      <tr>
         <td><input id="left4" oninput="transformLeftToRight(4)"></td>
         <td><input id="right4" oninput="transformRightToLeft(4)"></td>
      </tr>
      <tr>
         <td><input id="left5" oninput="transformLeftToRight(5)"></td>
         <td><input id="right5" oninput="transformRightToLeft(5)"></td>
      </tr>
      <tr>
         <td><input id="left6" oninput="transformLeftToRight(6)"></td>
         <td><input id="right6" oninput="transformRightToLeft(6)"></td>
      </tr>
      <tr>
         <td><input id="left7" oninput="transformLeftToRight(7)"></td>
         <td><input id="right7" oninput="transformRightToLeft(7)"></td>
      </tr>
      <tr>
         <td><input id="left8" oninput="transformLeftToRight(8)"></td>
         <td><input id="right8" oninput="transformRightToLeft(8)"></td>
      </tr>
      <tr>
         <td><input id="left9" oninput="transformLeftToRight(9)"></td>
         <td><input id="right9" oninput="transformRightToLeft(9)"></td>
      </tr>
      <tr>
         <td><input id="left10" oninput="transformLeftToRight(10)"></td>
         <td><input id="right10" oninput="transformRightToLeft(10)"></td>
      </tr>
   </tbody>
</table>