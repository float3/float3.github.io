+++
title = "Recursive Just-Intonation"
date = 2022-11-29
updated = 2023-06-15
+++

# THIS IS A WIP

░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█
░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█
░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░

# Recursive Just-Intonation: An unusable Tuning System

Disclaimer: the only music related qualification that I posses is "Junior Assistant Conductor"

# Visualize and listen to Frequencies in a Shader:
<iframe width="640" height="360" frameborder="0" allowfullscreen="allowfullscreen" src="https://www.shadertoy.com/embed/7tV3WV?gui=true&t=10&paused=false&muted=false">
</iframe> 

# 12TET: 12 Tone Equal Temperament
<!--https://github.com/ronkok/Temml -->
<!--P_n = P_a\big(\sqrt[12]{2}\big)^{(n-a)} -->
In 12TET a frequency is defined as
<math display="block" style="display:inline-block;">
  <mrow>
    <msub>
      <mi>P</mi>
      <mi>n</mi>
    </msub>
    <mo>=</mo>
    <msub>
      <mi>P</mi>
      <mi>a</mi>
    </msub>
    <mo fence="false" symmetric="true" minsize="1.2em">(</mo>
    <mroot>
      <mn>2</mn>
      <mn>12</mn>
    </mroot>
    <msup>
      <mo fence="false" symmetric="true" minsize="1.2em">)</mo>
      <mrow>
        <mo form="prefix" stretchy="false">(</mo>
        <mi>n</mi>
        <mo>−</mo>
        <mi>a</mi>
        <mo form="postfix" stretchy="false">)</mo>
      </mrow>
    </msup>
  </mrow>
</math>
# Why did I decide to make a [new conflicting standard](https://xkcd.com/927/)

I was frustrated with 12 TET and watched a video on Just Intonation,
I immediately realized the impracticality of it so I decided to make my own even less practical version.

# What makes one interval nice and another unpleasant

Nice mathematical ratios are pleasant to our ears.  
x+2*x where x is some frequency is gonna sound nice, because it has a short period,

<iframe width="1000" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+t)+sin(2*(x+t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
</iframe>

while for example x+13/12x has a much longer period 

<iframe width="1000" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+t)+sin((13/12)*(x+t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
</iframe>

# Why Just Intonation is good

Waves that are nice to look at are nice to the Ear.
Just Intonation is nice because intervals have nice mathematical ratios. For Example, a major chord is 4:5:6 (1:1.25:1.5).
While in 12TET the perfect fifth is roughly 500:630:749 (1:1.260:1.498) 
the following graph shows the difference between 2 sine waves 

<iframe width="1000" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+t)+sin(1.25*(x+t))+sin(1.5*(x+t))&v1=true&f2(x,t)=sin(x+t)+sin(1.26*(x+t))+sin(1.4983*(x+t))&v2=true&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=true&grid=1&coords=0,-3,12">
</iframe>

# Why Just Intonation is bad


1\*1.0625\*1.0625 != 1\*1.125

Now while nicer intervals are nicer all of these intervals are in relation to X, our Root
While a major third (4:5) and a perfect fifth (2:3) on their own sound good, 
if we keep going up the steps one by one (1.0625), we don't end up at the same place that we would end up if we skipped a step (1.125)










```
Just Intonation
1 1.0625 1.125 1.1875 1.25 1.34375 1.40625 1.5 1.59375 1.6875 1.78125 1.875 2
C        D            E    F               G           A              B     C

12 TET: the octave is split onto 12 equal parts:
1.000 1.059 1.122 1.189 1.260 1.335 1.414 1.498 1.587 1.682 1.782 1.888 2.000
C           D           E     F           G           A           B     C

Harmonic scale. next = prev * 1.5 and optionally /2
Only in harmonic scale frequencies relate as integer numbers
1.0000 1.5000 1.1250 1.6875 1.2656 1.8984 1.4238 1.0679
C      G      D      A      E      B      F      C
```