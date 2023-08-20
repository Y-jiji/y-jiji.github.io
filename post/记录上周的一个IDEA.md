---
topic: "AI + Numerical Algorithms"
tag: ["Paper Idea"]
abstr: "New Idea on Sparse Attention"
date: "2023-08-20"
title: "记录上周的一个IDEA"
---

记录上周想到的一个idea: 
- 现在大家都希望注意力机制能够在推理时复杂度不这么高. 
- 传统的 Transformer 每出一个新词都要 $O(文本长度)$ 的计算量
- 我想直接给它变成 $O(1)$ 
另一方面, 根据实践当中的观察, 我们也知道注意力一定是稀疏的. 
- 目前观察到的所有训练在真实数据上训练的Transformer里面, 注意力都是稀疏的. 
- 然后现在流行的结构都是Decoder Only的, 我就想改改Decoder Layer中的Mask Attention. 

然后非常搞的部分来了. 
我首先把Decoder Layer变成Decode两次. 
假定Input向量\[L, M\]的形状, 我们通过两个Linear层, 把它变成两个 \[L, D\] 维的张量, 分别记为 K_I 和 V_I . 
然后, 我们设置一个形状为 \[T, D\] 维的张量 Q_T, 将它和 K_I 还有 V_I 放到 Decoder Layer 中, 算出一组 \[T, D\] 维的输出, 记为 V_T . 
再设置一组参数 K_T , 形状同样是 \[T, D\]. 同时, 再根据Input向量算一次线性变换, 得到 Q_I . 根据 K_T, Q_I, V_T 再算一次注意力. 

那么问题来了, Decoder不是不能看后文吗, 这里可以看到后文了, 岂不是非常不妙?
这里就是刚刚那个问题出场的地方了
我们直接在 V_T\[i\] 当中包含它来自哪里的信息!
我们另外设一个张量 W_T , 它的形状为 \[T, D, N\] . 它的定义是 W_T\[t\]\[d\]\[n\] = V_T\[t\]\[d\] * Ph\[t\]\[n\]
第二次解码时, 将每个 W_T\[...\]\[...\]\[n\] 看作 V_T, 计算 N 个输出. 
记原来的输出为 Out, 计算 Out'\[t\]\[d\] = sum{n} Out\[t\]\[d\]\[n\] * Psi\[t\]\[n\] 就能消去所有 Phi\[t\]\[m\] m >= n 的成分. 
以上是并行情况. 
串行模式下, 因为对 V_T 的更新不影响前文, 所以不需要重新计算前面的内容, 而且因为 V_T 的形状是和句子大小无关, 计算一个新的Token只需常量时间. 
