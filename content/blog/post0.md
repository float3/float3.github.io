+++
title = "Recursive Just-Intonation"
date = 2022-11-29
updated = 2023-06-15
+++

# THIS IS A WIP

░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█░▒█░░▒█░▀█▀░▒█▀▀█
░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█░▒█▒█▒█░▒█░░▒█▄▄█
░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░░▒▀▄▀▄▀░▄█▄░▒█░░░

# Recursive Just-Intonation: An unusable Tuning System or a Frustrating Journey through tuning

# 12TET: the current standard
<!--https://github.com/ronkok/Temml -->
In 12TET the ratio P between two tones is defined as <!--P_n = P_a\big(\sqrt[12]{2}\big)^{(n-a)}-->
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block" class="tml-display" style="display:inline-block;"><semantics><mrow><msub><mi>P</mi><mi>n</mi></msub><mo>=</mo><msub><mi>P</mi><mi>a</mi></msub><mo fence="false" symmetric="true" minsize="1.2em" maxsize="1.2em">(</mo><mroot><mn>2</mn><mn>12</mn></mroot><msup><mo fence="false" symmetric="true" minsize="1.2em" maxsize="1.2em">)</mo><mrow><mo form="prefix" stretchy="false" lspace="0em" rspace="0em">(</mo><mi>n</mi><mo>−</mo><mi>a</mi><mo form="postfix" stretchy="false" lspace="0em" rspace="0em">)</mo></mrow></msup></mrow><annotation encoding="application/x-tex">P_n = P_a\big(\sqrt[12]{2}\big)^{(n-a)}</annotation></semantics></math>
or <!--P_n = P_a3^{(n-a)/12} -->
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block" class="tml-display" style="display:inline-block;"><semantics><mrow><msub><mi>P</mi><mi>n</mi></msub><mo>=</mo><msub><mi>P</mi><mi>a</mi></msub><msup><mn>2</mn><mrow><mo form="prefix" stretchy="false" lspace="0em" rspace="0em">(</mo><mi>n</mi><mo>−</mo><mi>a</mi><mo form="postfix" stretchy="false">)</mo><mo lspace="0em" rspace="0em">⁄</mo><mn>12</mn></mrow></msup></mrow><annotation encoding="application/x-tex">P_n = P_a2^{(n-a)/12}</annotation></semantics></math>
where n is the index of the second tone and a is the index of the first tone a starting at one. 
Which means that to go one semitone up you have to multiply your current frequency by <!-- 2^{(2-1)/12} --> 
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block" class="tml-display" style="display:inline-block;"><semantics><msup><mn>2</mn><mrow><mo form="prefix" stretchy="false" lspace="0em" rspace="0em">(</mo><mn>2</mn><mo>−</mo><mn>1</mn><mo form="postfix" stretchy="false">)</mo><mo lspace="0em" rspace="0em">⁄</mo><mn>12</mn></mrow></msup><annotation encoding="application/x-tex">2^{(2-1)/12}</annotation></semantics></math>
which is approximately equal to 1.059463...

This serves the purpose of making sure all steps have the same size, relative to their base frequency (every step is 100 cents).
e.g multiplying a frequency by 
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block" class="tml-display" style="display:inline-block;"><semantics><msup><mn>2</mn><mrow><mo form="prefix" stretchy="false" lspace="0em" rspace="0em">(</mo><mn>2</mn><mo>−</mo><mn>1</mn><mo form="postfix" stretchy="false">)</mo><mo lspace="0em" rspace="0em">⁄</mo><mn>12</mn></mrow></msup><annotation encoding="application/x-tex">2^{(2-1)/12}</annotation></semantics></math>
7 times in a row is the same as going 7 steps at once, which is a nice property that's not true for all tuning systems.

proof:
<!--
2^((8-1)/12) = (2^((2-1)/12))^7
2^(7/12) = 2^(((2-1)/12)*7)
2^(7/12) = 2^((14-7)/12)
2^(7/12) = 2^(7/12)

2^{ \frac{{8-1}}{{12}} } = \left( 2^{ \frac{{2-1}}{{12}} } \right)^7
2^{ \frac{{7}}{{12}} } = 2^{ \left( \frac{{2-1}}{{12}} \right) \cdot 7 }
2^{ \frac{{7}}{{12}} } = 2^{ \frac{{14-7}}{{12}} }
2^{ \frac{{7}}{{12}} } = 2^{ \frac{{7}}{{12}} }
-->
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block" class="tml-display" style="display:inline-block">
   <semantics>
      <mrow>
         <mtable columnalign="center" align="axis 3">
            <mtr>
               <mtd>
                  <msup>
                     <mn>2</mn>
                     <mfrac>
                        <mrow>
                           <mn>8</mn>
                           <mo>−</mo>
                           <mn>1</mn>
                        </mrow>
                        <mn>12</mn>
                     </mfrac>
                  </msup>
               </mtd>
               <mtd>
                  <mo>=</mo>
               </mtd>
               <mtd>
                  <msup>
                     <mrow>
                        <mo fence="true" form="prefix">(</mo>
                        <msup>
                           <mn>2</mn>
                           <mfrac>
                              <mrow>
                                 <mn>2</mn>
                                 <mo>−</mo>
                                 <mn>1</mn>
                              </mrow>
                              <mn>12</mn>
                           </mfrac>
                        </msup>
                        <mo fence="true" form="postfix">)</mo>
                     </mrow>
                     <mn>7</mn>
                  </msup>
               </mtd>
            </mtr>
            <mtr>
               <mtd>
                  <msup>
                     <mn>2</mn>
                     <mfrac>
                        <mn>7</mn>
                        <mn>12</mn>
                     </mfrac>
                  </msup>
               </mtd>
               <mtd>=</mtd>
               <mtd>
                  <msup>
                     <mn>2</mn>
                     <mrow>
                        <mrow>
                           <mo fence="true" form="prefix">(</mo>
                           <mfrac>
                              <mrow>
                                 <mn>2</mn>
                                 <mo>−</mo>
                                 <mn>1</mn>
                              </mrow>
                              <mn>12</mn>
                           </mfrac>
                           <mo fence="true" form="postfix">)</mo>
                        </mrow>
                        <mo>⋅</mo>
                        <mn>7</mn>
                     </mrow>
                  </msup>
                  <msup>
               </mtd>
            </mtr>
            <mtr>
               <mtd>
                  <msup>
                     <mn>2</mn>
                     <mfrac>
                        <mn>7</mn>
                        <mn>12</mn>
                     </mfrac>
                  </msup>
               </mtd>
               <mtd>=</mtd>
               <mtd>
                  <msup>
                     <mn>2</mn>
                     <mfrac>
                        <mrow>
                           <mn>14</mn>
                           <mo>−</mo>
                           <mn>7</mn>
                        </mrow>
                        <mn>12</mn>
                     </mfrac>
                  </msup>
               </mtd>
            </mtr>
            <mtr>
               <mtd>
                  <msup>
                     <mn>2</mn>
                     <mfrac>
                        <mn>7</mn>
                        <mn>12</mn>
                     </mfrac>
                  </msup>
               </mtd>
               <mtd>=</mtd>
               <mtd>
                  <msup>
                     <mn>2</mn>
                     <mfrac>
                        <mn>7</mn>
                        <mn>12</mn>
                     </mfrac>
                  </msup>
               </mtd>
            </mtr>
         </mtable>
      </mrow>
   </semantics>
</math>

here's a table of the ratios (rounded to 6 decimal places)
```
+-----+----------+----+----------+
| N   | Ratio    | N  | Ratio    |
+-----+----------+----+----------+
| -12 | 0.5      | 1  | 1        |
| -11 | 0.529732 | 2  | 1.059463 |
| -10 | 0.561231 | 3  | 1.122462 |
| -9  | 0.594604 | 4  | 1.189207 |
| -8  | 0.629961 | 5  | 1.259921 |
| -7  | 0.629961 | 6  | 1.33484  |
| -6  | 0.66742  | 7  | 1.414214 |
| -5  | 0.707107 | 8  | 1.498307 |
| -4  | 0.749154 | 9  | 1.587401 |
| -3  | 0.793701 | 10 | 1.681793 |
| -2  | 0.840896 | 11 | 1.781797 |
| -1  | 0.890899 | 12 | 1.887749 |
| 0   | 0.943874 | 13 | 2        |
+-----+----------+----+----------+
```
# Just Intonation:
In Just Intonation we take the ratios directly from the overtone series.
so as an exercise let's derrive them ourselves:
as a base frequency we'll use 1
to construct the overtone series we just start multiplying it with the Natural Number series:
let's have a look at the 64 first overtones

we can calculate the ratios by diving the overtone's frequency(or it's ratio to the base tone) by the next smaller power of 2

```
┬─[hill@nixos:~]─[20時10分43秒]─[I]
╰─>λ math 7 /4
1.75
┬─[hill@nixos:~]─[20時10分54秒]─[I]
╰─>λ math 9 / 8
1.125
┬─[hill@nixos:~]─[20時11分02秒]─[I]
╰─>λ math 11 / 8
1.375
┬─[hill@nixos:~]─[20時11分12秒]─[I]
╰─>λ math 13 / 8
1.625
┬─[hill@nixos:~]─[20時11分26秒]─[I]
╰─>λ math 15 / 8
1.875
┬─[hill@nixos:~]─[20時11分44秒]─[I]
╰─>λ math 17 / 8
2.125
┬─[hill@nixos:~]─[20時11分55秒]─[I]
╰─>λ math 17 / 16
1.0625
```
here are some tables
```
+----------+--------+---------+-------+             +----------+----+---------+----------+
| Overtone | N      | Ratio   |       |             | Overtone | N  | Ratio   | Multiple |
+----------+--------+---------+-------+             +----------+----+---------+----------+
| 1        | 1      | 1       | 1/1   |             | 1        | 1  | 1       | 1/1      |
| 3        | 8      | 1.5     | 3/2   |             | 3        | 8  | 1.5     | 3/2      |
| 5        | 5      | 1.25    | 5/4   |             | 5        | 5  | 1.25    | 5/4      |
| 7        | unused | 1.75    | 7/4   |             | 9        | 3  | 1.125   | 9/8      |
| 9        | 3      | 1.125   | 9/8   |             | 15       | 12 | 1.875   | 15/8     |
| 11       | unused | 1.375   | 11/8  |             | 17       | 2  | 1.0625  | 17/16    |
| 13       | unused | 1.625   | 13/8  |             | 19       | 4  | 1.1875  | 19/16    |
| 15       | 12     | 1.875   | 15/8  |             | 27       | 10 | 1.6875  | 27/16    |
| 17       | 2      | 1.0625  | 17/16 |             | 45       | 7  | 1.40625 | 45/32    |
| 19       | 4      | 1.1875  | 19/16 |             | 51       | 9  | 1.59375 | 51/32    |
| 21       | unused | 1.3125  | 21/16 |             | 57       | 11 | 1.78125 | 57/32    |
| 23       | unused | 1.4375  | 23/16 |             +----------+----+---------+----------+
| 25       | unused | 1.5625  | 25/16 |             
| 27       | 10     | 1.6875  | 27/16 |
| 29       | unused | 1.8125  | 29/16 |
| 31       | unused | 1.9375  | 31/32 |
| 33       | unused | 1.03125 | 33/32 |
| 35       | unused | 1.09375 | 35/32 |
| 37       | unused | 1.15625 | 37/32 |
| 39       | unused | 1.21875 | 39/32 |
| 41       | unused | 1.28125 | 41/32 |
| 43       | unused | 1.34375 | 43/32 |
| 45       | 7      | 1.40625 | 45/32 |
| 47       | unused | 1.46875 | 47/32 |
| 49       | unused | 1.53125 | 49/32 |
| 51       | 9      | 1.59375 | 51/32 |
| 53       | unused | 1.65625 | 53/32 |
| 55       | unused | 1.71875 | 55/32 |
| 57       | 11     | 1.78125 | 57/32 |
| 59       | unused | 1.84375 | 59/32 |
| 61       | unused | 1.90625 | 61/32 |
| 63       | unused | 1.96875 | 63/32 |
+----------+--------+---------+-------+

```
skipping over any duplicate ratios, we can find all 12 tones of the western tuning system, apart from the perfect fourth. in the first 64 overtones.
the reason we can't find the perfect fourth is that it's ratio of 4/3 has a rational denominator so it can never be part of the overtone series directly.
i.e. <math xmlns="http://www.w3.org/1998/Math/MathML" display="block" class="tml-display" style="display:inline-block;"><semantics><mrow><mo fence="true" symmetric="true" minsize="2.4em" maxsize="2.4em">{</mo><mfrac><mn>4</mn><mn>3</mn></mfrac><mo>×</mo><msup><mn>2</mn><mi>n</mi></msup><mo fence="false" stretchy="true" symmetric="true" minsize="2.4em" maxsize="2.4em">|</mo><mi>n</mi><mo>∈</mo><mi>ℕ</mi><mo fence="true" symmetric="true" minsize="2.4em" maxsize="2.4em">}</mo><mo>⊆</mo><mi>ℚ</mi><mo>∖</mo><mi>ℕ</mi></mrow><annotation encoding="application/x-tex">\biggl\{\frac{4}{3}\times 2^{n} \bigg| n \in \mathbb{N}\biggr\}\subseteq \mathbb{Q}\setminus\mathbb{N}</annotation></semantics></math> but it is present nonetheless as the ratio between individual overtones, for example between the 3rd and the 4th overtone (4/3).

The nice thing about Just intonation is that we have exact ratios,

<!--
# Why did I decide to try make a [new conflicting standard](https://xkcd.com/927/)

I was frustrated with 12 TET and watched a video on Just Intonation,
I immediately realized the impracticality of it so I decided to make my own even less practical version.
c-->
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






# My other music related work:

### Visualize and listen to Polyrhythms in a Shader:

<iframe width="640" height="360" frameborder="0" allowfullscreen="allowfullscreen" src="https://www.shadertoy.com/embed/7tV3WV?gui=true&t=10&paused=false&muted=false"></iframe> 



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