# Apraxus APXS Token

This repository contains the APXS ERC-20 token implementation and its automated tests.

## Token

- Name: Apraxus
- Symbol: APXS
- Decimals: 8
- Maximum Supply: 1,000,000,000 APXS
- Additional minting: None
- Burn: Token holders may burn their own APXS

## Contract

The APXS contract is located at:

`contracts/APXS.sol`

The contract inherits OpenZeppelin ERC20 and mints the fixed maximum supply once during construction to the specified initial holder.

The constructor rejects the zero address.

There is no owner-controlled mint function, upgrade mechanism, or administrative supply expansion mechanism.

## Supply

The maximum supply is:

`1,000,000,000 APXS`

With 8 decimals, this corresponds to:

`100000000000000000` atomic units.

The constructor performs the initial mint of the complete maximum supply.

Burning reduces total supply and cannot create additional APXS.

## Testing

Tests are located at:

`test/APXS.ts`

Run the full test suite with:

```bash
npx hardhat test
