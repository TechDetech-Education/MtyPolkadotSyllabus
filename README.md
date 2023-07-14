# Pre-PBA Scheduled Syllabus 

This present 4-month program is designed to prepare beginner programmers for the entry Rust exam of the Polkadot Blockchain Academy (PBA) by equipping them with the necessary knowledge and skills in Rust programming and smart contract development using Ink!. Throughout the program, students engage in hands-on activities, coding exercises, assessments, and discussions to reinforce their learning and prepare them for working as Smart-Contract developers in the Polkadot ecosystem.

The program was designed taking in account the topics that students will face at the PBA (more details [here](./PBASummary.md)), the [PBA Rust entry exam](https://github.com/Polkadot-Blockchain-Academy/pba-qualifier-exam), and the Google's guide [📖 Comprehensive Rust](https://google.github.io/comprehensive-rust/). Extra resources worth mentioning are listed [here](./LearningResources.md). Each week has a dedicated `Exercise` link where the didactic material will be located.

## Calendar (66 days - 22 weeks - 5 months)
```
Aug (5) : 1,3, 5 -  8,10,12 - 15,17,19 - 22,24,26 - 29,31,2
Sep (4) : 5,7, 9 - 12,14,16 - 19,21,23 - 26,28,30
Oct (5) : 3,5, 7 - 10,12,14 - 17,19,21 - 24,26,28 - 31, 2,4
Nov (4) : 7,9,11 - 14,16,18 - 21,23,25 - 28,30, 2 
Dec (4) : 5,7, 9 - 12,14,16 - 19,21,23 - 26,28,30
```

## August: Intro to Blockchain with Rust
- **Objectives**:
	- Familiarize students with the basics of Rust programming language
	- Introduce fundamental blockchain concepts and cryptography
- **Topics**:
	1. Introduction to Rust and its features
	2. Variables, data types, and operators in Rust
	3. Control flow statements (if-else, loops) in Rust
	4. Functions and modules in Rust
	5. Cryptography basics for blockchain
- **Activities**:
	- [ ] Week 1: Implement a simple program in Rust that performs basic mathematical operations using functions and control flow statements. [Exercise](./exercises/w1/README.md)
	- [ ] Week 2: Explore Rust data types by building a program that manipulates different variable types (e.g., integers, strings). [Exercise](./exercises/w2/README.md)
	- [ ] Week 3: Write a Rust program that implements a basic encryption algorithm (e.g., Caesar cipher) to introduce cryptography concepts. [Exercise](./exercises/w3/README.md)
	- [ ] Week 4: Create a module in Rust that includes functions to handle file I/O operations, focusing on Rust's module system. [Exercise](./exercises/w4/README.md)
	- [ ] Week 5: Develop a small blockchain project in Rust that includes basic transaction validation and block mining using the blockchain concepts learned. [Exercise](./exercises/w5/README.md)
- **Resources**:
	- `rustlings`: 

## September: Smart Contracts with Ink!
- **Objectives**:
	- Dive deeper into Rust programming and apply it to smart contract development with Ink!
- **Topics**:
	1. Error handling in Rust (Result, Option types)
	2. Pattern matching and enums in Rust
	3. Implementing traits in Rust
	4. Introduction to smart contracts using Ink!
	5. Core `macros` and `traits` in Ink!
- **Activities**:
    - [ ] Week 1: Practice error handling in Rust by implementing a program that reads a file and handles any potential errors that may occur. [Exercise](./exercises/w6/README.md)
    - [ ] Week 2: Explore pattern matching in Rust by creating a program that categorizes given numbers as even or odd using pattern matching techniques. [Exercise](./exercises/w7/README.md)
    - [ ] Week 3: Implement a custom trait in Rust that defines a common behavior for different structs, such as a trait for printable objects. [Exercise](./exercises/w8/README.md)
    - [ ] Week 4: Begin learning about the Ink! framework by setting up a development environment and writing a simple smart contract that manages a basic token. [Exercise](./exercises/w9/README.md)
- **Resources**:
	- `sol2ink`: a tool to convert Solidity smart contracts to Ink!

## October: Macros and Data Structs
- **Objectives**:
	- Learn about Rust macros, advanced Rust concepts, and their applications.
	- Explore additional blockchain-related topics
- **Topics**:
    - Introduction to macros in Rust (declarative and procedural)
    - Ownership, borrowing, and lifetimes in Rust
    - Iterators and functional programming in Rust
    - Generics and associated types in Rust
    - Advanced Ink! features and contract development
- **Activities**:
    - [ ] Week 1: Create a declarative macro in Rust that generates code for a basic data structure, such as a linked list or a binary tree. [Exercise](./exercises/w10/README.md)
    - [ ] Week 2: Explore ownership, borrowing, and lifetimes in Rust by implementing a program that manages ownership and borrowing of blockchain assets. [Exercise](./exercises/w11/README.md)
    - [ ] Week 3: Utilize iterators and functional programming concepts in Rust to filter and manipulate blockchain data. [Exercise](./exercises/w12/README.md)
    - [ ] Week 4: Practice using generics and associated types in Rust by designing a reusable smart contract component in Ink! that supports multiple data types. [Exercise](./exercises/w13/README.md)
    - [ ] Week 5: Explore advanced Ink! features such as event handling, `RPC` endpoints, and upgradable contracts, and incorporate them into a developed smart contract. [Exercise](./exercises/w14/README.md)
- **Resources**:
	- `ink examples`

## November: Polkadot Integration
- **Objectives**: 
	- Learn about Polkadot integration with smart contracts
	- Apply knowledge and skills to complete a final project
	- Guidance through the PBA Rust entry exam
- **Topics**:
	- Polkadot architecture and integration with smart contracts
	- Interacting with Polkadot using Ink!
	- Testing, debugging, and security considerations for smart contracts
- **Activities**:
	- [ ] Week 1: Explore the architecture and governance of Polkadot and design an Ink! smart contract that interacts with the Polkadot ecosystem. [Exercise](./exercises/w15/README.md)
	- [ ] Week 2: Integrate an Ink! smart contract with other parachains or relay chains in the Polkadot network to enable cross-chain functionality. [Exercise](./exercises/w16/README.md)
	- [ ] Week 3: Discuss and implement best practices for testing, debugging, and ensuring the security of smart contracts. [Exercise](./exercises/w17/README.md)
	- [ ] Week 4: Complete a final project that showcases the students' understanding of Rust programming, smart contract development, and Polkadot integration. [Exercise](./exercises/w18/README.md)
- **Resources**:
	- `Moonbeam`
 
## December: Advanced Smart Contracts
- **Objectives**:
	- Dive deeper into advanced concepts and features of smart contracts using Ink!
	- Explore more complex contract design patterns and functionalities
- **Topics**:
	- Advanced event handling and event-driven architectures
	- Optimizing gas usage and contract efficiency
	- Implementing upgradeable contracts with storage migration
	- Building and interacting with off-chain components and Oracles in Ink!
- **Activities**:
	- [ ] Week 1: Explore strategies for optimizing gas usage and improving the efficiency of Ink! contracts, considering storage limitations and execution costs. [Exercise](./exercises/w19/README.md)
	- [ ] Week 2: Dive into building and integrating off-chain components and Oracles with Ink! contracts, enabling interaction with external data sources and systems. [Exercise](./exercises/w20/README.md)
