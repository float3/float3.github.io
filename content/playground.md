+++
title = "playground"
updated = 1970-01-01
+++
<html lang="en">
<body>
  <script src="/sound.js"></script>
  <script src="https://unpkg.com/tone"></script>
</body>
</html>

use your keyboard

<label for="tuningSelect">Select Tuning System:</label>
<select id="tuningSelect" name="tuningSelect" onchange="toggleInputVisibility()">
  <option value="equal_temperament">Equal Temperament</option>
  <option value="just_intonation">Just Intonation</option>
  <option value="pythagorean_tuning">Pythagorean Tuning</option>
  <option value="eleven_limit">Eleven Limit</option>
  <option value="fortythree_tone">Fortythree tone tuning</option>
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

<div id="equalTemperamentBaseContainer" style="display: block;">
    <label for="equalTemperamentBase">Equal Temperament Base:</label>
    <input id="equalTemperamentBase" value="12">
</div>

<div id="logContainer"></div>
