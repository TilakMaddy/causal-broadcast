<p align="center"><h1 align="center">CAUSAL BROADCAST</h1></p>
<br>

##  Table of Contents

- [ Overview](#-overview)
- [ Getting Started](#-getting-started)
  - [ Prerequisites](#-prerequisites)
  - [ Installation](#-installation)
  - [ Usage](#-usage)

---

##  Overview

Super minimal codebase to demonstrate working on Causal Broadcast in distributed systems

##  Getting Started

###  Prerequisites

Before getting started with causal-broadcast, ensure your runtime environment meets the following requirements:

- **Rust Package Manager:** cargo
- **Javascript Package Manager:** npx


###  Installation

Install causal-broadcast using one of the following methods:

**Build from source:**

1. Clone the causal-broadcast repository:
```sh
❯ git clone https://github.com/TilakMaddy/causal-broadcast/
```

2. Navigate to the project directory:
```sh
❯ cd causal-broadcast
```

3. Install the project dependencies:

**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo build
```

###  Usage

Run causal-broadcast nodes using the following command:

```sh
❯ just run-nodes
```

Reuqest single node to broadcast

```sh
❯ just broadcast
```

Sequentially reuqest multiple nodes to broadcast

```sh
❯ just sequential-broadcast
```

Concurrently reuqest multiple nodes to broadcast

```sh
❯ just concurrent-broadcast
```

---
