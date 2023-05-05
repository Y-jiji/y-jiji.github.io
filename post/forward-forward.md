---
topic: "AI + Numerical Algorithms"
tag: ["Deep Learning", "Training Algorithm"]
abstr: "The idea is simple: a hidden layer's job is to extract good enough feature. "
date: "2023-05-05"
title: "Understanding Forward-Forward"
---

# Understanding Forward-Forward (FF)

#2023-05-05 #WIP

## Introduction + Outline

Recently, the founding father of deep learning, Geoffrey Hinton presented a novel algorithm for deep neural network training [-> original material](https://www.cs.toronto.edu/~hinton/FFA13.pdf). Actually the idea of forward-gradient was somewhat borrowed from [Baydin's research](https://arxiv.org/abs/2202.08587), but Professor Hinton's team contributed a serial of twiks that make it scale out ([see their ICLR23 paper](https://arxiv.org/abs/2210.03310)). 

In later sections, we will try to cover all of this ideas and provide some possible motivations of this work. First, a brief review on Baydin's research will be presented, followed up by an explanation about why naive FF method doesn't work in practice. Then, we will elborate on some possible remedies to mitigate naive FF's problem, leading our way to practical FF method. Finally, we will give a wild guess of what these ideas imply, and list some possibly interesting topic. 

Bear in mind that reading this article requires some familarity with deep learning and multi-variate calculus. However, skipping some math-heavy paragraphs doesn't mean you will lose any important information. 

## Naive FF Method

Naive FF Method is super simple, and can be roughly described like this: 

```pseudocode
ACQUIRE W = paramters in the model
ACQUIRE H = the objective function with the model plugged-in like \
	H(W, D) = Objective(Model(W, D.input), D.label)
ACQUIRE D = input data
FOREACH iteration: 
    DW = a randomly guessed direction
    IF stepping along DW from W makes H(W, D) decrease: 
        stepping along DW
    ELSE:
    	stepping along -DW
```

The question is how to know if H(W, D) will increase/decrease before we actually try to walk in that direction? Here we have to compute directional derviative. Technically, directional derviative describes how the value of a function changes on a certain point when you apply a perturbation (small adjustment) on that point. We stop here before we step into differential forms and tangent spaces, and keep ourselves satisfied with this interpretation. 

For a $\mathbb{R}^n \to \mathbb{R}^m$ differentiable function, it can be defined as follows. 
$$
\begin{aligned}
&f:\mathbb{R}^n\to \mathbb{R}^m\\
&\frac{{\rm d}f}{{\rm d}\vec x}(x)=\lim_{h\to 0^+}\frac{f(x+h\cdot \vec x)-f(x)}{h}
\end{aligned}
$$
Where $\vec x$ is the direction of perturbation, and $h$ controls how big is the perturbation, so we can get the local direction of $f$ with given $h$ . 

And naturally we also have the chain rule
$$
\begin{aligned}
& f:\mathbb{R}^n\to \mathbb{R}^m\\
& g:\mathbb{R}^m\to \mathbb{R}^k\\
& \frac{{\rm d} f}{{\rm d}g}\cdot \frac{{\rm d}g}{{\rm d}\vec x}(x)=\frac{{\rm d}f}{{\rm d}\vec{g}}(g(x))\\
& where: \vec{g}=\frac{{\rm d}g}{{\rm d}\vec x}(x)
\end{aligned}
$$
which means we first compute the direction of $g(x)$ when we move $x$ along $\vec x$ , and then move the input of $f$ towards this computed, and finally we get the direction of $f$ with respect to $\vec x$ . 

## Big Big Parallelism, Good Good Parallelism

To perform this computation numerically, this algorithm is presumably fast on contemporary hardware, because at the point where derivative of objective function is known, we can adjust our parameters simutaneously, while in backward propogation (BP), we have to compute gradients layer by layer, which is sequential. Speaking about memory consumption, FF is at least as good as BP, because in FF we only have to keep the perturbation and input of each layer can be just thrown away after output ot this layer is computed, and in BP part of the input as well as some intermidiate results should be kept to compute backward gradient. 

When I talked about contemporary hardward, it is actually distributed computation that poped up in my mind. Text may be boring, so I try to visualize my thoughts. 

#TODO-PICTURE-HERE

## Big Big Variance, Bad Bad Variance

Although in previous sections we described FF method as computationally efficient, it doesn't mean FF method have good mathematical properties. 

## Suppressing Large Variance



## Nothing But Feature Extraction

About 7 or 8 years ago, there is something called feature-enigneering. 

