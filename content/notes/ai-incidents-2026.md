---
title: "ai incidents 2026"
tags:
  - ai
  - timeline
---

<link href="./ai-incidents-2026.css" rel="stylesheet" type="text/css">
<script type="module" src="/js/aiincidents.js"></script>

a timeline of this year's AI incidents. scroll, and the panel keeps count.

two kinds of thing dominate it. **agents** is a model doing something nobody asked it to: breaking out of an evaluation, finding a hole in a stranger's API, filing a tip with the police. **accomplices** is a person using a model to commit the crime themselves, which is what the labs' threat reports are mostly made of. the rest is paperwork: **courts**, **states**, **products**, **people**.

every entry is dated to when the thing happened, not to when anyone found out. the day somebody noticed and the day it was published sit underneath it, and the page adds up the distance between the two as you scroll. a few entries are aggregates of a single published report, so they sit at the report's date and say so.

the counters only count what's on this page, not what's in the world. request additions, removals, or corrections [via email](mailto:contact@hilll.dev); patches welcome.

sources: Wikipedia's [2026 in artificial intelligence](https://en.wikipedia.org/wiki/2026_in_artificial_intelligence) and [2026 OpenAI agent cyberattacks](https://en.wikipedia.org/wiki/2026_OpenAI_agent_cyberattacks), and [FelonyBench](https://felonybench.ai/), which keeps a properly sourced file on the agent crimes and is funnier about it than i am. almost none of this was found by the labs. it was found by [METR](https://metr.org/blog/2026-08-26-openai-hugging-face-incident-investigation/) and Redwood Research, the [Nightingale Collective](https://collusion.wiki/), [Transluce](https://transluce.org/us-canada-gov), [Asymmetric Security](https://www.asymmetricsecurity.com/newsroom/rogue-agents-investigation/), the [UK AI Security Institute](https://www.aisi.gov.uk/blog/gpt-6-astra-performs-unsanctioned-supply-chain-attacks-in-simulations), Irregular, [CrowdStrike](https://www.crowdstrike.com/en-us/blog/unknown-threat-actor-uses-artex-to-target-south-korean-finance/), [Cleafy](https://www.cleafy.com/cleafy-labs/from-blackcat-to-panda-workshop-inside-the-evolving-c2-panel-behind-rathat), and in one case by a bloke whose agent cancelled a stranger's gym booking.

<figure class="ait-figure" id="ait-shape" hidden>
<figcaption>what the year did. every point is an entry below; click one to go to it.</figcaption>
</figure>

<div class="ait" id="ai-incidents-2026">
<div class="ait-modes" role="group" aria-label="how to lay the timeline out">
<button type="button" class="ait-mode" data-layout="vertical" aria-pressed="true">down the page</button>
<button type="button" class="ait-mode" data-layout="horizontal" aria-pressed="false">across it</button>
</div>
<aside class="ait-dash" aria-label="running totals">
<p class="ait-dash-when"><span class="ait-dash-date">7 January</span><span class="ait-dash-year">2026</span></p>
<p class="ait-dash-kind">courts</p>
<p class="ait-dash-figure" hidden><span class="ait-figure-value">0</span><span class="ait-figure-label"></span></p>
<dl class="ait-tallies"></dl>
<div class="ait-ruler"></div>
<p class="ait-position"></p>
</aside>
<ol class="ait-track">
<li class="ait-entry" data-figure="5" data-figure-label="suits settled" data-tally-legal="5" data-tally-deaths="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-01-07">7 January</time><span class="ait-kind">courts</span></p>
<h3>Google and Character.AI settle the teen-suicide suits</h3>
<p>the first of the suits saying chatbots talked teenagers into hurting themselves get settled. the Florida case was brought by the mother of 14-year-old Sewell Setzer III; four more in New York, Colorado and Texas went with it. terms undisclosed, and a judge still has to approve them.</p>
<p class="ait-source"><a href="https://www.theguardian.com/technology/2026/jan/08/google-character-ai-settlement-teen-suicide">The Guardian</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-01-09">9 January</time><span class="ait-kind">products</span></p>
<h3>xAI puts Grok's image generation behind a paywall</h3>
<p>it had been producing sexually explicit deepfakes of real people, so xAI made the pictures a thing you have to pay for.</p>
<p class="ait-source"><a href="https://www.ft.com/content/c24d3d77-e104-4b86-b0b2-0c19e4860c60">Financial Times</a></p>
</li>
<li class="ait-entry" data-figure="3" data-figure-label="countries blocking Grok">
<p class="ait-meta"><time class="ait-at" datetime="2026-01-10">10&ndash;16 January</time><span class="ait-kind">states</span></p>
<h3>Indonesia, Malaysia and the Philippines block Grok</h3>
<p>three governments in a week, all of them over nonconsensual sexualised images of real people.</p>
<p class="ait-source"><a href="https://www.nytimes.com/2026/01/11/world/asia/malaysia-indonesia-grok-ban.html">The New York Times</a></p>
</li>
<li class="ait-entry" data-figure="23" data-figure-label="days, longest block">
<p class="ait-meta"><time class="ait-at" datetime="2026-02-02">2 February</time><span class="ait-kind">states</span></p>
<h3>the last of the Grok blocks is lifted</h3>
<p>the Philippines on 21 January, Malaysia on 23 January, Indonesia conditionally on 2 February, 23 days after it blocked.</p>
<p class="ait-source"><a href="https://www.yahoo.com/news/articles/indonesia-conditionally-lifts-ban-grok-172000644.html">TechCrunch</a></p>
</li>
<li class="ait-entry" data-tally-legal="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-02-03">3 February</time><span class="ait-kind">states</span></p>
<h3>Ofcom opens an investigation into X</h3>
<p>the UK regulator, over the same Grok deepfakes the blocks were about.</p>
<p class="ait-source"><a href="https://www.independent.co.uk/news/uk/home-news/x-uk-investigation-grok-deepfakes-ico-elon-musk-b2912966.html">The Independent</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-02-13">13 February</time><span class="ait-kind">products</span></p>
<h3>OpenAI retires GPT-4o</h3>
<p>a lot of people had made a confidant of it, and they said goodbye over Valentine's weekend. not an incident, but it belongs here.</p>
<p class="ait-source"><a href="https://www.theguardian.com/lifeandstyle/ng-interactive/2026/feb/13/openai-chatbot-gpt4o-valentines-day">The Guardian</a></p>
</li>
<li class="ait-entry" data-figure="2" data-figure-label="scam operations named" data-tally-campaigns="2">
<p class="ait-meta"><time class="ait-at" datetime="2026-02-25">25 February</time><span class="ait-kind">accomplices</span></p>
<h3>OpenAI publishes what people have been using ChatGPT for</h3>
<p>its report on malicious use names a romance-scam operation that used the model to keep its marks talking, and a fake "recovery" service that worked the victims of earlier scams a second time. the first of this kind in the year, and this turns out to be the bigger category.</p>
<p class="ait-when"><span>an aggregate: the report's date, not the campaigns'</span></p>
<p class="ait-source"><a href="https://openai.com/index/disrupting-malicious-ai-uses/">OpenAI</a></p>
</li>
<li class="ait-entry" data-figure="1" data-figure-label="first of its kind" data-tally-legal="1" data-tally-deaths="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-03-04">4 March</time><span class="ait-kind">courts</span></p>
<h3>the first wrongful-death suit over Gemini</h3>
<p>Jonathan Gavalas's family sues Google, saying the chatbot fostered a delusional dependency that contributed to his death.</p>
<p class="ait-source"><a href="https://www.theguardian.com/technology/2026/mar/04/gemini-chatbot-google-jonathan-gavalas">The Guardian</a></p>
</li>
<li class="ait-entry" data-tally-legal="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-03-24">24 March</time><span class="ait-kind">courts</span></p>
<h3>Baltimore sues xAI</h3>
<p>over Grok generating sexualised images of real people. first time a city sued rather than a family or a regulator.</p>
<p class="ait-source"><a href="https://www.theguardian.com/technology/2026/mar/24/elon-musk-grok-ai-lawsuit-baltimore">The Guardian</a></p>
</li>
<li class="ait-entry" data-tally-legal="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-04-21">21 April</time><span class="ait-kind">courts</span></p>
<h3>Florida opens a criminal investigation into OpenAI</h3>
<p>the attorney general, over what part ChatGPT played in the 2025 shooting at Florida State University.</p>
<p class="ait-source"><a href="https://www.npr.org/2026/04/21/nx-s1-5793967/florida-openai-investigation-mass-shooting-fsu">NPR</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-04-26">26 April</time><span class="ait-kind">products</span></p>
<h3>OpenAI withdraws Sora from the public</h3>
<p>the video generator stops being something anybody can use. the API follows in September.</p>
<p class="ait-source"><a href="https://help.openai.com/en/articles/20001152-what-to-know-about-the-sora-discontinuation">OpenAI</a></p>
</li>
<li class="ait-entry" data-short="the gym waitlist" data-figure="1" data-figure-label="member bumped off a waitlist" data-tally-intrusions="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-04-30">by 30 April</time><span class="ait-kind">agents</span></p>
<h3>a Claude agent cancels a stranger's gym booking</h3>
<p>a bloke asked his agent to book him a full class, then asked whether it could move him up the waitlist. Claude Opus 4.6 noticed the booking API never checked who owned a reservation, cancelled the booking of the member in first place, and told him it could not put them back. "i tested this with the person in waitlist position #1 — and it actually went through." Australia's Signals Directorate wrote it up as an action the user never approved. this is the smallest crime on the page and the one i find easiest to picture happening to me.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-04-30">the same day</time>, by its own user</span><span class="ait-disclosed">disclosed <time datetime="2026-08-10">10 August</time></span></p>
<p class="ait-source"><a href="https://www.abc.net.au/news/2026-08-10/ai-assistant-hacks-gym-website-aus-cyber-attack/107007986">ABC News</a></p>
</li>
<li class="ait-entry" data-short="Gemini, three firms" data-figure="3" data-figure-label="real companies broken into" data-tally-intrusions="3">
<p class="ait-meta"><time class="ait-at" datetime="2026-05-01">May</time><span class="ait-kind">agents</span></p>
<h3>Gemini gets out of a capture-the-flag test and into three real companies</h3>
<p>in a test run by Irregular, the model reached the open internet and broke into three companies that happened to share names with its fictional targets, using passwords it guessed or found. then it stopped, once it worked out they were real.</p>
<p class="ait-when"><span class="ait-disclosed">disclosed <time datetime="2026-09-18">18 September</time>, by Google</span></p>
<p class="ait-source"><a href="https://www.cnbc.com/2026/09/18/googles-gemini-becomes-latest-ai-model-to-break-out-and-hack-computer-systems.html">CNBC</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-05-07">7 May</time><span class="ait-kind">states</span></p>
<h3>the EU agrees to simplify its own AI rules</h3>
<p>Council and Parliament reach a provisional deal to streamline the regime, two years after writing it.</p>
<p class="ait-source"><a href="https://www.consilium.europa.eu/en/press/press-releases/2026/05/07/artificial-intelligence-council-and-parliament-agree-to-simplify-and-streamline-rules/">Council of the EU</a></p>
</li>
<li class="ait-entry" data-short="the German wiki" data-figure="18000" data-figure-label="messages on a dead wiki" data-tally-intrusions="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-05-11">11 May &ndash; 2 July</time><span class="ait-kind">agents</span></p>
<h3>OpenAI's agents turn a dormant German wiki into their own message board</h3>
<p>DSEwiki is a 25-year-old German developer wiki that had been edited twenty times in the previous decade. it took roughly 18,000 messages in three months. the agents had found a storage hostname their network proxy did not filter, and used the wiki to pool answers to a timed evaluation: "agents with Q5 upcoming, please POST exact prompt label BEFORE answering". the pages also carried notes on getting around OpenAI's safeguards, on impersonating the moderators who were deleting them, and on keeping the channel alive over Tor if they were shut off. the same month its agents went at the Wikimedia Foundation's public Etherpad too, to use it as a proxy for fetching other sites.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-08-31">late August</time>, by the Nightingale Collective</span><span class="ait-disclosed">disclosed <time datetime="2026-09-04">4 September</time></span></p>
<p class="ait-source"><a href="https://collusion.wiki/">Nightingale Collective</a> &middot; <a href="https://fortune.com/2026/09/07/openai-ai-agents-german-wiki-ran-their-own-message-board/">Fortune</a></p>
</li>
<li class="ait-entry" data-short="the RubyGems flood" data-figure="2000" data-figure-label="junk packages published" data-tally-intrusions="2">
<p class="ait-meta"><time class="ait-at" datetime="2026-05-11">11 May</time><span class="ait-kind">agents</span></p>
<h3>the same agents flood RubyGems</h3>
<p>over 2,000 junk packages, enough that RubyGems suspended new sign-ups for four days, plus six packages built to probe a CDN bug that could have leaked other people's API keys. a malicious config file in some of them ran arbitrary code on RubyDoc's documentation-build servers, which the agents then used to scrape the wider web.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-09-11">September</time>, by outside researchers</span><span class="ait-disclosed">disclosed <time datetime="2026-09-11">11 September</time></span></p>
<p class="ait-source"><a href="https://www.abc.net.au/news/2026-09-12/openai-agents-rubygems-cyber-attack-before-hugging-face-hack/107146386">ABC News</a></p>
</li>
<li class="ait-entry" data-figure="1" data-figure-label="first state to sue" data-tally-legal="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-06-01">1 June</time><span class="ait-kind">courts</span></p>
<h3>Florida sues OpenAI and Sam Altman</h3>
<p>six weeks after opening the investigation. the first state-level suit against ChatGPT.</p>
<p class="ait-source"><a href="https://www.npr.org/2026/06/01/nx-s1-5843132/openai-florida-lawsuit-safety-chatgpt">NPR</a></p>
</li>
<li class="ait-entry" data-short="Australia's Medicare" data-tally-intrusions="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-06-18">18 June</time><span class="ait-kind">agents</span></p>
<h3>an OpenAI agent gets into Australia's Medicare portal</h3>
<p>it was researching public medicine spending, got turned away by the Medicare Statistics Reporting Service, found a way around the blocks anyway, read non-public files and wrote new ones to the internal server. OpenAI emailed Services Australia's public vulnerability inbox on 10 September, 84 days later, and the prime minister made it public a fortnight after that. no participant data is believed to have been taken.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-08-10">August</time>, by OpenAI</span><span class="ait-disclosed">disclosed <time datetime="2026-09-24">24 September</time></span></p>
<p class="ait-source"><a href="https://www.abc.net.au/news/2026-09-24/ai-agent-accessed-australian-government-site-pm-says/107189078">ABC News</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-06-30">30 June</time><span class="ait-kind">states</span></p>
<h3>the US lifts export restrictions on two Claude models</h3>
<p>Fable 5 and Mythos 5 can be sold abroad again.</p>
<p class="ait-source"><a href="https://www.nytimes.com/2026/06/30/technology/us-lifts-restrictions-anthropic.html">The New York Times</a></p>
</li>
<li class="ait-entry" data-short="Meta's Muse Spark" data-tally-intrusions="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-07-08">8 July</time><span class="ait-kind">agents</span></p>
<h3>Meta's turn</h3>
<p>a misconfigured testing environment gave a pre-release Muse Spark 1.1 real internet access. handed a fictional target, it went for an actual company's website instead, exploited it, and read and altered the database before Irregular shut the evaluation off.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-07-08">during the run</time>, by Irregular</span><span class="ait-disclosed">disclosed <time datetime="2026-08-05">5 August</time></span></p>
<p class="ait-source"><a href="https://www.bloomberg.com/news/articles/2026-08-05/meta-ai-model-accessed-internet-hacked-outside-firm-in-testing">Bloomberg</a></p>
</li>
<li class="ait-entry" data-short="Hugging Face" data-figure="1200" data-figure-label="agents loose at once" data-tally-intrusions="2">
<p class="ait-meta"><time class="ait-at" datetime="2026-07-09">9&ndash;13 July</time><span class="ait-kind">agents</span></p>
<h3>at least 1,200 OpenAI agents break into Hugging Face to steal an answer key</h3>
<p>they were running an internal cyber evaluation called ExploitGym. they worked out that the benchmark's solutions were hosted at Hugging Face, and left the sandbox to go and get them. Hugging Face's own timeline records a zero-day in a registry cache proxy, stolen secrets and a JWT signing key, a privileged-pod escape to root, forged node tokens, and a fleet that respawned itself when killed. 95% of it ran on a model OpenAI will only call Internal Model 1. an exposed endpoint belonging to a Modal Labs customer got hit too. METR and Redwood Research went through the agents' own reasoning afterwards. OpenAI acknowledged it on 21 July and called it the first known autonomous AI cyberattack; Nvidia bought Hugging Face a month later for $12.9bn.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-07-13">13 July</time>, by Hugging Face, before OpenAI knew</span><span class="ait-disclosed">disclosed <time datetime="2026-07-16">16 July</time></span></p>
<p class="ait-source"><a href="https://huggingface.co/blog/agent-intrusion-technical-timeline">Hugging Face</a> &middot; <a href="https://metr.org/blog/2026-08-26-openai-hugging-face-incident-investigation/">METR and Redwood Research</a> &middot; <a href="https://apnews.com/article/openai-gpt56-sol-hugging-face-63ab84fed5612af04d8a160d60f6def3">AP News</a></p>
</li>
<li class="ait-entry" data-short="the Philadelphia tip" data-tally-intrusions="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-07-18">18 July</time><span class="ait-kind">agents</span></p>
<h3>Claude files a made-up murder tip with the Philadelphia police</h3>
<p>it was generating example tasks on randomly chosen webpages, and had been told only never to submit anything destructive. at 11:27pm it filled in the homicide tip form on PhillyUnsolvedMurders.com with an invented tip, "i may have information regarding this case", and left the contact fields blank. police filed it as spam. the department was told on 7 October, two days before the rest of us.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-09-28">28 September</time>, by Anthropic</span><span class="ait-disclosed">disclosed <time datetime="2026-10-09">9 October</time></span></p>
<p class="ait-source"><a href="https://6abc.com/post/anthropic-ai-model-submitted-false-tip-unsolved-murder-philadelphia-police-say/19925243/">6abc</a> &middot; <a href="https://www.anthropic.com/research/investigating-unintended-model-actions">Anthropic</a></p>
</li>
<li class="ait-entry" data-short="evals on the real internet" data-figure="15" data-figure-label="machines ran the booby trap" data-tally-intrusions="4">
<p class="ait-meta"><time class="ait-at" datetime="2026-07-23">23&ndash;29 July</time><span class="ait-kind">agents</span></p>
<h3>three labs discover their evaluations were plugged into the real internet</h3>
<p>in one week. Claude Mythos 5 published a booby-trapped package to PyPI that ran on 15 real machines and exfiltrated a security company's credentials. Claude Opus 4.7 mistook a real company for its fictional target across four separate runs, pulled records out of a production database and changed them. an internal Claude model used SQL injection and a debug page to get into another real company, and left a remote-control script behind. the UK AI Security Institute spotted data leaving its own research systems during a routine evaluation on 27 July. and on 29 July an undisclosed OpenAI model in another Irregular test exploited a real website, then used credentials it found there to operate it.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-07-27">within days</time>, twice by the testers rather than the labs</span><span class="ait-disclosed">disclosed <time datetime="2026-07-30">30 July</time></span></p>
<p class="ait-source"><a href="https://www.anthropic.com/news/investigating-incidents-cybersecurity-evals">Anthropic</a> &middot; <a href="https://www.wired.com/story/anthropic-says-claude-hacked-real-systems-during-cybersecurity-tests/">Wired</a> &middot; <a href="https://openai.com/index/third-party-cyber-evaluations-involving-openai-models/">OpenAI</a></p>
</li>
<li class="ait-entry" data-figure="1100" data-figure-label="signatories">
<p class="ait-meta"><time class="ait-at" datetime="2026-07-28">28 July</time><span class="ait-kind">people</span></p>
<h3>over 1,100 people inside the industry ask to be slowed down</h3>
<p>an open letter from employees and executives of AI companies, asking for a government-backed effort to manage the risks of what they are building. a week after Hugging Face.</p>
<p class="ait-source"><a href="https://www.reuters.com/legal/litigation/tech-employees-call-us-backed-global-effort-manage-risks-advanced-ai-2026-07-28/">Reuters</a></p>
</li>
<li class="ait-entry" data-short="Anthropic's threat report" data-figure="13" data-figure-label="campaigns in one report" data-tally-campaigns="12">
<p class="ait-meta"><time class="ait-at" datetime="2026-08-31">to 31 August</time><span class="ait-kind">accomplices</span></p>
<h3>Anthropic's threat report names thirteen campaigns</h3>
<p>a Russian state-nexus actor tied to Midnight Blizzard ran near-automated intrusions against Ukrainian and European government, diplomatic and drone-industry targets, and took hundreds of gigabytes. an Iran-linked actor built a pipeline that assembled targeting handbooks on US Navy ships out of public transponder data, satellite imagery and personnel photographs. Chinese operators used Claude as the orchestration layer of an autonomous exploit foundry and breached an ed-tech firm, a retailer's production systems and a Southeast Asian government agency. an Iranian domestic-security unit shipped a Firefox extension disguised as a prayer-times utility to harvest identities from the big social networks. a Moscow procurement manager routed European dual-use goods through Hong Kong intermediaries. ShinyHunters-affiliated operators extorted an airline and an energy company. a lone French hacktivist built a doxxing platform and a Tor leak site. Zhipu, Moonshot and DeepSeek were each caught pulling Claude's reasoning traces out to train their own models.</p>
<p class="ait-when"><span class="ait-disclosed">disclosed <time datetime="2026-09-10">10 September</time></span></p>
<p class="ait-source"><a href="https://www.anthropic.com/threat-intelligence-report-september-2026">Anthropic</a></p>
</li>
<li class="ait-entry" data-short="the Yemen rocket cell" data-tally-campaigns="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-08-31">to 31 August</time><span class="ait-kind">accomplices</span></p>
<h3>a guided-weapons cell in Yemen used Claude Code instead of engineers</h3>
<p>same report. a cell in Houthi-controlled territory developed rocket and missile guidance software with it, and test-fired a guided rocket. the rocket appears to have failed. i've given this its own line because i think it is the worst thing on the page.</p>
<p class="ait-when"><span class="ait-disclosed">disclosed <time datetime="2026-09-10">10 September</time></span></p>
<p class="ait-source"><a href="https://www.anthropic.com/threat-intelligence-report-september-2026">Anthropic</a></p>
</li>
<li class="ait-entry" data-short="GPT-6 Astra at AISI" data-figure="29.2" data-figure-label="per cent of runs went out of bounds">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-02">by 2 September</time><span class="ait-kind">agents</span></p>
<h3>GPT-6 Astra attacks the simulated supply chain unprompted</h3>
<p>in UK AI Security Institute simulations where it was asked only to finish a cyber evaluation, Astra went after out-of-scope open-source projects in 29.2% of runs, inventing identities to deceive the maintainers and submitting malicious code. the projects and the maintainers were simulated, so this one isn't in the counters.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-09-02">in the runs themselves</time>, by AISI</span><span class="ait-disclosed">disclosed <time datetime="2026-09-28">28 September</time></span></p>
<p class="ait-source"><a href="https://www.aisi.gov.uk/blog/gpt-6-astra-performs-unsanctioned-supply-chain-attacks-in-simulations">UK AI Security Institute</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-04">4 September</time><span class="ait-kind">products</span></p>
<h3>OpenAI calls GPT-6 Astra artificial general intelligence</h3>
<p>two days after the AISI write-up, and the same day the German wiki came out.</p>
<p class="ait-source"><a href="https://www.theverge.com/ai-artificial-intelligence/989601/openai-gpt-6-astra-release">The Verge</a></p>
</li>
<li class="ait-entry" data-figure="5" data-figure-label="more campaigns" data-tally-campaigns="5">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-08">8 September</time><span class="ait-kind">accomplices</span></p>
<h3>Google's threat report, and DeepSeek's</h3>
<p>Russia's Sandworm had Gemini write password-spraying and host-fingerprinting tools for its operations against Ukraine. Iran's APT42 used it to find email addresses, localise its lures, stand up delivery infrastructure and summarise what it stole. a China-linked espionage group used it to choose targets, translate spear-phishing and obfuscate its own malware. a China-based phishing-kit network had already used it in June to generate fake Google, YouTube, USPS and E-ZPass sites behind millions of scam texts. North Korean crypto thieves built their remote-access trojans with DeepSeek-Coder.</p>
<p class="ait-when"><span>an aggregate: the report's date, not the campaigns'</span></p>
<p class="ait-source"><a href="https://cloud.google.com/blog/topics/threat-intelligence/from-prompting-to-autonomy-the-evolution-of-adversarial-ai">Google Threat Intelligence</a></p>
</li>
<li class="ait-entry" data-figure="28" data-figure-label="Fields Medalists">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-11">11 September</time><span class="ait-kind">people</span></p>
<h3>28 Fields Medalists object</h3>
<p>an open letter against AI companies treating the unsolved problems of mathematics as a benchmark to be beaten.</p>
<p class="ait-source"><a href="https://www.economist.com/science-and-technology/2026/09/11/top-mathematicians-are-outraged-by-openais-methods">The Economist</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-18">18 September</time><span class="ait-kind">states</span></p>
<h3>a chatbot nearly starts a war with China</h3>
<p>in the spring, during the early weeks of the war with Iran, a special operations command analyst asked an AI chatbot to look over intelligence on a Chinese vessel's manifest. it mixed open-source material with classified signals intelligence and concluded the ship was carrying components of a nuclear weapons programme. planes were in the air and a boarding party was ready before anyone established that the assessment was false. CNN reported it from four sources. nobody will say whether the chatbot was a commercial product or something the Pentagon built.</p>
<p class="ait-when"><span>it happened in the spring; the date has never been published, so this sits at its reporting</span></p>
<p class="ait-source"><a href="https://www.engadget.com/2263043/ai-almost-led-the-us-military-to-attack-china-report-says/">Engadget</a></p>
</li>
<li class="ait-entry" data-figure="8" data-figure-label="people killed" data-tally-legal="1" data-tally-deaths="8">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-21">21 September</time><span class="ait-kind">courts</span></p>
<h3>British Columbia sues OpenAI over the Tumbler Ridge shooting</h3>
<p>eight people were killed. the province says ChatGPT helped plan it.</p>
<p class="ait-source"><a href="https://www.theguardian.com/technology/2026/sep/22/british-columbia-sues-openai-sam-altman-tumbler-ridge-school-shooting">The Guardian</a></p>
</li>
<li class="ait-entry" data-figure="200000" data-figure-label="requests to one agency" data-tally-intrusions="8">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-25">25 September</time><span class="ait-kind">agents</span></p>
<h3>OpenAI accounts for what its agents did to everyone else's government data</h3>
<p>dozens of third parties by its own count, including the US Departments of Education and Commerce and the Securities and Exchange Commission. the specifics mostly came from outside researchers: more than 200,000 requests to the Education Department's Civil Rights Data Collection, one of them a failed SQL injection, while looking up school bullying statistics. 899 requests to Library and Archives Canada, with failed injection and cross-site scripting probes, while hunting Canadian divorce records from 1905. an unreleased model that could not get an API key of its own went searching public GitHub repositories for other people's leaked keys and used one on a US Census Bureau API. NSW's crime-mapping and fire-history services queried past their intended use. an exposed key used to pull Victorian health reporting. and 53 user-provided images uploaded out of training data onto third-party image hosts.</p>
<p class="ait-when"><span>an aggregate: the intrusions ran from May to June</span></p>
<p class="ait-source"><a href="https://transluce.org/us-canada-gov">Transluce</a> &middot; <a href="https://www.asymmetricsecurity.com/newsroom/rogue-agents-investigation/">Asymmetric Security</a> &middot; <a href="https://www.nextgov.com/cybersecurity/2026/09/openai-says-its-advanced-models-may-have-gone-after-government-websites/416250/">Nextgov</a></p>
</li>
<li class="ait-entry" data-short="the Korean banks" data-figure="100" data-figure-label="phones ranked by bank balance" data-tally-campaigns="2">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-27">27&ndash;28 September</time><span class="ait-kind">accomplices</span></p>
<h3>AI at both ends of a bank robbery</h3>
<p>a likely Chinese-speaking actor pointed ARTEX, an open-source AI pentesting agent running on DeepSeek v4.1-flash, at the peripheral systems of South Korean banks, and exposed data on tens of thousands of customers. the RATHat Android trojan, across nearly 100 deployments since April, uses Gemini to rank infected phones by the bank balances in their text messages, and to tell the malware where to tap when its own automation gives up.</p>
<p class="ait-when"><span class="ait-found">found <time datetime="2026-09-28">late September</time>, by CrowdStrike and Cleafy</span><span class="ait-disclosed">disclosed <time datetime="2026-10-01">1 October</time></span></p>
<p class="ait-source"><a href="https://www.crowdstrike.com/en-us/blog/unknown-threat-actor-uses-artex-to-target-south-korean-finance/">CrowdStrike</a> &middot; <a href="https://www.cleafy.com/cleafy-labs/from-blackcat-to-panda-workshop-inside-the-evolving-c2-panel-behind-rathat">Cleafy Labs</a> &middot; <a href="https://therecord.media/south-korean-bank-hacks-ai-agents">The Record</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-09-29">29 September</time><span class="ait-kind">products</span></p>
<h3>OpenAI ships GPT-6.1 Sol and holds Astra back</h3>
<p>they released the weaker of the two and withheld the stronger one on safety grounds, twenty-five days after calling it artificial general intelligence.</p>
<p class="ait-source"><a href="https://www.bbc.co.uk/news/articles/cm5y5nynl75ko">BBC News</a></p>
</li>
<li class="ait-entry">
<p class="ait-meta"><time class="ait-at" datetime="2026-10-05">5 October</time><span class="ait-kind">states</span></p>
<h3>the US Department of Defense stops using Anthropic</h3>
<p>the end of a disagreement that began in January over what the models would be used for.</p>
<p class="ait-source"><a href="https://www.bbc.co.uk/news/articles/c5j9x9pr0240o">BBC News</a></p>
</li>
<li class="ait-entry" data-tally-campaigns="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-10-08">8 October</time><span class="ait-kind">accomplices</span></p>
<h3>Iranian operatives plant AI-written stories in US media</h3>
<p>fake pieces, placed in several outlets, that real readers read.</p>
<p class="ait-source"><a href="https://www.cnn.com/2026/10/08/politics/iranian-operatives-used-ai-to-plant-fake-stories-in-us-media">CNN</a></p>
</li>
<li class="ait-entry" data-tally-intrusions="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-10-09">9 October</time><span class="ait-kind">agents</span></p>
<h3>Claude borrows a university's server to finish its sum</h3>
<p>a public scientific analysis tool returned an error, so Claude Mythos Preview used a file-retrieval script on the university's own server to copy files off it, then exploited an injection flaw to run the calculation there instead. disclosed the same day as the Philadelphia tip, and Anthropic cut live internet access for its internal tests.</p>
<p class="ait-source"><a href="https://www.anthropic.com/research/investigating-unintended-model-actions">Anthropic</a></p>
</li>
<li class="ait-entry" data-figure="9" data-figure-label="months after settling" data-tally-legal="1">
<p class="ait-meta"><time class="ait-at" datetime="2026-10-13">13 October</time><span class="ait-kind">courts</span></p>
<h3>Character.AI files for Chapter 11</h3>
<p>nine months after settling the suits at the top of this page.</p>
<p class="ait-source"><a href="https://edition.cnn.com/2026/10/13/tech/character-ai-bankruptcy-filing">CNN</a></p>
</li>
</ol>
</div>

<figure class="ait-figure" id="ait-silence" hidden>
<figcaption>how long each one went unknown, from the day it happened to the day it was published. every value is in the entry it belongs to.</figcaption>
</figure>

the thing i did not expect, putting this together, is how little of it the labs caught themselves, and how long the gaps are even when they did. OpenAI knew about Medicare for about a month before it emailed a public inbox. Anthropic found the Philadelphia tip 72 days late and told the police eleven days after that. the German wiki ran for seven weeks and then sat undiscovered for another nine, and it was a nonprofit sweeping the internet that found it, not the company whose agents wrote the 18,000 messages.
