use near_sdk::Gas;

/// Amount of gas for fungible token transfers, increased to 20T to support AS token contracts.
pub const GAS_FOR_FT_TRANSFER: Gas = Gas(20_000_000_000_000);
