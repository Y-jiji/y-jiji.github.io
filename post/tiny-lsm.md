---
topic: System Design and Implementation
tags:
  - LSM-Tree
abstr: Implement an LSM tree in Rust
date: 2023-12-04
title: Tiny LSM
---

# Tiny LSM
#2023-12-04 #LSM-Tree #Work-In-Progress
## What is an LSM-tree?
Unlike mappings represented in B-tree or Red-black-tree, technically LSM-tree is not a ***data structure*** for mappings. 
Instead, an LSM-tree is a running ***system*** for maintaining key-value mappings. 
LSM tree is an abbreivation of log structured merge tree. For system people, when people say something is 'log structured', it denotes an ***append only scheme***. 
In this blog, we will first investigate a naive idea of 'log structured' storage, and then extend this idea to LSM tree.
## LS But not Tree
Consider an application that: 
- needs to store a key-value mappings that supports point and range searching, and updating functionality
- has heavy write workload
- has mere read workload
The strawman approach is to create a stack-alike structure of key-value pairs. 
As the code listed below, we maintain a full log of write operations by denoting put by `(k, v)` and delete by `(k, None)` . 
It can be implemented as follows (PVec is some mapped disk storage): 
```rust
pub type LSVec<K, V> = PVec<(K, Option<V>)>
trait LSV {
    fn get(&self, k: K) -> Option<V>;
    fn put(&mut self, k: K, v: V);
    fn del(&mut self, k: K);
    fn scan(&self, k: Range<K>) -> impl Iterator<V>;
}
impl<K, V> LSV for LSVec<K, V>
    where K: Ord
{
    fn get(&self, k: K) -> Option<V> {
        self.iter().rev().filter(|(_k, v)| **_k == k)
            .map(|(_, v)| v).next().unwrap_or(None)
    }
    fn put(&mut self, k: K, v: V) {
        self.push((k, Some(v)));
    }
    fn del(&mut self, k: K, v: V) {
        self.push((k, None));
    }
    fn scan(&self, k: Range<K>) -> impl Iterator<Option<V>> {
        use std::collections::BTreeMap;
        let mut it = self.iter().rev().filter(|(_k, v)| **_k >= k.start && **_k < k.end);
        let mut rt = BTreeMap::new();
        for &(k, v) in it {
            if rt.contains(&k) { continue; }
            else { rt.insert((k, v)); }
        }
        return rt.into_iter().map(|(_, v)| v);
    }
}
```
## LS But not M
