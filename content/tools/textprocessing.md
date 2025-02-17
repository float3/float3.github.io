---
title: textprocessing
date: 2024-12-09
updated: 2025-02-17
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
        <input id="left0" oninput="transformLeftToRight(0)" value="wèi shén me">
      </td>
      <td>
        Zhuyin 
        <button onclick="copyToClipboard('right0')">Copy</button>
        <input id="right0" oninput="transformRightToLeft(0)" value="ㄨㄟˋ ㄕㄣˊ ㄇㄜ˙">
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
        <input id="left9" oninput="transformLeftToRight(9)" value="行">
      </td>
      <td>
        Zhuyin all readings
        <button onclick="copyToClipboard('right9')">Copy</button>
        <input id="right9" oninput="transformRightToLeft(9)" value="ㄒㄧㄥˊ ㄏㄤˊ ㄏㄥˊ ㄒㄧㄥˋ ㄏㄤˋ" disabled>
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
      <td>
        Hangeul
        <button onclick="copyToClipboard('left19')">Copy</button>
        <input id="left19" oninput="transformLeftToRight(19)" value="재원한국">
      </td>
      <td>
        Revised Romanization
        <button onclick="copyToClipboard('right3')">Copy</button>
        <input id="right19" oninput="transformRightToLeft(19)" value="jaewonhanguk">
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
        <input id="left7" oninput="transformLeftToRight(7)" value="3339">
      </td>
      <td>
        Roman
        <button onclick="copyToClipboard('right7')">Copy</button>
        <input id="right7" oninput="transformRightToLeft(7)" value="MMMCCCXXXIX">
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left18')">Copy</button>
        <input id="left18" oninput="transformLeftToRight(18)" value="1234567890">
      </td>
      <td>
        Japanese
        <button onclick="copyToClipboard('right18')">Copy</button>
        <input id="right18" oninput="transformRightToLeft(18)" value="十ニ億三千四百五十六万七千八百九十" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left10')">Copy</button>
        <input id="left10" oninput="transformLeftToRight(10)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right10')">Copy</button>
        <input id="right10" oninput="transformRightToLeft(10)" value="壹秭貳垓參京肆兆伍億陸萬柒仟捌佰玖拾" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left11')">Copy</button>
        <input id="left11" oninput="transformLeftToRight(11)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right11')">Copy</button>
        <input id="right11" oninput="transformRightToLeft(11)" value="拾貳億參仟肆佰伍拾陸萬柒仟捌佰玖拾" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left12')">Copy</button>
        <input id="left12" oninput="transformLeftToRight(12)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right12')">Copy</button>
        <input id="right12" oninput="transformRightToLeft(12)" value="拾貳億參仟肆佰伍拾陸萬柒仟捌佰玖拾" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left13')">Copy</button>
        <input id="left13" oninput="transformLeftToRight(13)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right13')">Copy</button>
        <input id="right13" oninput="transformRightToLeft(13)" value="拾貳億參仟肆佰伍拾陸萬柒仟捌佰玖拾" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left14')">Copy</button>
        <input id="left14" oninput="transformLeftToRight(14)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right14')">Copy</button>
        <input id="right14" oninput="transformRightToLeft(14)" value="一秭二垓三京四兆五億六萬七千八百九十" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left15')">Copy</button>
        <input id="left15" oninput="transformLeftToRight(15)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right15')">Copy</button>
        <input id="right15" oninput="transformRightToLeft(15)" value="十二億三千四百五十六萬七千八百九十" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left16')">Copy</button>
        <input id="left16" oninput="transformLeftToRight(16)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right16')">Copy</button>
        <input id="right16" oninput="transformRightToLeft(16)" value="十二億三千四百五十六萬七千八百九十" disabled>
      </td>
    </tr>
    <tr>
      <td>
        Arabic
        <button onclick="copyToClipboard('left17')">Copy</button>
        <input id="left17" oninput="transformLeftToRight(17)" value="1234567890">
      </td>
      <td>
        Chinese
        <button onclick="copyToClipboard('right17')">Copy</button>
        <input id="right17" oninput="transformRightToLeft(17)" value="十二億三千四百五十六萬七千八百九十" disabled>
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
