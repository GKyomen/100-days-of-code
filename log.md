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
