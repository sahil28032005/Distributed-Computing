# 🐭 MiceDB v1
A lightweight, distributed database system built with Rust and managed with Turborepo

## 📋 Table of Contents
- [Overview](#-overview)
- [Architecture](#-architecture)
- [Features](#-features)
- [Components](#-components)
- [Getting Started](#-getting-started)
- [Usage Examples](#-usage-examples)
- [Performance](#-performance)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)

## 🔍 Overview
**MiceDB** is a distributed database system designed for high performance, reliability, and scalability. Built in **Rust** for maximum efficiency and memory safety, MiceDB combines the flexibility of a distributed **key-value store** with the familiarity of **SQL**, making it suitable for a wide range of applications.

> Just as mice are small but mighty creatures working together in colonies, MiceDB nodes collaborate in a distributed environment to provide robust, fault-tolerant data storage and querying capabilities.

## 🏗️ Architecture

```mermaid
graph TD
    Client[Client Applications] --> Coordinator

    subgraph Coordinator[Coordinator Node]
        SQL[SQL Parser] --> QueryPlanner[Query Planner]
        QueryPlanner --> MetadataManager[Metadata Manager]
        MetadataManager --> Router[Request Router]
    end

    subgraph DataNodes[Data Nodes]
        Node1[Node 1] 
        Node2[Node 2]
        Node3[Node 3]
    end

    Router --> Node1
    Router --> Node2
    Router --> Node3

    subgraph RaftConsensus[Raft Consensus Groups]
        Raft1[Raft Group 1]
        Raft2[Raft Group 2]
    end

    Node1 --> Raft1
    Node2 --> Raft1
    Node1 --> Raft2
    Node3 --> Raft2

    subgraph Storage[Storage Layer]
        RocksDB1[RocksDB Instance 1]
        RocksDB2[RocksDB Instance 2]
        RocksDB3[RocksDB Instance 3]
    end

    Node1 --> RocksDB1
    Node2 --> RocksDB2
    Node3 --> RocksDB3


## 🚀 Features
| Feature | Description | Status |
|---------|-------------|--------|
| Distributed Architecture | Scale horizontally across multiple nodes | 🟡 In Progress |
| SQL Interface | Support for SQL queries on distributed data | 🟡 In Progress |
| Key-Value API | Simple KV operations for high-performance access | 🟢 Implemented |
| Raft Consensus | Ensures data consistency across replicas | 🟡 In Progress |
| Automatic Sharding | Data automatically distributed across nodes | 🔴 Planned |
| Fault Tolerance | System continues operating despite node failures | 🟡 In Progress |
| Observability | Metrics, tracing, and logging for system monitoring | 🟡 In Progress |
| Snapshot & Recovery | Point-in-time snapshots and recovery capabilities | 🔴 Planned |
 