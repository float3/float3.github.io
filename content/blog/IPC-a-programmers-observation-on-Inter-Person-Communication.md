---
title: "IPC - a programmers observations on inter person communication"
tags:
  - programming
---

<style>
  .error { color: red; }
  .note { color: green; }
  .evidence { color: purple; }
  .grey { color: grey; }
  .orange { color: orange; }
</style>

this blogpost was partially inspired by <https://lajili.com/posts/post-1/>

## IPC

### type mismatches

consider the following conversation

Q: "does god exist?" \
a: "in my opinion, the existence of a deity or deities is not supported by scientific evidence or reasoning, which guides my understanding of the universe and our place in it."

<pre class="compact-pre">
<span class="grey">   | </span>
<span class="grey">2  | </span><!--                    -->answer(Opinion("the existence of ..."));
<span class="grey">   | -------</span><span class="error">^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `Opinion&lt;String>`</span>
<span class="grey">   | |</span>
<span class="grey">   | arguments to this function are incorrect</span>
<span class="grey">   | </span>
<span class="grey">   = note:</span> expected type `String`
<span class="grey">              <!---->found enum </span>`Opinion&lt;String>`
</pre>

this is a type mismatch, the Asker was seeking a statement of fact.
thankfully our interpreter can do context dependent implicit conversion.

<!-- add answering in binary/boolean to an answer that expects an enumerator, "you would think the order of the enum members is determined by the order they are said in so when binary is casted to the enum false would stand for 0 and true for 1"-->

here are a couple of other examples and what types they would expect,

<pre><code>
|----------------------------------------------|-------------------------|-----------------------|
| Question                                       | Expected Type               | Available Conversions   |
| ---------------------------------------------- | --------------------------- | ----------------------- |
| "do you still want this, or can I eat this?"   | `Tuple&lt;Boolean,Boolean>` | `Boolean`               |
| "do you want A, B or C"                        | `Enum(A,B,C)`               | `Integer`               |
| "A or B"                                       | `Enum(A,B)`                 | `Boolean, Integer`      |
| ---------------------------------------------- | -------------------------   | ----------------------- |
</code></pre>

### value/sign mismatch

another source of misunderstanding is the use of double negatives.

sometimes when we ask a question we get an answer in the form of a `Tuple<Boolean,String>`
where examining either the `Boolean` or the `String` will result in different conclusions

a common example would be:

Q: "you don't want desert right?"
A: "no, i don't."

"no i don't." is of type `Tuple<Boolean,String>` specifically `Tuple<False, "I don't">`.
the boolean `False` could initially suggest agreement with the statement (implying they do want dessert),
but the string "i don't" actually affirms the initial question's negative phrasing,
indicating they do not want dessert. this creates a situation where the `Boolean` and `String`
components of the answer seem to contradict each other if taken at face value without considering
the context of double negatives. in cases like this the `String` usually overwrites the `Boolean`

### type aliasing and conflicting definitions

when having a conversation, two parties might have conflicting definitions
of concepts, similar to a "dependency conflict" in software development. this
issue is akin to a linker error that arises when your project's dependencies
require two different versions of a library, each containing conflicting
definitions.

<pre class="compact-pre">
<span class="error">error[E0499]</span>: version conflict for 'definitions' dependency
<span class="grey">  --> src/conversation.ipc:1:5</span>
<span class="grey">   | </span>
<span class="grey">1  | </span><!--             -->use definitions::RATIONALITY;
<span class="grey">   | </span>    <span class="error">^^^^^^^^^^^^^^^^^^^^^^^^</span>
<span class="grey">   | </span>
<span class="grey">   = note:</span> the current application depends on 'definitions' version <span class="note">15.3.8 (economics)</span> and <span class="orange">13.7.3 (philosophy)</span>
<span class="grey">   = note:</span> Answerer's 'definitions' version <span class="note">15.3.8 (economics)</span> defines RATIONALITY as "Making decisions based on maximizing utility or benefit within constraints."
<span class="grey">   = note:</span> Asker's 'definitions' version <span class="orange">13.7.3 (philosophy)</span> defines RATIONALITY as "Being reasonable, coherent, and logical in thinking and decision-making, beyond just self-interest"
<span class="grey">   = note:</span> consider calling "<span class="evidence">evidence_sharing()</span>" or "<span class="evidence">agree_to_disagree()</span>" or explicitly specifying which version you are referencing to resolve the conflict
</pre>

sometimes it's acceptable to fail silently when encountering conflicting definitions in a conversation, like in cases where:
(a) the definition in question is not central to the information being communicated.
(B) the definitions are close enough that they still allow for effective communication. <!--(C) Contextual understanding is sufficient for the purposes of the conversation. (D) The discussion is non-critical, and precise definitions are not crucial. (E) The conversation involves conceptual brainstorming, where flexibility in definitions can be beneficial. (F) Cultural or idiomatic expressions are being used, where meaning is derived from context.-->

however, in all other scenarios, it may be necessary to engage in evidence sharing to resolve these conflicts.

<!--Rational action if you're using the word correctly means the best action

### Evidence sharing-->
