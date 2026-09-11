---
title: Pascal my friend
tags:
  - philosophy
  - religion
  - tools
  - wasm
  - rust
---

<link href="./pascal.css" rel="stylesheet" type="text/css">
<script type="module" src="/js/pascal.js"></script>

Pascal's wager has two rows and two columns. Either God exists or he does not; either you believe or you do not. Believing costs a little if he does not exist and wins infinitely if he does, so any probability above zero on the first row makes believing the better bet.

The standard objection, older than Diderot, is that the table has more rows than two. Every religion that promises an infinity puts one in a different cell, and several of them promise a negative infinity to exactly the people Pascal's column would produce. Below is that table, filled in for fifty traditions, every one of them taken as the true account of the world once (the rows), and every one of them followed once (the columns). A cell says what the row's tradition teaches about somebody who spent a lifetime in the column's.

## how the cells are counted

An outcome is written as _eternal · ∞ + finite_.

The coefficient on infinity is +1 for a paradise that does not end and −1 for a torment that does not end. Where a tradition itself leaves the outcome to chance, it is the chance of paradise minus the chance of hell: Catholicism gives a baptised Catholic in a state of grace something like nine chances in ten, and that is +0.8∞. Where a tradition gives an eternity that is neither bliss nor pain, the Greek asphodel meadows or the Mesopotamian house of dust, it is a small negative number. Where nothing lasts forever, as in Judaism, Norse religion, or the Jehovah's Witnesses' destruction of the wicked, it is 0.

The finite part is everything that ends, measured in ordinary human lifetimes: a purgatory, a hell that empties, feasting until Ragnarök, a run of rebirths. Above the table each column has a cost of practice in the same unit, my rough estimate of what a lifetime of tithes, fasts, prayers, dietary law, fees and shunning costs after subtracting what the practice gives back, and you can change it.

Two expected utilities of that shape compare infinite part first, and finite part only when the infinite parts tie. That is what makes the calculation possible: a prior spread over the rows never has to evaluate ∞ − ∞, it only averages the coefficients. It also means the finite part almost never decides anything. Any tradition with a nonzero weight and an infinity in the cell outweighs every cost of practice in the column, and only when all the infinities cancel does it matter whether the religion was expensive.

The person in every column is the same decent person. Only creed, rite, burial and diet differ, so a tradition that judges by conduct alone gives every column the same verdict, and a tradition that judges by creed does not. The cells are my reading of what each tradition teaches, and I have tried to describe each as its own teachers describe it rather than as its critics do. Point at a cell to see which teaching it is based on. If I have misread yours, leave a comment.

## where the numbers come from

Three kinds of number go into the table, and they are not equally solid.

The **verdicts** are the part that matters and the part I could get wrong. Each one is my reading of what a tradition teaches, and each cell carries a note naming the teaching it rests on: a scripture where one settles the question (Qur'an 3:85, 1 Corinthians 15:22, Analects 11.12), a council or confession where one does (Lumen Gentium 16, Westminster Confession chapter 3), or the doctrinal term a tradition uses for the case (Madhva's *tamo-yogya*, the Valentinian *hylic*, Amida's eighteenth vow and its exclusion clause). Under the table there is a list of where I read each row, one entry per tradition, so you can check any row against something that is not me. Where a tradition disagrees with itself I have said so in the cell rather than picking a winner, which is why several cells are odds rather than a verdict.

The **adherent shares** behind the headcount button are the Pew Research Center's [2020 estimates](https://www.pewresearch.org/religion/2025/06/09/how-the-global-religious-landscape-changed-from-2010-to-2020/): Christians 28.8%, Muslims 25.6%, unaffiliated 24.2%, Hindus 14.9%, Buddhists 4.1%, other religions 2.2%, Jews 0.2%. Pew does not break those down as far as this table does, so the split within each family is mine, and a test in the build refuses the table if a family's rows stop adding up to Pew's number for it. The unaffiliated are not all naturalists and I have counted them as though they were, which overstates that row.

The **costs of practice** are guesses. They are the one column of numbers with no source behind them at all, which is why they are editable, and they almost never change the answer anyway.

## the table

Put a probability on each row. They do not have to sum to anything, they are normalised.

The rows start on my own prior: naturalism at ninety percent, and the remaining ten split between the other forty-nine in proportion to how many people hold them, on the reasoning that if some religion is true it is likelier to be one many people hold than one nobody does. That is a statement about me and not about the world, so the buttons will also give you a uniform prior, a prior weighted only by headcount, and an empty one to fill in yourself.

<div id="pascal-wager" class="pascal-wager"></div>

<p class="wasm-credit">made with rust compiled to wasm</p>

## what the table says

A few things hold whatever prior you put in.

The wager assumes that belief is what is rewarded, and most of the table does not agree. Judaism, Zoroastrianism, Sikhism, the Yoruba tradition and the Baháʼí Faith judge by conduct. The Aztecs judged by manner of death. Every school of Buddhism but one, and most of Hinduism, promise everyone the same end after a different number of lives. Calvinism says the matter was settled before you were born and your column is evidence about it, not a cause, which is a decision problem of its own.

The traditions that damn are few and they damn each other. Sunni and Shia Islam, evangelical Protestantism and Calvinism put −∞ on almost every other column, and on each other. Dvaita Vedanta, alone in Hinduism, has souls whose nature is eternal darkness, and it counts the Advaitins among them. Nichiren put every other Buddhist in the hell of incessant suffering and left the atheists, who never heard the Lotus Sutra and so never slandered it, better off.

The infinities decide the result, and a small probability is still a probability. At the default prior every religion together is ten percent, and that ten percent settles the ranking completely: the ninety percent on naturalism is indifferent between the columns except for their cost, so the columns are ordered entirely by the traditions I think are almost certainly false. Pascal's structure survives the objection this far: a small probability on an infinity still decides the ranking. What it does not survive is that the ten percent contains rows pointing in opposite directions, so the infinities partly cancel and which column wins depends on how the ten percent is divided.

With the rows weighted by headcount instead, the Islamic rows put −∞ on every non-Muslim column at a combined weight of about a quarter, and the Christian rows put a fractional −∞ on every non-Christian column at a combined weight of about the same, and the universalist rows give +∞ to every column and so change nothing between columns. Which column wins is then decided by which of those two blocks you weight more, since the remaining rows give every column the same verdict. Set the atheism row to a hundred percent and the question becomes which practice is cheapest.
