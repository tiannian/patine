# Patine

> Write EVM contract using Rust.

## Features

- Generate EVM bytecode from Rust code.
- Contract compostion.
- Call contract in contract.
- Follow the soldity ABI and call convention.
- Generate ABI.
- Generate client SDK.

## TODO

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

## Getting Started

Refer to [store.rs](./std/examples/store.rs)

## License

Licensed under:

- MIT license ([LICENSE](./LICENSE) or http://opensource.org/licenses/MIT)
