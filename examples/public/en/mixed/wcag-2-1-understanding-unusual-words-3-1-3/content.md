---
source_url: https://www.w3.org/WAI/WCAG21/Understanding/unusual-words.html
title: WCAG 2.1 Understanding — Unusual Words (3.1.3)
upstream_type: standard
polarity: mixed_unaligned
languages:
- en
redistribution: public_ok
license: W3C Document Licence.
rules_relevant:
- lexicon.jargon-undefined
- lexicon.weasel-words
conditions:
- aphasia
- non-native
- general
fetched_at: '2026-04-22T21:39:11+00:00'
markdownable: 4
---

# Understanding SC 3.1.3 Unusual Words (Level AAA)

## In Brief

- Goal
- Users can identify and learn what unusual words mean.
- What to do
- Provide definitions for technical jargon and unusual terms.
- Why it's important
- More people, especially those with cognitive disabilities, can understand the meaning of content.

## Success Criterion (SC)

A mechanism is available for identifying specific definitions of words or phrases used in an unusual or restricted way, including idioms and jargon.

## Intent

Certain disabilities make it difficult to understand non-literal word usage and specialized words or usage. Certain disabilities make it difficult to understand figurative language or specialized usage. Providing such mechanisms is vital for these audiences. Specialized information intended for non-specialist readers is encouraged to satisfy this Success Criterion, even when claiming only Single-A or Double-A conformance.

## Benefits

This success criterion may help people with cognitive, language and learning disabilities who:

- have difficulty decoding words
- have difficulty understanding words and phrases
- have difficulty using context to aid understanding

It would also help people with visual disabilities who:

- lose context when zoomed-in with a screen magnifier

## Examples

- Text that includes a definition for a word used in an unusual way
- Organize the list or "cascade" of dictionaries and other resources so that the definition search will find the intended definitions instead of displaying definitions from other sources in the "cascade." (The "cascade" lists the dictionaries and other reference materials in the order most likely to bring up the right definition. This controls the order to follow when searching for definitions.)
- Including definitions in the glossary
- WCAG 2 uses the word "text" in a specific way. Thus, when the word "text" is used within WCAG 2 it is linked to the definition of "text" provided in a glossary within the same web page.
- The specific definition of a word is provided at the bottom of the page
- The internal link from the word to the corresponding definition is also provided within the page.

## Related Resources

Resources are for information purposes only, no endorsement implied.

Note

The inclusion of a product or vendor name in the list below does not constitute an endorsement by the Accessibility Guidelines Working Group or the Web Accessibility Initiative of the World Wide Web Consortium. This list is provided simply for convenience and to give users an idea of what resources may be available

- Free bilingual dictionaries for a number of languages are available from the Freedict.org website. The dictionaries are of uneven quality and size as noted on the site. Retrieved 9 April 2005.
- The WorldStar Free Dictionaries, Translators and Search Engines site provides access to free on-line dictionaries and search engines in many languages. Retrieved 18 November 2005.
- More dictionaries are at your dictionary, freelang.com (in English, Spanish and French!) and many other places.

## Techniques

Each numbered item in this section represents a technique or combination of techniques that the Accessibility Guidelines Working Group deems sufficient for meeting this success criterion. A technique may go beyond the minimum requirement of the criterion. There may be other ways of meeting the criterion not covered by these techniques. For information on using other techniques, see Understanding Techniques for WCAG Success Criteria, particularly the "Other Techniques" section.

### Sufficient Techniques

Select the situation below that matches your content. Each situation includes techniques or combinations of techniques that are known and documented to be sufficient for that situation.

#### Situation A: If the word or phrase has a unique meaning within the web page:

- G101: Providing the definition of a word or phrase used in an unusual or restricted way for the first occurrence of the word or phrase in a web page using one of the following techniques:

- G101: Providing the definition of a word or phrase used in an unusual or restricted way for each occurrence of the word or phrase in a web page using one of the following techniques:

#### Situation B: If the word or phrase means different things within the same web page:

- G101: Providing the definition of a word or phrase used in an unusual or restricted way for each occurrence of the word or phrase in a web page using one of the following techniques:

### 

## Key Terms

- assistive technology

- hardware and/or software that acts as a user agent, or along with a mainstream user agent, to provide functionality to meet the requirements of users with disabilities that go beyond those offered by mainstream user agents

  Note 1

  functionality provided by assistive technology includes alternative presentations (e.g., as synthesized speech or magnified content), alternative input methods (e.g., voice), additional navigation or orientation mechanisms, and content transformations (e.g., to make tables more accessible).

  Note 2

  Assistive technologies often communicate data and messages with mainstream user agents by using and monitoring APIs.

  Note 3

  The distinction between mainstream user agents and assistive technologies is not absolute. Many mainstream user agents provide some features to assist individuals with disabilities. The basic difference is that mainstream user agents target broad and diverse audiences that usually include people with and without disabilities. Assistive technologies target narrowly defined populations of users with specific disabilities. The assistance provided by an assistive technology is more specific and appropriate to the needs of its target users. The mainstream user agent may provide important functionality to assistive technologies like retrieving web content from program objects or parsing markup into identifiable bundles.

  - screen magnifiers, and other visual reading assistants, which are used by people with visual, perceptual and physical print disabilities to change text font, size, spacing, color, synchronization with speech, etc. in order to improve the visual readability of rendered text and images;
  - screen readers, which are used by people who are blind to read textual information through synthesized speech or braille;
  - text-to-speech software, which is used by some people with cognitive, language, and learning disabilities to convert text into synthetic speech;
  - speech recognition software, which may be used by people who have some physical disabilities;
  - alternative keyboards, which are used by people with certain physical disabilities to simulate the keyboard (including alternate keyboards that use head pointers, single switches, sip/puff and other special input devices.);
  - alternative pointing devices, which are used by people with certain physical disabilities to simulate mouse pointing and button activations.

- conformance

- satisfying all the requirements of a given standard, guideline or specification

- idiom

- phrase whose meaning cannot be deduced from the meaning of the individual words and the specific words cannot be changed without losing the meaning

  Note

  idioms cannot be translated directly, word for word, without losing their (cultural or language-dependent) meaning.

- jargon

- words used in a particular way by people in a particular field

- mechanism

- process or technique for achieving a result

  Note 1

  The mechanism may be explicitly provided in the content, or may be relied upon to be provided by either the platform or by user agents, including assistive technologies.

  Note 2

  The mechanism needs to meet all success criteria for the conformance level claimed.

- process

- series of user actions where each action is required in order to complete an activity

- relied upon

- the content would not conform if that technology is turned off or is not supported

- technology

- mechanism for encoding instructions to be rendered, played or executed by user agents

  Note 1

  As used in these guidelines "web technology" and the word "technology" (when used alone) both refer to web content technologies.

  Note 2

  Web content technologies may include markup languages, data formats, or programming languages that authors may use alone or in combination to create end-user experiences that range from static web pages to synchronized media presentations to dynamic Web applications.

- used in an unusual or restricted way

- words used in such a way that requires users to know exactly which definition to apply in order to understand the content correctly

- user agent

- any software that retrieves and presents web content for users
