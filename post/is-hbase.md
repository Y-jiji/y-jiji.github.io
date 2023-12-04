---
topic: System Design and Implementation
tags:
  - Paper-Sharing
abstr: Notes on in storage HBase, attributing to Zhichao Cao
date: 2023-12-04
title: Notes on In-Storage HBase
---
# Notes on In-Storage HBase
#2023-12-04 #Paper-Sharing
## Introduction
This blog entry attributes to Zhichao Cao's IS-HBase paper. 
The main problem addressed in this article is the seperation of computation resources and storage resources in modern infrastructure. 
In the past, we usually assign each server a fixed proportion of computation / storage. The system architecture looks like: 
![|500](Distribute-1.svg)
It all looks good (computation-near-storage), but has one fatal deficiency: you cannot flexibly add more computation resource or more storage resource independently. Therefore, for software tailored to run on this architecture, you cannot easily run one computation-intensive application on that, and run another data-intensive application later. Or in an application with flexible computation v.s. storage needs, you have to use a server even if you don't need the computation or storage. 
Therefore, in modern data centers, a currerntly prevailing architecture looks like: 
![|600](Distribute-2.svg)
With computation-capable servers and storage-capable servers, the desired amount of storage and computation can be balanced. 
However, this architecture featured independent scaling by introducing another problem (especially for KV-databases): the locality of storage and computation is not good, and you cannot push computation to storage as much as you want, which is previously considered as a principle in system software design. 
To address this issue Cao proposed IS-HBase, leveraging limited computation resources on storage servers to reduce network traffic in this new architecture. 
## Preliminaries on HBase
HBase provides reading and updating primitives: 
- Get(key): returns value from the storage. 
- Put(key, value): returns value from the storage
- Scan(key_range): returns value from storge
HBase logically divides into three components: 
- RegionServer: a key range is called a region, it receive and respond to the queries from clients. 
- HMaster: it decides key range assignment, manages tables, and handle high-level services
- ZooKeeper: it monitors and maintains the states of clusters (mostly for lock managing)
On storage, each SSTable is called an HFile in HBase. In our first architecture, the HBase maintains RegionServer and the storage on the same physical machine, which means most files are local to read requests. However, due to limited storage space in computation servers, we cannot simply run ResgionServer on the storage machine. 
## Profiling Performance Degradation
The original article presented a more detailed profiling over naive HBase design. 
Here we only present the implications of these profiling data for brevity: 
- Compaction causes higher read and write amplification -> influences overall performance
- Read amplification becomes more severe for `Get` and small `Scan`s. 
- Performance of filtered `Scan`s are highly influenced upon network congestion. 
## Designing IS-HBase
Environment: 
- Region server as VMs in computation clusters
- Computation cluster: big memory & performant CPU
- Storage cluster $\approx$ HDFS datanode
Design choices: 
- Block Cache + Partial Cache -> Region Server
- Scanner / Filter -> Storage Server
- Meta Data Service -> Storage Server
## Scan and Get
Get operation $\approx$ short scan
Therefore, we only describe scan operation: 
- Long and dense scans -> send block from storage servers (manual decision)
- Short or sparse (filtered) scans -> send entries from storage servers (manual decision)
- When a Scan RPC call reaches a storage server, it starts a scanner for each HFile such that:  
    - The starter key is within the key range of the HFile
    - Or it is smaller then the end key of the HFile
- Those scanners merge data into a single stream / iterator, and then hand data to Region Server, where data from different storage servers are merged into a single stream / iterator. 
## Compaction
Compaction involves loading data from different storage servers. However, we don't want these data to be processed by Region Servers. 
A possible compaction workflow looks like: 
- A Region Server decides to compact HFiles in its region. It chooses several HFiles as candidates. 
- The Region Server queries the storage system for metadata, and locate these HFiles. 
- Region Server creates and empty HFile X. 
- The storage server (X's owner) starts a compaction job with metadata from the Region Server. It calls Scan RPC on the located HFiles, and merge scanned files into one. 
- Remove merged HFiles. 
## Partial Cache
As mentioned before, sometimes data is not transmitted to Region Servers as blocks. Instead, they are transmitted as single entries. Naturally, this invalidates the notion of 'block cache'. 
The partial cache is maintained as a hashmap -- 'HFile:Key -> Value'  and the replacement policy is LRU policy. 
## Adaptive Cache
Implement partial cache by managing the space explicitly (e.g. using arena-alike allocation scheme). 
Track the range of the partial block. 
- High cache missing rate in one partial block -> more complete blocks
- High cache hitting rate on a few KV-pairs -> evict complete block
(Only effective if locality changing slowly through time. Otherwise, the case is analogous to trembling virtual memory in OS)
