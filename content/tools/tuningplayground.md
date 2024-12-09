---
title: tuningplayground
date: 2024-01-10
updated: 2024-11-25
tags:
  - music
  - wasm
---

<link href="./tuningplayground.css" rel="stylesheet" type="text/css">
<noscript> hey this page needs javascript</noscript>
use your computer keyboard, a midi device, or provide a midi file for
example
<a
   href="https://www.midiworld.com/midis/other/mozart/jm_mozdi.mid"
   download="mozart_dies_irea.mid">
this one
</a>
or 
<a
   href="/misc/blobs/jm_mozdi.mid"
   download="mozart_dies_irea.mid">
or this one in case the other site goes down
</a>
<br>
theres also the debug version of this page: <a href="/tools/tuningplayground_debug.md">here</a>
<div style="display: block">
   <input type="file" id="fileinput" accept=".midi,.mid" />
   <!-- <input type="text" id="linkinput" value="https://www.midiworld.com/midis/other/mozart/jm_mozdi.mid" placeholder="enter midi file link"> -->
   <button id="playbutton">play</button>
   <button id="stopbutton">stop</button>
</div>
<p>
   <label for="tuningselect">select tuning system:</label>
   <select id="tuningselect" name="tuningselect">
      <option value="justintonation">just intonation</option>
      <option value="justintonation24">just intonation 24</option>
      <option value="stepmethod">just intonated step method</option>
      <option value="equaltemperament">equal temperament</option>
      <!-- <option value="thai">thai</option>
         <option value="javanese">javanese</option> -->
      <option value="wholetone">wholetone</option>
      <option value="quartertone">quartertone</option>
      <option value="pythagoreantuning">pythagorean tuning</option>
      <option value="fivelimit">five limit</option>
      <option value="elevenlimit">eleven limit</option>
      <option value="fortythreetone">fortythree tone tuning</option>
      <option value="indian">indian</option>
      <option value="indianalt">indian alt</option>
      <option value="indianfull">indian full</option>
      <!-- <option value="meantone_temperament">meantone temperament</option>
         <option value="well_temperament">well temperament</option> -->
      <option value="equal_temperament">equal temperament</option>
   </select>
</p>
<p>
   <label for="soundmethod">select sound method:</label>
   <select id="soundmethod" name="soundmethod">
      <option value="sample">sample</option>
      <option value="native">native</option>
      <!-- <option value="tone.js">tone.js</option> -->
   </select>
</p>
<p>
   volume:
   <input
      type="range"
      id="volumeslider"
      min="0"
      max="1"
      step="0.01"
      value="0.25"
      />
</p>
<p>transpose: <input id="transpose" /></p>
<div id="stepsizecontainer" style="display: none">
   <label for="stepsize">step size (co-primes with 12):</label>
   <select id="stepsize">
      <option value="1">1</option>
      <option value="5">5</option>
      <option value="7" selected>7</option>
      <option value="11">11</option>
   </select>
</div>
<div id="octavesize_container" style="display: block">
   <label for="octavesize">octave size:</label>
   <input id="octavesize" value="12" />
</div>
<div id="markedbuttons" style="display: none">
   <button id="playmarked">play marked notes</button>
   <button id="sharemarked">share marked notes</button>
</div>
<div id="output" style="background-color: white; color: black"></div>
<div class="keyboard dark-mode-invert">
   <!-- <div class="octave">
      <div class="white-key" data-note="0"></div>
      <div class="black-key" data-note="1"></div>
      <div class="white-key" data-note="2"></div>
      <div class="black-key" data-note="3"></div>
      <div class="white-key" data-note="4"></div>
      <div class="white-key" data-note="5"></div>
      <div class="black-key" data-note="6"></div>
      <div class="white-key" data-note="7"></div>
      <div class="black-key" data-note="8"></div>
      <div class="white-key" data-note="9"></div>
      <div class="black-key" data-note="10"></div>
      <div class="white-key" data-note="11"></div>
      </div> -->
   <div class="octave">
      <!-- <div class="white-key" data-note="12"></div>
         <div class="black-key" data-note="13"></div>
         <div class="white-key" data-note="14"></div>
         <div class="black-key" data-note="15"></div>
         <div class="white-key" data-note="16"></div>
         <div class="white-key" data-note="17"></div>
         <div class="black-key" data-note="18"></div>
         <div class="white-key" data-note="19"></div>
         <div class="black-key" data-note="20"></div> -->
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
<div id="logcontainer"></div>
</div>
<script src="./tuningplayground/bootstrap.js"></script>
