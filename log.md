# 100 Days Of Code - Log

### Day 1: February 26, 2024

**Today's Progress**: Progressed through the book up until chapter 2

**Thoughts:**

- This book is really good in maintaining the reader focus. Also, Rust was made on top of a great challenge: balancing developer experience and performance, and I really appreciate that effort!

- I enjoyed doing the game and now I'm thinking about coding some sorting algorithms to practice today's learning.

**Link to work:**

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Guessing game](/day1/guessing_game/)

### Day 2: February 27, 2024

**Today's Progress**: Progressed through the book up until chapter 3

**Thoughts:**

- Rust continues to show its care to safety through the book after showing how it implements common programming languages concepts, like the compiler error when possibly accessing an out of bounds array position. I like that!

- This chapter ends with some interesting suggestions about real projects to implement. I'll do it tomorrow.

**Link to work:**

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's learnings](/day2/)

### Day 3: February 28, 2024

**Today's Progress**: Made a fibonacci calculator and a temperature converter.

**Thoughts:**

- Today I've put my learnings to the test. It was good to practice the common concepts, like loops, match expressions and the difference between statements and expressions.

- Rust continues to impress me about code safety, but I still wish a language that can infer types over numbers during compile time, which is not present either in Rust or Typescript.

**Link to work:**

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Fibonacci calculator](/day3/fibonacci/)
3. [Temperature converter](/day3/temperature_converter/)

### Day 4: February 29, 2024

**Today's Progress**: Progressed through the book up until chapter 4

**Thoughts:**

- Today I've learned about Rust most talked concept: Ownership. For someone who already has experienced pointers in C, the book shows pretty well why they took this idea to the language project.

- At the same time, this was the most difficult chapter to read up until now. There is a ton to learn, and I'm certain that it will only be possible through hands-on programming.

- One other thing that impresses me is how this concept forces the developers not only to look to possible bugs (undefined behavior) but also to look at logical errors, like how a function to append a title to a person's name (i.e PhD) could be fixed in three different manners, but only one was correct because it didn't changed the original person's name forever, which would probably be bad in the program context.

**Link to work:**

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day4/ownership/)

### Day 5: March 3, 2024

**Today's Progress**: Progressed through the book up until chapter 5

**Thoughts:**

- This chapter was a bit easier to understand as I saw structs in C and OOP in Java before.

- The book continues to show how important is to understand ownership to develop the best solutions in Rust. I'm planning to do a project to study it more deeply.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day5/rectangles/)

### Day 6: March 4, 2024

**Today's Progress**: Progressed through the book up until chapter 6

**Thoughts:**

- This chapter was really easier to understand as enums exists in Typescript and match was used before in chapter 2. Because of that, little work has been done in code today

- As the book states, after seeing Rust "null" (None, to be more precise) and enums implementations, I wish it had been like this in all other languages.

- Ownership inventory is a really good way to test our learnings on ownership, but it crashed my browser twice. Note to self: close some tabs before trying this kind of quiz.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day6/enums/)

### Day 7: March 5, 2024

**Today's Progress**: Progressed through the book up until chapter 7

**Thoughts:**

- This chapter prooves once more the book care about the developers because it talks about the `mod` and `use` features in the context of developing large projects.

- Each day into reading the book has increased my understanding of the idiomatic way to think and write Rust code. I really need to make a hands on project to test my learning, but so far I think I'm understanding each concept after some chapters (i.e. understanding ownership better after chapters 5, 6 and 7)

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day7/restaurant/)

### Day 8: March 6, 2024

**Today's Progress**: Progressed through the book up until chapter 8

**Thoughts:**

- I really liked to learn about Rust String implementation and the space to interpretation it gives since Strings are really complicated in real world applications and Rust doesn't try to hide it like many other languages do it.

- I was also super excited to learn about hashmaps in Rust as I saw it too much time ago, in C, but only in a superficial way. I'm excited to try the book suggestions of challenges tomorrow

- Lastly, I'm really happy about Ownership inventory #2 as I did well in the test (starting with 4 out of 6 correctly answered questions) as opposed to the first and the original chapter quizzes (starting with most questions wrong and needing a lot of time to figure out why)

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day8/collections/)

### Day 9: March 7, 2024

**Today's Progress**: Made three challenges proposed by the book

**Thoughts:**

- Its so good to complete a challenge in Rust. Today I completed three: A calculator for mean, median and mode; A pig latin word converter; and a "database" for employees

- I'm excited to learn more and get back to this code and have that feeling of "I was a awful programmer in this language some time ago", which indicates that I'm getting experienced!

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's challenges](/day9/)

### Day 10: March 8, 2024

**Today's Progress**: Progressed through the book up until chapter 9

**Thoughts:**

- Today I learned about error handling, which is important for me because I will use Rust to implement APIs, so error handling is a core feature, and also because I'm currently studying automated testing, which uses errors too.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day10/)

### Day 11: March 10, 2024

**Today's Progress**: Progressed through the book up until chapter 10

**Thoughts:**

- This chapter taught me some important concepts to strong typed programming languages like Rust. Comparing to TypeScript, generics is almost the same, and Traits aren't really necessary in TS due to it's more general types (like everything in TypeScript has Rust `Display` trait), but it's super interesting how traits works!

- The different part was lifetimes, but it was easy to understand the reasons behind this feature and how to think when we need to implement it. Also, I'm glad that I'm learning Rust now since lifetimes was really boilerplate code before elision rules.

- There was another ownership inventory today. I'm 100% capable of finding the errors now, but I'm not answering correctly on how to fix them. I guess it will come over time. I don't think that those questions like "if this code wasn't rejected by the compiler, what code would cause errors?" are actually good... like, the code doesn't compile, so you can't run anything anyway. But I'll keep trying to be better.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day11/)

### Day 12: March 12, 2024

**Today's Progress**: Progressed through the book up until chapter 11

**Thoughts:**

- Yet again Rust prooves that it cares about its devs. This chapter shows that testing is native to the language, without the need to use a external crate to make it. I'm in love with this language.

- I was going to read two chapters today to fix the missed day yesterday, however the next one guides the reader through a project development, which in general takes a long time to complete and understand.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day12/)

### Day 13: March 13, 2024

**Today's Progress**: Progressed through the book up until chapter 12

**Thoughts:**

- This chapter brings A LOT of content to the table. It was incredible to develop the mini grep program in a real way using TDD, code refactoring and error handling.

- Also, this chapter not only summarizes most of the learnings so far, but also teaches how to work with environment variables and how to use the standard error. It was really well written.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's project -> minigrep](/day13/)

### Day 14: March 14, 2024

**Today's Progress**: Progressed through the book up until chapter 13

**Thoughts:**

- I was wondering about functional programming not so long ago as Elixir got some hype. I really thought it would be hard to understand functional paradigm, but Typescript had some features like closures. It was easier than I imagined and the last part of the chapter proved that Rust implementations are as good as it could be, and I'm impressed!

- Iterators was hard to understand in C, but my current experience and the Rust book explanation made it really easy to get. I'm really invested into it and can't wait to try it in a personal project.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day14/)

### Day 15: March 15, 2024

**Today's Progress**: Progressed through the book up until chapter 14 and made some exercises on LeetCode

**Thoughts:**

- Today's chapter was short, so I made exercises in LeetCode. I'm proud to been able to solve them!

- Nice chapter! Now I know how to use crates.io

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's exercises](/day15/)
3. [Link to exercise 1](https://leetcode.com/problems/contains-duplicate/)
4. [Link to exercise 2](https://leetcode.com/problems/valid-anagram/)
5. [Link to exercise 3](https://leetcode.com/problems/two-sum/)

### Day 16: March 18, 2024

**Today's Progress**: Made some more exercises on LeetCode

**Thoughts:**

- Not my proudest solutions: had to copy some parts of solution for ex 1 as mine was not fast enough for 40 of the 150 cases, and the ex 2 had the best runtime but one of the worst memory uses. Anyway I'm happy to be able to solve the problems.

1. [Today's exercises](/day16/)
2. [Link to exercise 1](https://leetcode.com/problems/group-anagrams/)
3. [Link to exercise 2](https://leetcode.com/problems/top-k-frequent-elements/)

### Day 17: March 19, 2024

**Today's Progress**: Progressed through the book up until half of chapter 15

**Thoughts:**

- I decided to break this chapter read in half because is a large one and to fix the concepts more clearly.

- Smart pointers seemed to be hard at first glance, but they are incredibly useful in many situations. I'm eager to finish the chapter.

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day17/)

### Day 18: March 21, 2024

**Today's Progress**: Finished chapter 15

**Thoughts:**

- This chapter gave me some idea on how rust have the unsafe feature behind the scenes. It also explained very useful concepts, link reference counting and interior mutability.

- Had an idea to write graph algorithms with the new knowledge of smart and shared pointers.

- I have to be more disciplined in this challenge

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day18/)

### Day 19: March 26, 2024

**Today's Progress**: Progressed through the book up until chapter 16

**Thoughts:**

- I was a little afraid of concurrency in programming as I never used it before, but Rust showed it garantees that makes it so much easier than other languages.

- I'm excited about the last chapter's project, which will use the knowledge of this chapter

- I took some time off and now I'm feeling good again to make the challenge everyday as intended in the beginning!

1. [The Rust Programming Language Experiment Book](https://rust-book.cs.brown.edu/)
2. [Today's tests and learnings](/day19/)
