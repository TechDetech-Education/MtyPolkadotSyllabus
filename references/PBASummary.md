# PBA January 2023

## Rust Exam Topics
0. builtin types, keywords, lifetimes, scopes, iterators (Vec, ranges, slices),
1. "in place", common traits (From, Into, Ord), error handling
2. structural pattern matching, enums
3. implement traits (Ord, Eq, From, Default)
4. iterator api (filter, map, take, lambdas)
5. advanced traits (associated/generic types, PhantomData, `const`, `as`)
6. macros (declarative, procedural)
7. patterns (builder, type state)
8. search/sort algorithms, path finders

## Learning Outcomes
By the end of the PBA, learners will be able to:
- Explain the conceptual underpinnings of blockchain technology
- Apply economic, political and computer science concepts to blockchain  
  decisions
- Design blockchains and parachains using the Substrate framework
- Use pallets and FRAME to expedite Blockchain development
- Apply systems engineering principles to blockchain development
- Use XCM for cross-consensus messaging
- Work as a blockchain developer in the Polkadot ecosystem

## Syllabus
1. Cryptography 
    - [ ] Cryptography Intro
    - [ ] Digital Signatures
    - [ ] Exotic Primitives
    - [ ] Hashes
    - [ ] Hash Bases DS
    - **Excercises**: a1(Merkle Tree), e(Many Time Pad)
2. Economics & Game Theory
    - [ ] Economics Basics
    - [ ] Game Theory
    - [ ] Prices Finding Mechanisms
    - [ ] Collective Decision Making
    - **Excercises**: a1(Nash solver)
3. Blockchain
    - [ ] Overview of Blockchain
    - [ ] Blockchain Structure
    - [ ] Accounts and UTXOs
    - [ ] Consensus Authoring
    - [ ] Consensus Systems
    - [ ] Consensus Finality
    - [ ] Resource Allocation Fees Ordering
    - [ ] Light Clients and Bridges
    - [ ] Unstoppable Applications
    - **Excercises**: e(Blockchain from scratch)
4. Smart Contracts (before was mod-8)
    - [ ] Contracts in Web3
    - [ ] ink!
    - [ ] Contracts Pallets
    - [ ] Chain extensions
    - [ ] Contracts in the Polkadot context
    - **Excercises**: 
5. Substrate
    - [ ] Substrate Overview
    - [ ] Block Concepts
    - [ ] Runtime and Host Functions
    - [ ] Transaction Queue and Block Builder
    - [ ] SCALE
    - [ ] DB and Merklized Storage
    - [ ] Consensus Block Authoring and Finality
    - [ ] Networking
    - **Excercises**: a2(Frameless cryptocurrency)
6. FRAME and Pallets
    - [ ] Intro to FRAME
    - [ ] Dispatchables 
    - [ ] Hooks and Inherent
    - [ ] Exotic FRAME Methods
    - [ ] Important Pallets to Know
    - [ ] Connecting Pallets
    - [ ] Weights and Benchmarking
    - **Excercises**: a3(Dex, Quadratic Voting, Delegation PoS), e(Inkless
      flipper, Benchmarking)
7. Polkadot and Parachains
    - [ ] Polkadot Architecture and Governance
    - [ ] Parachains
    - [ ] Hashing in Polkadot
    - [ ] Relay to Parachain Communication
    - [ ] Cumulus
    - [ ] Authority and Logic Selection
    - **Excercises**: e(cumuless parachain)
8. XCM (Cross-Consensus Messaging Format)
    - [ ] Core Concepts of XCM
    - [ ] Writing Sending Executing XCM
    - [ ] Parachain Config in XCM
    - [ ] Testing and Troubleshooting XCM
    - [ ] XCM in Polkadot
    - **Excercises**: a4(send assets using XCM)
