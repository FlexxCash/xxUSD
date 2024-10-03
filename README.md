# xxUSD Project

xxUSD is a Solana-based stablecoin project implemented using the Anchor framework. This project aims to provide a stable cryptocurrency pegged to the US Dollar, with additional features such as locking and hedging strategies.

## Project Structure

The project is organized as follows:

```
xxUSD/
├── programs/
│   └── xxusd/
│       ├── src/
│       │   ├── core/
│       │   ├── error/
│       │   ├── events/
│       │   ├── instructions/
│       │   ├── oracle/
│       │   ├── state/
│       │   ├── utils/
│       │   └── lib.rs
│       └── Cargo.toml
└── Cargo.toml
```

## Folder and File Descriptions

### programs/xxusd/src/

This is the main source directory for the xxUSD program.

- `lib.rs`: The entry point of the program. It defines the program ID, imports necessary modules, and declares the program's instruction handlers.

### programs/xxusd/src/core/

Contains core data structures and functions used throughout the project.

- Defines `Amount` and `Timestamp` types.
- Implements `NumericType` trait.
- Provides utility functions for type conversions.

### programs/xxusd/src/error/

Defines custom error types for the xxUSD program.

- `XxusdError`: An enum of all possible errors that can occur in the program.

### programs/xxusd/src/events/

Likely contains definitions for various events that can be emitted by the program.

### programs/xxusd/src/instructions/

Contains the implementation of all instruction handlers for the program.

- `initialize_controller.rs`: Handles the initialization of the controller.
- `mint.rs`: Handles the minting of new xxUSD tokens.
- `redeem.rs`: Handles the redemption of xxUSD tokens.
- `lock_xxusd.rs`: Handles the locking of xxUSD tokens.
- `release_xxusd.rs`: Handles the release of locked xxUSD tokens.
- `manage_product_price.rs`: Handles the management of product prices.
- `manage_hedging_strategy.rs`: Handles the management of hedging strategies.
- `freeze_program.rs`: Handles freezing the program.
- `edit_controller.rs`: Handles editing the controller.

### programs/xxusd/src/oracle/

Likely contains implementations related to price oracles used in the project.

### programs/xxusd/src/state/

Contains definitions of various state accounts used in the program.

- `controller.rs`: Defines the `Controller` struct and its methods.
- `lock_manager.rs`: Defines the `LockManager` struct and its methods.
- `hedging_strategy.rs`: Defines structures and methods related to hedging strategies.

### programs/xxusd/src/utils/

Contains utility functions used throughout the project.

- `maths.rs`: Provides mathematical utility functions like `checked_add`, `checked_sub`, etc.

## Key Functions

1. `initialize_controller`: Initializes the controller with the specified redeemable mint decimals.
2. `mint`: Mints new xxUSD tokens in exchange for collateral.
3. `redeem`: Redeems xxUSD tokens for collateral.
4. `lock_xxusd`: Locks a specified amount of xxUSD tokens for a given period.
5. `release_xxusd`: Releases previously locked xxUSD tokens.
6. `manage_product_price`: Manages the price of a product in the system.
7. `manage_hedging_strategy`: Manages the hedging strategy, allowing deposits or withdrawals.
8. `freeze_program`: Freezes or unfreezes the program.
9. `edit_controller`: Edits the controller, potentially changing its authority.

## Dependencies

The project uses the following main dependencies:

- anchor-lang = "0.28.0"
- anchor-spl = "0.28.0"
- solana-program = "1.16.0"
- bytemuck = "1.13.1"

## Building and Testing

To build the project, use the Anchor CLI:

```
anchor build
```

To run tests:

```
anchor test
```

Note: Make sure you have the Solana toolchain and Anchor framework installed and properly configured before building or testing the project.

## License

[Insert license information here]

## Contributors

[List of contributors or link to contributors file]

## Additional Notes

This README provides an overview of the project structure and main components. For detailed information about each module and function, please refer to the inline documentation in the respective source files.