---
title: textprocessing
date: 2024-12-09
tags:
  - text
  - language
  - wasm
  - tools
  - rust
---

made with love and rust (compiled to wasm)

<script src="./textprocessing/bootstrap.js"></script>
<link href="./textprocessing.css" rel="stylesheet" type="text/css">
<table>
  <tbody>
    <tr>
      <td colspan="2">
        Chinese
      </td>
    </tr>
    <tr>
      <td>
        Pinyin 
        <button onclick="copyToClipboard('left0')">Copy</button>
        <input id="left0" oninput="transformLeftToRight(0)" value="zhùang">
      </td>
      <td>
        Zhuyin 
        <button onclick="copyToClipboard('right0')">Copy</button>
        <input id="right0" oninput="transformRightToLeft(0)" value="ㄓㄨㄤˋ">
      </td>
    </tr>
    <tr>
      <td>
        Traditional 繁體字
        <button onclick="copyToClipboard('left1')">Copy</button>
        <input id="left1" oninput="transformLeftToRight(1)" value="為什麼">
      </td>
      <td>
        Simplified 簡體字
        <button onclick="copyToClipboard('right1')">Copy</button>
        <input id="right1" oninput="transformRightToLeft(1)" value="为什么">
      </td>
    </tr>
    <tr>
      <td>
        Hanzi
        <button onclick="copyToClipboard('left4')">Copy</button>
        <input id="left4" oninput="transformLeftToRight(4)" value="漢字">
      </td>
      <td>
        pīnyīn
        <button onclick="copyToClipboard('right4')">Copy</button>
        <input id="right4" oninput="transformRightToLeft(4)" value="hàn zì" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Hanzi
        <button onclick="copyToClipboard('left8')">Copy</button>
        <input id="left8" oninput="transformLeftToRight(8)" value="漢字">
      </td>
      <td>
        Zhuyin
        <button onclick="copyToClipboard('right8')">Copy</button>
        <input id="right8" oninput="transformRightToLeft(8)" value="ㄏㄢˋ ㄗˋ" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Hanzi
        <button onclick="copyToClipboard('left9')">Copy</button>
        <input id="left9" oninput="transformLeftToRight(9)" value="漢字">
      </td>
      <td>
        Zhuyin all readings
        <button onclick="copyToClipboard('right9')">Copy</button>
        <input id="right9" oninput="transformRightToLeft(9)" value="ㄏㄢˋ ㄗˋ" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Hanzi
        <button onclick="copyToClipboard('left5')">Copy</button>
        <input id="left5" oninput="transformLeftToRight(5)" value="行">
      </td>
      <td>
        pīnyīn all readings
        <button onclick="copyToClipboard('right5')">Copy</button>
        <input id="right5" oninput="transformRightToLeft(5)" value="xíng háng héng xìng hàng" disabled>
      </td>
    </tr> 
    <tr>
      <td colspan="2">
        Japanese
      </td>
    </tr>
    <tr>
      <td>
        Hiragana 
        <button onclick="copyToClipboard('left2')">Copy</button>
        <input id="left2" oninput="transformLeftToRight(2)" value="ひらがな">
      </td>
      <td>
        Katakana 
        <button onclick="copyToClipboard('right2')">Copy</button>
        <input id="right2" oninput="transformRightToLeft(2)" value="ヒラガナ">
      </td>
    </tr>
    <tr>
      <td colspan="2">
        Korean
      </td>
    </tr>
    <tr>
      <td colspan="2" style="font-size: smaller;">
        Hangeul to Hanja, and Hanja to Hangeul conversion is not perfect as the first match is picked but the library returns a list of matches.
      </td>
    </tr>    
    <tr>
      <td>
        Hanja
        <button onclick="copyToClipboard('left3')">Copy</button>
        <input id="left3" oninput="transformLeftToRight(3)" value="在元韓國">
      </td>
      <td>
        Hangeul 
        <button onclick="copyToClipboard('right3')">Copy</button>
        <input id="right3" oninput="transformRightToLeft(3)" value="재원한국">
      </td>
    </tr>
    <tr>
      <td colespan="2">
        Numbers
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left7')">Copy</button>
        <input id="left7" oninput="transformLeftToRight(7)" value="1234567890">
      </td>
      <td>
        Roman
        <button onclick="copyToClipboard('right7')">Copy</button>
        <input id="right7" oninput="transformRightToLeft(7)" value="CXXIIIIVXVIX">
      </td>
    </tr>
    <!-- <tr>
      <td>
        Hanja
        <button onclick="copyToClipboard('left6')">Copy</button>
        <input id="left6" oninput="transformLeftToRight(6)" value="我愛你">
      </td>
      <td>
        Hangeul 
        <button onclick="copyToClipboard('right6')">Copy</button>
        <input id="right6" oninput="transformRightToLeft(6)" value="아애니">
      </td>
    </tr> -->
  </tbody>
</table>
