+++
title = "test"
updated = 1970-01-01
+++
<html lang="en">
<body>
  <script src="/sound.js"></script>
  <script src="https://unpkg.com/tone"></script>
</body>
</html>

<label for="tuningSelect">Select Tuning System:</label>
<select id="tuningSelect" name="tuningSelect">
  <option value="twelve_tone">12TET</option>
  <option value="just_intonation">Just Intonation</option>
</select>

<label for="instrumentSelect">Select Sound Library:</label>
<select id="instrumentSelect" name="instrumentSelect">
  <option value="tone.js">tone.js</option>
  <option value="audioContext">Audio Context</option>
</select>
