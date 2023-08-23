
Rust-hosted programming languages

answer:
Is Rust viable?
Is it good?
Better than C, C++, something managed (JS/TS, C#, Go, Java, etc.)


assume no familiarity with compiler implementation - build-your-own programming language tutorial, crash-course style

- Pipeline
  - Source code
  - Syntax (parsing)
  - Semantics (type checking, borrow checking)
  - Code generation

Parsing
- Lexer/Tokenizer
- Parser
  - It's totally possible to skip constructing an AST,
    but it simplifies latter steps
  - You can also do tokenization lazily, or skip it altogether

Type checking
- Type systems are syntax-directed
  - The type of terms (nodes in the AST) can be inferred
    just from their syntax alone, together with a type
    environment to keep track of the types of variables
  - Super basic examples of HM (binary +)
- Dynamically typed languages would skip this step.

Additional passes
- AST Lowering/Normalization/Simplification
  - Makes it easier to reason about the program
- Other forms of analysis, e.g. borrow checking

After various forms of analysis (or none) and transformations, it's time to output _something_.
- Code for a target language (transpilers, e.g. TypeScript)
- Machine code ("compilers", Rust, C, etc.)
- Bytecode ("interpreters", JS, Lua, Python, etc.)

What is the common theme between all stages of the compilation pipeline? **Pattern matching**

Because pattern matching is so prevalent in compilers, any language which is good at this task will naturally also be great for writing compilers. It's really common to see the "shiny" (as in, push the state of the art) languages being implemented in OCaml or Haskell for this reason. But, as we all know, Rust can do it too! Just like OCaml and Haskell, Rust is part of the ML family, and it is _amazing_ at pattern matching.

While functional languages _are_ great for writing compilers, Rust has something that they don't, which is total control over memory layout, alllocation, and access patterns. It's a league above the others in terms of how _fast_ you can make the compiler run.

Let's write a basic interpreter in Rust. We'll implement a calculator for arithmetic expressions, and we'll only support integers.

Here's our pipeline:
- Lazy lex + parse into AST
- Emit bytecode

```rust

```
