# Web3 Resources

## Technologies/Tools
- [ ] [WASM](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [ ] [Solidity]()
- [ ] [Substrate](https://docs.substrate.io/tutorials/)
- [ ] [Ink!](https://use.ink/examples/smart-contracts)
- [ ] [ZKP]()
- [ ] [libp2p]()

## Concepts
- Cryptography 101: Hash Based DS, Digital signatures, encryption
- Blockchain from scratch
- Accounts and UTXOs
- Zero Knowledge Proofs
- Smart Contracts

## Ink Topics
- Template, basic macros: `#[ink::contract]`, `#[ink(storage)]`,
	`#[ink(constructor)`, `#[ink(message)]`, `#[ink::test]`,
	`#[ink_e2e::test]`
- Events, RPC Endpoints
- Upgradable contracts
- Environment functions, Chain environment types
- Contract testing, debugging and verification

## Ink Template
```rust
#[ink::contract]
pub mod flipper {
	#[ink(storage)]
	pub struct Flipper(bool);

	impl Flipper {
		#[ink(constructor)]
		pub fn new() -> Self {}

		#[ink(message)]
		pub fn flip(&mut self) {}

		#[ink(message)]
		pub fn get(&self) -> bool {}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[ink::test] 
		fn new_works() {}
	}
}


#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
	use super::*;
    use ink_e2e::build_message;
    type E2EResult<T> = Result<T, Box<dyn Error>>;

	#[ink_e2e::test] 
	async fn it_works(
		mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {}
}
```

