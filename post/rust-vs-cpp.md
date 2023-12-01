---
topic: Casual / Journal
tags:
  - Dev
abstr: Several things that make system engineering more pleasurable.
date: 2023-12-01
title: Why I prefer Rust over C++
---

# Why I prefer Rust over C++
#2023-12-01
## Tools for Abstraction
As a modern programming language, design patterns are abundant for C++, but they brought more trouble than merits. 
For example, developer as are allowed to cast from children classes to a base class. In C++, children classes are allowed to have additional attributes, but this actually makes the behaviour of the base class harder to understand. For Rust, these kind of problems don't exist. Analogous to C++, similar abstractions can available as trait objects.  However, no additional data is allowed, and all functions are marked as 'virtual' in C++ terms. Therefore, the memory in these objects are safe, and the compiler exactly knows how to release them. 
## Lifetime Managment
Rust language introduces the concept of explicit lifetime and borrow checks. 
The ownership concept is originally provided in C++ core guideline and the guideline supporting library (know as GSL). 
However, because there doesn't exist a compiler to enforce this guideline, developers can easily make mistakes. For example, taking the raw pointer out of gslowner, and move it elsewhere, or using a counted reference without multi-threading compactability in multiple threads. To this end, shit happens in C++ projects, and we need extra time to fix them. 
## F\*\*k You Namespace!
The most weird design in C++ is namespace. 
At the first place, namespace is defined to save the developers from naming clashes. 
However, it preserves the original C-flavour: you generally cannot tell where a function is implemented from its name and included files. 
For Rust, the module name is more canonical and provides more visual hints about its implementation. 
Moreover, the module name design leads to a better build system. We don't suffer from building and compiling projects any longer. 