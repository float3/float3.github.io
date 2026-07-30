---
title: when to buy insurance
date: 2026-07-30
updated: 2026-07-30
tags:
  - economics
---
My argument against buying most types of insurances is two pronged:

If you're buying a train ticket, a flight ticket, or any electronics today, you're often offered insurance or an extended warranty on it. Most of the time, for the majority of people, this is a scam.

Insurance is not an investment or a way to save money.  
Insurance is a tool for transferring catastrophic risk.

If you think about it like this:

Insurance companies need to be profitable to survive. They have administrative costs, employees, &c. That means they need to make a profit, and in order to make a profit, the average customer of an insurance company has to have negative expected value. If, on average, the expected value for the customer was positive, the insurance company would go bankrupt.

So why pay for any insurance at all if, on average, the expected value is negative for you?

If someone offered you a bet  
Heads: lose everything you own  
Tails: you get 100,000€ (replace with an amount that's reasonable for your financial situation)


<div style="max-width:600px;padding:1em;border:1px solid ccc;border-radius:8px">

<label>
Chance of Tails (%):
<br>
<input id="tailsProb" type="number" value="50" min="0" max="100" step="0.1">
</label>
<br>
<label>
Current Net Worth (€):
<br>
<input id="netWorth" type="number" value="50000" min="0" step="1000">
</label>
<br>
<label>
Reward if Tails (€):
<br>
<input id="reward" type="number" value="100000" min="0" step="1000">
</label>

<div id="results"></div>

</div>

<script>
function money(x) {
    return "€" + x.toLocaleString(undefined, {
        maximumFractionDigits: 2
    });
}

function updateCalculator() {
    const p = Number(document.getElementById("tailsProb").value) / 100;
    const wealth = Number(document.getElementById("netWorth").value);
    const reward = Number(document.getElementById("reward").value);

    const wealthHeads = 0;
    const wealthTails = wealth + reward;

    const expectedWealth =
        (1 - p) * wealthHeads +
        p * wealthTails;

    const expectedGain = expectedWealth - wealth;

    const breakEvenReward =
        p > 0 ? wealth / p - wealth : Infinity;

    document.getElementById("results").innerHTML = `
        <b>If Heads:</b> ${money(wealthHeads)}<br>
        <b>If Tails:</b> ${money(wealthTails)}<br><br>

        <b>Expected Wealth:</b> ${money(expectedWealth)}<br>
        <b>Expected Gain/Loss:</b>
        <span style="color:${expectedGain >= 0 ? "green" : "red"}">
            ${money(expectedGain)}
        </span>
        <br><br>

        <b>Probability of Ruin:</b> ${(100 * (1 - p)).toFixed(1)}%<br>
        <b>Break-even Reward:</b> ${
            Number.isFinite(breakEvenReward)
                ? money(breakEvenReward)
                : "Impossible"
        }
    `;
}

document.querySelectorAll("input").forEach(i =>
    i.addEventListener("input", updateCalculator)
);

updateCalculator();
</script>

For this example, if you own 50,000€, taking the deal will result in you gaining 25,000€ on average. But I would guess that most people who have 50,000€ would not take this deal. This is because the first 10,000€ are way more important than the tenth 10,000€. Most people don't optimize for money but for utility instead. Money can sometimes be an OK proxy for utility, but it breaks down in cases like this. This should be obvious to you but if it isn't try to imagine getting X€ of money at different levels of net worth.

Now, insurance companies know exactly how much to charge for the average person to have negative expected value on any single insurance purchase. The reason it works is marketing strategies and psychology. I assume people are more likely to go, "What's another {5% of the total price}€?" at checkout right before they're about to pay for their flight, train ticket, or new washing machine.

Whenever you're offered insurance, ask yourself: would paying for this myself ruin me, or could I comfortably replace it? If you can replace it, don't buy the insurance UNLESS you have a very good reason to believe that you are unusually likely to make a claim, or rather, is the premium reasonable relative to my risk? But this requires extraordinary evidence, like being exceptionally clumsy and regularly losing expensive items, having a chronic condition, or routinely canceling flights.

Remember:  
If the loss would be catastrophic and you can't comfortably absorb it, buy the insurance.  
If the loss is merely inconvenient and you can comfortably cover it yourself, decline the insurance.

The second argument is that insurance companies want to pay as little claims as possible, and in many parts of the world they employ, full-time, huge stables of employees that make it harder or try to prevent you from getting your money and huge stables of lawyers that will try to deter you from going to and beat you in court.


 That said some people I've talked to seem to deal with very severe anxiety regarding all sorts of things, these insurances can give them peace of mind, which might be a worth tradeoff, depending on someone's priorities. Of course you are responsible for your own financial decision, while the above text is my opinion, and it is advice, common financial wisdom, please don't sue me for any damages you suffer from not having insurance.
