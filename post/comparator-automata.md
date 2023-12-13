---
topic: Programming Languages and Theoretical Computers
tags:
  - Paper-Sharing
abstr: An practical method for comparing how system runs, attributing to Prof. Suguman Bansal
date: 2023-12-13
title: "Comparator Automaton : Part 1 🔥"
---

# Comparator Automaton : Part 1 🔥
#2023-12-13 #Paper-Sharing
## Introduction: modeling system verfication
This is a paper sharing blog about Prof. Suguman Bansal's [COMPARATOR AUTOMATA IN QUANTITATIVE VERIFICATION](https://arxiv.org/pdf/1812.06569.pdf) . 
Suppose we have a transition system $S$ and a linear temporal logic predicate $P$ on that system. A transition system $S$ consists of a set of states $V$ and transtions between states $E$, and a predicate $P$ consists of sets of valid paths in that transition system. 
Traditional formulation describes $S$ as a set of paths
$$
S:=\{\overline{x_1x_2\cdots x_{n+1}}:(\forall i\in (1\cdots n+1):x_{i}\in V) \wedge (\forall i\in (1\cdots n):(x_{i}\to x_{i+1})\in E)\}
$$
And also $P$ directly a set of paths
$$
P:=\{\overline{x_1x_2\cdots x_n}: \overline{x_1x_2\cdots x_n}\in P\}
$$
Therefore, we can state that the system is valid if $S \subset P$ , which means we cannot construct invalid paths from $S$ . 
Excitingly, in this paper, there is another formulation that unleashes more possibilities! We can consider an aggregate function ${\frak a}_{S}$ and a weight function ${\frak a}_{P}$ s.t. for a path $p$ , ${\frak a}_{S}(p)=\mathbb{1}_{p\in S}$ and ${\frak a}_{P}(p)=\mathbb{1}_{p\in P}$ . Determine $S\subset P$ means $\forall p\in V^{*}\diagdown \{\varepsilon\}: {\frak a}_{S}(p) \subset {\frak a}_{P}(p)$ , where $V^{*}$ is the Kleene star construction. (At least this is not known to me when I was reading this paper. ) Another construction introduced in this paper is comparator automaton. Without loss of generality, a comparator automaton is co-related to an aggregate function $\frak a$ . It take$s a pair of words(paths) $a$ and $b$ as input, and accepts them iff ${\frak a}(a)\le {\frak a}(b)$ . 
## Buchi automaton
A Buchi automaton is a finite (or pushdown) automaton $\frak A$ with an additional set $\mathcal{F}$ which is a subset of states. 
A Buchi automaton accepts an infinite word $w$ iff running $\frak A$ on input $w$ gives a state sequence $\overline{s}$ (or several state sequences, for NFA case) such that a state $s\in \mathcal{F}$ appears infinitely often in $\overline{s}$ . 
When we refer to Buchi automaton, we usually mean the NFA version. 
## Aggregate fn automaton
Let's write $\beta$-based representation of $\mathbb{R}$ as $\mathtt{Repr}_{\beta}$ . 
We call a Buchi automaton $\frak A$ with state space $\Sigma\times \mathtt{Repr}_{\beta}$ aggregate function automation iff $\forall A\in \Sigma^{\omega}:\exists ! x\in \mathbb{R}$ such that $(A,\mathtt{repr}_{\beta}(x))$ is accepted by $\frak A$ . 
There are other formulations of for this notion using parity automaton, but we don't present them here because I don't know what is a parity automaton. 
## Weighted automaton
We begin with a Buchi automaton $\frak A$ that accepts every input, which means $\mathcal{F}$ is just the set of states and for each state, there exists a transition associated with each symbol in alphabet. 
In weighted automaton, we additionally assign each transition $(start,input,end)$ a rational weight, denoted by $\gamma: V_{\frak A}\times E_{\frak A}\times V_{\frak A}\to \mathbb{Q}$ .  
Then, an aggergate function $f$ operates on the assigned weights $\rho=\overline{\rho_1\rho_2\cdots\rho_n}$ where $\rho_{i}=\gamma(start_i,input_i,end_i)$ , and we define the weight of an infinite sequence as
$$
f(\rho):=\sup_{n} f(\rho_{1:n})
$$
For non-deterministic state machines, we assign $f(\rho^\dagger)=\inf_{\rho\in \rho^\dagger} f(\rho)$ , where $\rho^\dagger$ denotes weight all possible trajectories of transitions. 
We define the weight by $\frak a$ , computed from $f$ , $\gamma$ and execution trajectories. 
## Comparator Automaton
Comparator automaton is a composed from or weighted automaton. 
Using the product of two weighted automata, we find naturally we can define an automaton such that it accepts compositional inputs $w_{\frak A}, w_{\frak B}$ iff ${\frak a}_{\frak A}(w_{\frak A})\le {\frak a}_{\frak B}(w_{\frak B})$ 
However, while the definition is very clear here, the problem remains in how to construct such an automaton, and if this is always the case to construct it such that it is effectively computable / effectively enumerable. 
## Quantitive inclusion
Let $\frak P$ and $\frak Q$ be weighted $\omega$-automata with the same aggregate function $f$ and the same alphabet. The quantitive inclusion problem asks whether $\frak P\overset{\mathit f}\subset Q$ , which is defined as $\forall w:{\frak a}_{\frak P}(w)<{\frak a}_{\frak Q}(w)$ (strict) or $\forall w:{\frak a}_{\frak P}(w)\le {\frak a}_{\frak Q}(w)$ (non-strict). 
Given the definition of quantitive inclusion, now we try to construct a comparator automaton for quantitive inclusion problem. The problem is how to transform the $\forall w:\cdots$ into a final-check fashion in comparator automaton. 