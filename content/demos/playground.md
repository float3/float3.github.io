+++
title = "playground"
date = 2024-01-27
updated = 2024-01-27
+++
<noscript> hey this page needs javascript</noscript>
<script src="https://cdnjs.cloudflare.com/ajax/libs/tone/14.8.49/Tone.js" integrity="sha512-jduERlz7En1IUZR54bqzpNI64AbffZWR//KJgF71SJ8D8/liKFZ+s1RxmUmB+bhCnIfzebdZsULwOrbVB5f3nQ==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
<script src="https://requirejs.org/docs/release/2.3.6/minified/require.js" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
<script src="js/playground.js"></script>
use your computer keyboard or a midi device

<label for="tuningSelect">Select Tuning System:</label>
<select id="tuningSelect" name="tuningSelect" onchange="tuningSelectOnChange()">
  <option value="JustIntonation">Just Intonation</option>
  <option value="JustIntonation24">Just Intonation 24</option>
  <option value="StepMethod">Just Intonated Step Method</option>
  <option value="EqualTemperament">Equal Temperament</option>
  <option value="PythagoreanTuning">Pythagorean Tuning</option>
  <option value="FiveLimit">Five Limit</option>
  <option value="ElevenLimit">Eleven Limit</option>
  <option value="FortythreeTone">Fortythree tone tuning</option>
  <option value="Indian">Indian</option>
  <option value="IndianFull">Indian Full</option>
  <!--option value="meantone_temperament">Meantone Temperament</option>
  <option value="well_temperament">Well Temperament</option>
  <option value="equal_temperament">Equal Temperament</option-->
</select>

<label for="soundMethod">Select Sound Method:</label>
<select id="soundMethod" name="soundMethod">
  <option value="native">Native</option>
  <option value="tone.js">tone.js</option>
</select>


Volume: <input type="range" id="volumeSlider" min="0" max="1" step="0.01" value="0.5">

Base Freq: <input id="baseFreq" value="220">

<div id="stepSizeContainer" style="display: block;">
    <label for="stepSize">Step Size (co-primes with 12):</label>
    <select id="stepSize">
        <option value="1">1</option>
        <option value="5">5</option>
        <option value="7" selected>7</option>
        <option value="11">11</option>
    </select>
</div>

<div id="equalTemperamentBaseContainer" style="display: block;">
    <label for="equalTemperamentBase">Equal Temperament Base:</label>
    <input id="equalTemperamentBase" value="12">
</div>

<div id="logContainer"></div>
