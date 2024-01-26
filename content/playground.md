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

<!-- TODO number picker for equal temperament-->

<label for="tuningSelect">Select Tuning System:</label>
<select id="tuningSelect" name="tuningSelect">
  <option value="twelve_tone">12TET</option>
  <option value="twentyfour_tone">24TET</option>
  <option value="just_intonation">Just Intonation</option>
  <option value="pythagorean_tuning">Pythagorean Tuning</option>
  <option value="eleven_limit">Eleven Limit</option>
  <option value="fortythree_tone">Fortythree tone tuning</option>
  <!--option value="meantone_temperament">Meantone Temperament</option>
  <option value="well_temperament">Well Temperament</option>
  <option value="equal_temperament">Equal Temperament</option-->
</select>

<label for="instrumentSelect">Select Sound Library:</label>
<select id="instrumentSelect" name="instrumentSelect">
  <option value="tone.js">tone.js</option>
  <option value="audioContext">Audio Context</option>
</select>


Volume: <input type="range" id="volumeSlider" min="0" max="1" step="0.01" value="0.5">
