---
title: tuningplayground
date: 2024-11-26
updated: 2025-02-08
tags:
  - music
  - wasm
  - tools
  - rust
---

<link href="./tuningplayground.css" rel="stylesheet" type="text/css">
<noscript> hey this page needs javascript</noscript> use your computer keyboard, a midi device, or provide a midi file for example <a href="https://www.midiworld.com/midis/other/mozart/jm_mozdi.mid" download="mozart_dies_irea.mid"> this one </a> or <a href="/misc/blobs/jm_mozdi.mid" download="mozart_dies_irea.mid"> or this one in case the other site goes down </a>
<div style="display: block">
  <input type="file" id="fileInput" accept=".midi,.mid" />
  <button id="playButton">Play</button>
  <button id="stopButton">Stop</button>
</div>
<div style="display: block">
<p>
  <label for="tuningSelect">Select Tuning System:</label>
  <select id="tuningSelect" name="tuningSelect">
    <option value="JustIntonation">Just Intonation</option>
    <option value="JustIntonation24">Just Intonation 24</option>
    <option value="StepMethod">Just Intonated Step Method</option>
    <option value="EqualTemperament">Equal Temperament</option>
    <option value="WholeTone">WholeTone</option>
    <option value="QuarterTone">QuarterTone</option>
    <option value="PythagoreanTuning">Pythagorean Tuning</option>
    <option value="FiveLimit">Five Limit</option>
    <option value="ElevenLimit">Eleven Limit</option>
    <option value="FortythreeTone">Fortythree tone tuning</option>
    <option value="Indian">Indian</option>
    <option value="IndianAlt">Indian Alt</option>
    <option value="IndianFull">Indian Full</option>
    <option value="equal_temperament">Equal Temperament</option>
  </select>
</p>
<p>
  <label for="soundMethod">Select Sound Method:</label>
  <select id="soundMethod" name="soundMethod">
    <option value="sample">Sample</option>
    <option value="native">Native</option>
  </select>
</p>
<p> Volume: <input type="range" id="volumeSlider" min="0" max="1" step="0.01" value="0.25" />
</p>
<p>Transpose: <input id="transpose" />
</p>
<div id="stepSizeContainer" style="display: none">
  <label for="stepSize">Step Size (co-primes with 12):</label>
  <select id="stepSize">
    <option value="1">1</option>
    <option value="5">5</option>
    <option value="7" selected>7</option>
    <option value="11">11</option>
  </select>
</div>
<div id="octaveSize_container" style="display: block">
  <label for="octaveSize">Octave Size:</label>
  <input id="octaveSize" value="12" />
</div>
<div id="markedButtons" style="display: none">
  <button id="playMarked">Play Marked Notes</button>
  <button id="shareMarked">Share Marked Notes</button>
</div>
<div id="output" style="background-color: #00000000; color: white"></div>
<div class="keyboard dark-mode-invert">
  <div class="octave">
    <div class="white-key" data-note="21"></div>
    <div class="black-key" data-note="22"></div>
    <div class="white-key" data-note="23"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="24"></div>
    <div class="black-key" data-note="25"></div>
    <div class="white-key" data-note="26"></div>
    <div class="black-key" data-note="27"></div>
    <div class="white-key" data-note="28"></div>
    <div class="white-key" data-note="29"></div>
    <div class="black-key" data-note="30"></div>
    <div class="white-key" data-note="31"></div>
    <div class="black-key" data-note="32"></div>
    <div class="white-key" data-note="33"></div>
    <div class="black-key" data-note="34"></div>
    <div class="white-key" data-note="35"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="36"></div>
    <div class="black-key" data-note="37"></div>
    <div class="white-key" data-note="38"></div>
    <div class="black-key" data-note="39"></div>
    <div class="white-key" data-note="40"></div>
    <div class="white-key" data-note="41"></div>
    <div class="black-key" data-note="42"></div>
    <div class="white-key" data-note="43"></div>
    <div class="black-key" data-note="44"></div>
    <div class="white-key" data-note="45"></div>
    <div class="black-key" data-note="46"></div>
    <div class="white-key" data-note="47"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="48"></div>
    <div class="black-key" data-note="49"></div>
    <div class="white-key" data-note="50"></div>
    <div class="black-key" data-note="51"></div>
    <div class="white-key" data-note="52"></div>
    <div class="white-key" data-note="53"></div>
    <div class="black-key" data-note="54"></div>
    <div class="white-key" data-note="55"></div>
    <div class="black-key" data-note="56"></div>
    <div class="white-key" data-note="57"></div>
    <div class="black-key" data-note="58"></div>
    <div class="white-key" data-note="59"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="60"></div>
    <div class="black-key" data-note="61"></div>
    <div class="white-key" data-note="62"></div>
    <div class="black-key" data-note="63"></div>
    <div class="white-key" data-note="64"></div>
    <div class="white-key" data-note="65"></div>
    <div class="black-key" data-note="66"></div>
    <div class="white-key" data-note="67"></div>
    <div class="black-key" data-note="68"></div>
    <div class="white-key" data-note="69"></div>
    <div class="black-key" data-note="70"></div>
    <div class="white-key" data-note="71"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="72"></div>
    <div class="black-key" data-note="73"></div>
    <div class="white-key" data-note="74"></div>
    <div class="black-key" data-note="75"></div>
    <div class="white-key" data-note="76"></div>
    <div class="white-key" data-note="77"></div>
    <div class="black-key" data-note="78"></div>
    <div class="white-key" data-note="79"></div>
    <div class="black-key" data-note="80"></div>
    <div class="white-key" data-note="81"></div>
    <div class="black-key" data-note="82"></div>
    <div class="white-key" data-note="83"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="84"></div>
    <div class="black-key" data-note="85"></div>
    <div class="white-key" data-note="86"></div>
    <div class="black-key" data-note="87"></div>
    <div class="white-key" data-note="88"></div>
    <div class="white-key" data-note="89"></div>
    <div class="black-key" data-note="90"></div>
    <div class="white-key" data-note="91"></div>
    <div class="black-key" data-note="92"></div>
    <div class="white-key" data-note="93"></div>
    <div class="black-key" data-note="94"></div>
    <div class="white-key" data-note="95"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="96"></div>
    <div class="black-key" data-note="97"></div>
    <div class="white-key" data-note="98"></div>
    <div class="black-key" data-note="99"></div>
    <div class="white-key" data-note="100"></div>
    <div class="white-key" data-note="101"></div>
    <div class="black-key" data-note="102"></div>
    <div class="white-key" data-note="103"></div>
    <div class="black-key" data-note="104"></div>
    <div class="white-key" data-note="105"></div>
    <div class="black-key" data-note="106"></div>
    <div class="white-key" data-note="107"></div>
  </div>
  <div class="octave">
    <div class="white-key" data-note="108"></div>
  </div>
</div>
<div id="logContainer"></div>
</div>
<script src="./wasm/tuningplayground.js"></script>
</div>
