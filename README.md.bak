# Patine

> Write EVM contract using Rust.

## Features and TODO

- Generate EVM bytecode from Rust code.
- Contract compostion.
- Call contract in contract.
- Follow the soldity ABI and call convention.
- Generate ABI.
- Generate client SDK.


- [ ] Core library.
  - [X] FFI
  - [X] Warpped Function
  - [X] UInt
  - [X] Fixed BytesN
  - [X] SInt
  - [X] Address
  - [ ] Add `AsRef` and `AsMut`
  - [X] Add From
  - [ ] Add `TryFrom`
  - [X] Add "UnsafeFromLiteral"
  - [X] Memory
  - [ ] Bytes
  - [ ] String
- [ ] Std
  - [X] Selector
  - [ ] ABI
  - [X] Contract
  - [X] Context
    - [X] Msg
    - [X] Transaction
    - [X] Block
  - [X] Storage
    - [ ] Add `StorageEnDe`
- [ ] Macros
  - [ ] Contract
  - [ ] uint!
  - [ ] Event
  - [ ] AbiEncode, AbiDecode
  - [ ] Function Call

## Perquisites

Install `cargo-evm` by `cargo binstall`.

```bash
cargo binstall cargo-evm
```

## Create Contract Project

Create project using the following command:

```bash
cargo evm new "<your-project-name>"
```

## Write Contract

Please add your contract file under `src`. We use simple store contract as example.

```rust
use patine::{storage::{Value, Storage}, U256};

#[contract]
pub struct Contract {
    value: Value<U256, Storage>,
}

#[contract]
impl Contract {
    #[expose]
    pub fn store(&mut self, value: U256) {
        self.value.set(value)
    }

    #[expose]
    pub fn load() -> U256 {
        self.value.get()
    }
}
```
