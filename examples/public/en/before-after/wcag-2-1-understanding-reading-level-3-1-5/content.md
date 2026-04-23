---
source_url: https://www.w3.org/WAI/WCAG21/Understanding/reading-level.html
title: WCAG 2.1 Understanding — Reading Level (3.1.5)
upstream_type: standard
polarity: before_after
languages:
- en
redistribution: public_ok
license: W3C Document Licence — copying permitted with notice preservation; derivative
  work subject to conditions.
rules_relevant:
- readability.score
- lexicon.jargon-undefined
- structure.sentence-too-long
conditions:
- general
- non-native
- aphasia
fetched_at: '2026-04-22T21:39:10+00:00'
markdownable: 4
---

# Understanding SC 3.1.5 Reading Level (Level AAA)

## In Brief

- Goal
- Users can get a simplified version of complex information.
- What to do
- When text information becomes complex, create a more easily understood version.
- Why it's important
- More people, especially those with cognitive disabilities, can understand the meaning of content.

## Success Criterion (SC)

When text requires reading ability more advanced than the lower secondary education level after removal of proper names and titles, supplemental content, or a version that does not require reading ability more advanced than the lower secondary education level, is available.

## Intent

Content should be written as clearly and simply as possible. The intent of this Success Criterion is:

- to ensure that additional content is available to aid the understanding of difficult or complex text;
- to establish a testable measure indicating when such additional content is required.

This success criterion helps people with reading disabilities while also allowing authors to publish difficult or complex web content. Text difficulty is described in terms of the level of education required to read the text. Education levels are defined according to the International Standard Classification of Education \[UNESCO\], which was created to allow international comparison among systems of education.

Difficult or complex text may be appropriate for most members of the intended audience (that is, most of the people for whom the content has been created). But there are people with disabilities, including reading disabilities, even among highly educated users with specialized knowledge of the subject matter. It may be possible to accommodate these users by making the text more readable. If the text cannot be made more readable, then supplemental content is needed. Supplemental content is required when text demands reading ability more advanced than the lower secondary education level—that is, more than nine years of school. Such text presents severe obstacles to people with reading disabilities and is considered difficult even for people without disabilities who have completed upper secondary education.

Reading disabilities such as dyslexia make it difficult to recognize written or printed words and associate them with the correct sounds. This is called "decoding" the text. Decoding must be automatic in order for people to read fluently. The act of decoding text word by word consumes much of the mental energy that most people are able to use for understanding what they read. Text that uses short, common words and short sentences is easier to decode and usually requires less advanced reading ability than text that uses long sentences and long or unfamiliar words.

The education level required to read text content (also called "readability") is measured by analyzing selected passages of text from the web page. If the web page includes text written for different purposes or different styles are used, the selected passages include samples of the types of content in the web page and the different styles in which the content is written. (In many cases, the web page contains only one kind of text content—e.g., technical documentation, a legal notice, marketing material, etc.—and all the content uses the same style.)

Educators can also measure the education level required to read text content. For example, qualified teachers can evaluate text according to local education standards to determine if it requires reading ability beyond what is expected for students in the last year of lower secondary education.

Because people's names, the names of cities or other proper names cannot be changed to shorter names with fewer syllables, and because doing so or just referring to everyone by pronouns can make sentences harder to understand, the success criterion specifies that proper names can be ignored or removed from the text before assessing whether it meets the reading ability requirement. Titles refer to the name of documents, books, movies, etc. Titles are removed or ignored for the analysis because changing the words in titles might make the titles easier to read but would make it impossible to understand the item to which the title refers. This would make it harder to read and understand the content. (e.g., a book, academic paper, article, etc.). Therefore, titles are also exempted specifically.

When a web page contains multiple languages, a readability result should be calculated for each language that constitutes at least 5% of the content and that is used in full sentences or paragraphs (not just individual words or phrases). The overall readability of the page should be judged on the language that yields the worst readability results.

The readability of content may also be determined by applying a readability formula to the selected passage. Many (though not all) readability formulas base their calculations on passages of 100 words. Such formulas have been developed for many languages. The number of words in the passage is counted and the length of the words is determined by counting either the number of syllables or the number of characters. Most readability formulas also count the number and length of sentences. The average length of words and sentences in the content is then used to calculate a readability score. (Some languages, such as Japanese, may include multiple scripts within the same passage. Readability formulas for these languages may use the number and length of such "runs" in their calculations.) The result may be presented as a number (for example, on a scale from 0-100) or as a grade level. These results can then be interpreted using the education levels described in the International Standard Classification of Education. Readability formulas are available for at least some languages when running the spell checkers in popular software if you specify in the options of this engine that you want to have the statistics when it has finished checking your documents.

### Levels of education

- Primary education
- First 6 years of school
- Lower secondary education
- 7-9 years of school
- Upper secondary education
- 10-12 years of school
- Advanced education
- More than 12 years of school

Adapted from International Standard Classification of Education (UNESCO).

Note

According to the Open Society Mental Health Initiative, the concept of Easy to Read cannot be universal, and it will not be possible to write a text that will suit the abilities of all people with literacy and comprehension problems. Using the clearest and simplest language appropriate is highly desirable, but the Accessibility Guidelines Working Group could not find a way to test whether this had been achieved. The use of reading level is a way to introduce testability into a success criterion that encourages clear writing. Supplementary content can be a powerful technique for people with some classes of cognitive disability.

## Benefits

This success criterion may help people who:

- Have difficulty comprehending and interpreting written language (e.g., articles, instructions, or newspapers in text or braille), for the purpose of obtaining general knowledge or specific information

## Examples

- A scientific journal including readable summaries of complex research articles

- A scientific journal includes articles written in highly technical language aimed at specialists in the field. The journal's Table of Contents page includes a plain-language summary of each article. The summaries are intended for a general audience with eight years of school. The metadata for the journal uses the Dublin Core specification to identify the education level of the articles' intended audience as "advanced," and the education level of the intended audience for the summaries as "lower secondary education."

- Medical information for members of the public

- A medical school operates a website that explains recent medical and scientific discoveries. The articles on the site are written for people without medical training. Each article uses the Dublin Core metadata specification to identify the education level of the intended audience as "lower secondary education" and includes the Flesch Reading Ease score for the article. A link on each page displays the education level and other metadata. No supplemental content is required because people who read at the lower secondary education level can read the articles.

- An e-learning application

- An on-line course about Spanish cultural history includes a unit on Moorish architecture. The unit includes text written for students with different reading abilities. Photographs and drawings of buildings illustrate architectural concepts and styles. Graphic organizers are used to illustrate complex relationships, and an audio version using synthetic speech is available. The metadata for each version describes the academic level of the content and includes a readability score based on formulas developed for Spanish-language text. The learning application uses this metadata and metadata about the students to provide versions of instructional content that match the needs of individual students.

- Science information that requires a reading ability at the lower secondary education level

- The text below (116 words total) requires a reading ability of grade 4.2 in the United States according to the Flesch-Kincaid formula. In the US, grade 4.2 is at the primary education level.

  Science is about testing — and about looking closely. Some scientists use microscopes to take a close look. We're going to use a simple piece of paper.

  Here's what you do: Print this page and cut out the square to make a "window" one inch square. (It's easiest to fold the page in half before you cut.)

  Choose something interesting: a tree trunk, a leaf, flower, the soil surface, or a slice of soil from a shovel.

  Put your window over the thing and look at it closely. Take your time — this is not a race.

  To help you see more details, draw a picture of what's inside your square.

  Now let's think about what you've found.

  (Source: Howard Hughes Medical Institute "CoolScience for Kids" Project)

## Related Resources

Resources are for information purposes only, no endorsement implied.

- The Plain Language Association International (PLAIN) website provides many useful resources to help writers produce documents that communicate clearly in a variety of cultural and rhetorical contexts.
- The US Government's plain language website provides general information about plain language as well as information about use of plain language in US Government documents, including legal requirements
- The Plain English Campaign website provides useful information and guidance for authors writing in English.
- Juicy Studio's Readability Test analyzes the readability of all rendered content.
- Entry for Audience Education Level. Using Dublin Core – Dublin Core Qualifiers
- IMS Learner Information Packaging Model Information Specification, Table 6.3 "accessibility" learner information data structure detailed description
- Leesbaar Nederlands ("Readable Dutch") contains guidelines for writing text that is accessible for people with a reading disability. These guidelines address language, content and design.
- European Easy-to-Read Guidelines
- Flesch-Kincaid Readability Test

## Techniques

Each numbered item in this section represents a technique or combination of techniques that the Accessibility Guidelines Working Group deems sufficient for meeting this success criterion. A technique may go beyond the minimum requirement of the criterion. There may be other ways of meeting the criterion not covered by these techniques. For information on using other techniques, see Understanding Techniques for WCAG Success Criteria, particularly the "Other Techniques" section.

### Sufficient Techniques

- G86: Providing a text summary that can be understood by people with lower secondary education level reading ability
- G103: Providing visual illustrations, pictures, and symbols to help explain ideas, events, and processes
- G79: Providing a spoken version of the text
- G153: Making the text easier to read
- G160: Providing sign language versions of information, ideas, and processes that must be understood in order to use the content

Note

Different sites may address this success criterion in different ways. An audio version of the content may be helpful to some users. For some people who are deaf, a sign language version of the page may be easier to understand than a written language version since sign language may be their first language. Some sites may decide to do both or other combinations. No technique will help all users who have difficulty. So different techniques are provided as sufficient techniques here for authors trying to make their sites more accessible. Any numbered technique or combination above can be used by a particular site and it is considered sufficient by the Working Group.

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

- content

- information and sensory experience to be communicated to the user by means of a user agent, including code or markup that defines the content's structure, presentation, and interactions

- lower secondary education level

- the two or three year period of education that begins after completion of six years of school and ends nine years after the beginning of primary education

  Note

  This definition is based on the International Standard Classification of Education \[UNESCO\].

- presentation

- rendering of the content in a form to be perceived by users

- primary education level

- six year time period that begins between the ages of five and seven, possibly without any previous education

  Note

  This definition is based on the International Standard Classification of Education \[UNESCO\].

- process

- series of user actions where each action is required in order to complete an activity

- structure

- 

- supplemental content

- additional content that illustrates or clarifies the primary content

- user agent

- any software that retrieves and presents web content for users

- web page

- a non-embedded resource obtained from a single URI using HTTP plus any other resources that are used in the rendering or intended to be rendered together with it by a user agent

  Note 1

  Although any "other resources" would be rendered together with the primary resource, they would not necessarily be rendered simultaneously with each other.

  Note 2

  For the purposes of conformance with these guidelines, a resource must be "non-embedded" within the scope of conformance to be considered a web page.
