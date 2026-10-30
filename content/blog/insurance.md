---
title: when to buy insurance - this is financial advice
tags:
  - economics
  - finances
---

edit: if you want to see the proper math of when an insurance is worth it, check out [this post](https://www.lesswrong.com/posts/wf4jkt4vRH7kC2jCy/when-is-insurance-worth-it) on lesswrong. \
if you want to just calculate if an insurance is worth it, use the [calculator](https://xkqr.org/insurance/).

my argument against buying most types of insurances is two pronged:

if you're buying a train ticket, a flight ticket, or any electronics today, you're often offered insurance or an extended warranty on it. most of the time, for the majority of people, this is a scam.

insurance is not an investment or a way to save money.  
insurance is a tool for transferring catastrophic risk.

if you think about it like this:

insurance companies need to be profitable to survive. they have administrative costs, employees, &c. that means they need to make a profit, and in order to make a profit, the average customer of an insurance company has to have negative expected value. if, on average, the expected value for the customer was positive, the insurance company would go bankrupt.

so why pay for any insurance at all if, on average, the expected value is negative for you?

if someone offered you a bet  
heads: lose everything you own  
tails: you get 100,000€ (replace with an amount that's reasonable for your financial situation)


<div id="insurance-calculator" class="tool tool-panel insurance">
<div class="tool-bar">
<label class="tool-field"><span>Chance of tails</span><input id="tailsProb" type="number" value="50" min="0" max="100" step="0.1" /> </label>
<label class="tool-field"><span>Net worth, €</span><input id="netWorth" type="number" value="50000" min="0" step="1000" /></label>
<label class="tool-field"><span>Reward if tails, €</span><input id="reward" type="number" value="100000" min="0" step="1000" /></label>
</div>
<div id="results" class="insurance-results"></div>
</div>

</div>

<script type="module" src="/js/insurance.js"></script>

for this example, if you own 50,000€, taking the deal will result in you gaining 25,000€ on average. but i would guess that most people who have 50,000€ would not take this deal. this is because the first 10,000€ are way more important than the tenth 10,000€. most people don't optimize for money but for utility instead. money can sometimes be an OK proxy for utility, but it breaks down in cases like this. try to imagine getting X€ of money at different levels of net worth.

now, insurance companies know exactly how much to charge for the average person to have negative expected value on any single insurance purchase. the reason it works is marketing strategies and psychology. i assume people are more likely to go, "What's another {5% of the total price}€?" at checkout right before they're about to pay for their flight, train ticket, or new washing machine.

whenever you're offered insurance, ask yourself: would paying for this myself ruin me, or could i comfortably replace it? if you can replace it, don't buy the insurance UNLESS you have a very good reason to believe that you are unusually likely to make a claim, or rather, is the premium reasonable relative to my risk? but this requires extraordinary evidence, like being exceptionally clumsy and regularly losing expensive items, having a chronic condition, or routinely canceling flights.

remember:  
if the loss would be catastrophic and you can't comfortably absorb it, buy the insurance.  
if the loss is merely inconvenient and you can comfortably cover it yourself, decline the insurance.

the second argument is that insurance companies want to pay out as few claims as possible, and in many parts of the world they employ, full-time, large teams of employees that make it harder or try to prevent you from getting your money and large teams of lawyers that will try to deter you from going to court, and beat you there.


some people i've talked to seem to deal with very severe anxiety regarding all sorts of things, if this is you, these bad insurances could give you peace of mind, which might seem like a worth tradeoff for you, but i think you should be aware that you're committing a (mathematical) mistake, the question of whether to buy an insurance always has a correct answer. of course you are responsible for your own financial decisions; please don't sue me for any damages you suffer from not having insurance.


FAQ: 

Q: so should i buy insurance for every thing that could financially ruin me, no matter how small the risk?

a: so in a properly adjusted market, the price of an insurance that is very costly (financial ruin) but extremely unlikely will be very cheap, and everyone should get it. the cost of the insurance will be proportional to the chance of you claiming the insurance. in a properly adjusted, efficient insurance market, small cheap insurances for super unlikely things could exist and would be worth it and rational for you to buy.  
