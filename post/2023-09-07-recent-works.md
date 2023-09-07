---
topic: System Design and Implementation
tags:
  - P1-spectrum
  - P2-ethgame
  - P3-nest
  - P4-tenspr
  - P5-approxformer
abstr: Things that I'm recently working on.
title: Recent Works (2023/09/07)
date: 2023-09-07
---

# Recent Projects

## P1-Spectrum

Spectrum is an ECNU blockchain lab project exploring how existing parallel protocols perform on permissioned blockchain and trying to boost the performance by ad-hoc optimizations on these protocols. This project is led by [Zhihao Cheng](https://github.com/jacklightChen), a Ph.D. candidate at ECNU, and is advised by [Zhao Zhang](https://ieeexplore.ieee.org/author/37086856133), a professor at ECNU. 

I contributed to this project by proposing and implementing persistent memory and the concept of partial rollback, which speeds up parallel execution by cutting down rollback overhead in speculative execution. 

The original codebase in C++ is not available right now, but I'm rewriting the codebase of this project in Rust to serve a broader range of educational and academic purposes. 

[Repository (Rust Rewriting)](https://github.com/Y-jiji/db-playground)

## P2-EthGame
EthGame is a project trying to fix security bugs in Ethereum using game theory. 

The current solution to vulnerability bugs is based on SMT solvers. This is good at detecting and understanding bugs, but in this project, I'm trying to generate rewriting suggestions directly applicable to the current smart contract. 

This project is now advised by [Youzhi Zhang](https://youzhi333.github.io/). 

[Repository](https://github.com/Y-jiji/ethgame)

## P3-NeSt

NeSt is a weird abbreviation for Network Security Game. It is a game on a graph structure where an attacker tries to reach some given nodes and a defender with multiple movable units tries to catch them before the attacker succeeds. 

There is a series of papers on this topic. In June 2023, Youzhi Zhang decided to implement an open-source environment for this game and put all current solutions into a unified codebase. These tasks were assigned to interns and RAs in July. I'm currently the leading developer. 

[Repository (Inaccessible before finishing)](github.com/lcskxj/PursuitEvasionPlantform)

## P4-Tenspr

Tenspr is a project trying to automatically search suitable sparse tensor formats for a computation graph.

I'm working with [Egor Larionov](https://github.com/elrnv) from Meta Reality Labs on this as a side project. 

Currently, I'm working on designing and implementing an intermediate representation for describing sparse tensor formats and modeling the cost dynamics. 

This work is inspired by [TACO](https://github.com/tensor-compiler/taco), [Apache TVM,](https://github.com/apache/tvm) and [TC-GNN](https://github.com/YukeWang96/TC-GNN_ATC23). 

```
Tenspr = {Hardware-Aware TACO} = {Sparse-Version-Of TVM} = {Automated TC-GNN}
```

[Repository](https://github.com/Y-jiji/tenspr)

## P5-ApproxFormer

ApproxFormer is a kind of new masked attention that trains like a Transformer and infers like RNN. 

The basic idea is to decompose the diagonal matrix $M$ to $A$ and $B$ so that we can add masks to linear transformers. 
$$
((\phi(Q)\times \phi(K^T))\odot M)\times V \approx (\phi(Q)\odot A)\times(\phi(K^T)\odot B)\times V
$$
This work is inspired by [TC-GNN](https://github.com/YukeWang96/TC-GNN_ATC23) and [RWKV](https://wiki.rwkv.com/) in two different ways. 

It can be seen as a numerical version of TC-GNN by compressing the mask matrix $M$ (analogous to weight matrix for weighted graphs) into $A\times B$ , where $B$ is a condensing matrix pushing row windows into one and $A$ is a recovering matrix pulling out row windows into original matrix. 

It can also be seen as a successor of RWKV, using a different approach to achieve the inference-time RNN performance. 

[Repository](https://github.com/Y-jiji/approxformer)

