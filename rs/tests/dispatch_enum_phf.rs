use phf::{phf_map, Map};

/// Perfect hash table generated at compile time (address -> rank u8)
static ADDRESS_MAP: Map<&'static str, u8> = phf_map! {
    // 50 of the most popular Ethereum smart contract addresses (example)
    "0x00000000219ab540356cbb839cbe05303d7705fa" => 1u8,   // Beacon Deposit Contract
    "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" => 2u8,   // Wrapped Ether (WETH)
    "0xdac17f958d2ee523a2206206994597c13d831ec7" => 3u8,   // Tether USD (USDT)
    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48" => 4u8,   // USD Coin (USDC)
    "0x6b175474e89094c44da98b954eedeac495271d0f" => 5u8,   // Dai Stablecoin (DAI)
    "0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce" => 6u8,   // Shiba Inu (SHIB)
    "0xae7ab96520de3a18e5e111b5eaab095312d7fe84" => 7u8,   // Lido Staked Ether (stETH)
    "0xf164fc0ec4e93095b804a4795bbe1e041497b92a" => 8u8,   // Uniswap V2: Router
    "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f" => 9u8,   // Uniswap V2: Factory
    "0x1111111254eeb25477b68fb85ed929f73a960582" => 10u8,  // 1inch Exchange Router
    // ... (40 other Ethereum contract addresses) ...
    "0x084bd9eded57eb30d5358f056ba84f6e14812704" => 11u8,
    "0x094df45c068cc031dada0a2eb043fe0efb244197" => 12u8,
    "0x0c23c042729c9d873480057b327e422db19752c3" => 13u8,
    "0x1376243c3b4f5611b9ea3d5ea29e5b228f56b1bf" => 14u8,
    "0x153e7fbc2af897e90f1c14910345885e7a4ec5a8" => 15u8,
    "0x175a2d8d951f2c9ab6fc5f1a7f89a8b6fd41e8b7" => 16u8,
    "0x1f8f38f87e2d2939dfdd5a946a77ea74759c1bb8" => 17u8,
    "0x21c35d2fd9ca0a351834fa176f5926d4cb8f1e9b" => 18u8,
    "0x28b5e3f98779c2f523e3bcbe441acbb4ccbf9b93" => 19u8,
    "0x2ed86798cdb592d2f45912734be570c7e6c03e6c" => 20u8,
    "0x32cc1f6a35cfbb480a0b8b0a3c33c2bd564d6c07" => 21u8,
    "0x3a8e0d0b4d99d544f32438e37e8866aa7c8db3f2" => 22u8,
    "0x3d7bc55e4dc95cbdfaad72e30423bd4140580d79" => 23u8,
    "0x409c7f77d9d3309c879ec0fdcbeb0d5ab1b3d3e4" => 24u8,
    "0x41817d5a51705b4b15d26afdfd4a5b1f9b8d1c0c" => 25u8,
    "0x4ecf8fc5e6eabdf6e03042631e5cf9d7badb4fc9" => 26u8,
    "0x5586b0d5b4c3bfd1b64893f59913e2c4d7dd28e3" => 27u8,
    "0x5a9b6d7081e3e4a34bf36b82e307081f62f8da59" => 28u8,
    "0x5f9c8d09e2fe39d4bc0c76616c30a9f96d1436d2" => 29u8,
    "0x6b6bf035c53d3ecfab98bef1a9c238a22528273e" => 30u8,
    "0x6d8b0b1c3f136b1c4a57bd201b2d4a2d429b9d18" => 31u8,
    "0x702d6f4b5f7e6e545faaefdc64f996e8c50a7b8a" => 32u8,
    "0x7a8d2aa67d9b049d632077102a2283b927f1f7dc" => 33u8,
    "0x7c6b5c6e08f8b3fcf3f2380bfd6dbc25556c9d4d" => 34u8,
    "0x7db4d0d0b71c9e632dcfa33d99caba909bcff7a9" => 35u8,
    "0x823c7e70454aae8c7c7c9237b485e38d071b9c8c" => 36u8,
    "0x8b0ad53a32e7f7c4008ce7c69e1897ec95f71110" => 37u8,
    "0x8c5120a8362928471e295b6a9d3df57d586dbf6d" => 38u8,
    "0x8fa9b2b80b92ba7bd331b52e4f7e0d0b5e6199ea" => 39u8,
    "0x991a8f0a860e6f8fc0751920234fbceaea6208b3" => 40u8,
    "0x9b39cfb2f4b33f5678e7a0700689ca7b8e2b60fa" => 41u8,
    "0x9c9f0f8cf5d8ffb4d202ba244a8b16b8f4360920" => 42u8,
    "0xb715d2a434d29279fa8ccbd8fd3054d8af4f4748" => 43u8,
    "0xc03fbe4d6f1755f525a856c6fde3f800ce6a5342" => 44u8,
    "0xc187229bdd24262d4e360087ea72fe226eb90881" => 45u8,
    "0xc192132e832c5267a56195dacfed3f7323b31b7a" => 46u8,
    "0xc4b0987a626b50d954635a9f907667a03324bd97" => 47u8,
    "0xeb420b09f3b12877f7ab915da1c62b2eca64c126" => 48u8,
    "0xf0714b2a3fbdcda6bde2a3dc2c8a9fef8ab0b7c1" => 49u8,
    "0xfd0e230aee06a80838734bb5d4bd703841a8bb0e" => 50u8
};

/// Sorted array of addresses (for binary search)
static SORTED_ADDRESSES: [&str; 50] = [
    "0x00000000219ab540356cbb839cbe05303d7705fa",
    "0x084bd9eded57eb30d5358f056ba84f6e14812704",
    "0x094df45c068cc031dada0a2eb043fe0efb244197",
    "0x0c23c042729c9d873480057b327e422db19752c3",
    "0x1111111254eeb25477b68fb85ed929f73a960582",
    "0x1376243c3b4f5611b9ea3d5ea29e5b228f56b1bf",
    "0x153e7fbc2af897e90f1c14910345885e7a4ec5a8",
    "0x175a2d8d951f2c9ab6fc5f1a7f89a8b6fd41e8b7",
    "0x1f8f38f87e2d2939dfdd5a946a77ea74759c1bb8",
    "0x21c35d2fd9ca0a351834fa176f5926d4cb8f1e9b",
    "0x28b5e3f98779c2f523e3bcbe441acbb4ccbf9b93",
    "0x2ed86798cdb592d2f45912734be570c7e6c03e6c",
    "0x32cc1f6a35cfbb480a0b8b0a3c33c2bd564d6c07",
    "0x3a8e0d0b4d99d544f32438e37e8866aa7c8db3f2",
    "0x3d7bc55e4dc95cbdfaad72e30423bd4140580d79",
    "0x409c7f77d9d3309c879ec0fdcbeb0d5ab1b3d3e4",
    "0x41817d5a51705b4b15d26afdfd4a5b1f9b8d1c0c",
    "0x4ecf8fc5e6eabdf6e03042631e5cf9d7badb4fc9",
    "0x5586b0d5b4c3bfd1b64893f59913e2c4d7dd28e3",
    "0x5a9b6d7081e3e4a34bf36b82e307081f62f8da59",
    "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f",
    "0x5f9c8d09e2fe39d4bc0c76616c30a9f96d1436d2",
    "0x6b175474e89094c44da98b954eedeac495271d0f",
    "0x6b6bf035c53d3ecfab98bef1a9c238a22528273e",
    "0x6d8b0b1c3f136b1c4a57bd201b2d4a2d429b9d18",
    "0x702d6f4b5f7e6e545faaefdc64f996e8c50a7b8a",
    "0x7a8d2aa67d9b049d632077102a2283b927f1f7dc",
    "0x7c6b5c6e08f8b3fcf3f2380bfd6dbc25556c9d4d",
    "0x7db4d0d0b71c9e632dcfa33d99caba909bcff7a9",
    "0x823c7e70454aae8c7c7c9237b485e38d071b9c8c",
    "0x8b0ad53a32e7f7c4008ce7c69e1897ec95f71110",
    "0x8c5120a8362928471e295b6a9d3df57d586dbf6d",
    "0x8fa9b2b80b92ba7bd331b52e4f7e0d0b5e6199ea",
    "0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce",
    "0x991a8f0a860e6f8fc0751920234fbceaea6208b3",
    "0x9b39cfb2f4b33f5678e7a0700689ca7b8e2b60fa",
    "0x9c9f0f8cf5d8ffb4d202ba244a8b16b8f4360920",
    "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
    "0xae7ab96520de3a18e5e111b5eaab095312d7fe84",
    "0xb715d2a434d29279fa8ccbd8fd3054d8af4f4748",
    "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    "0xc03fbe4d6f1755f525a856c6fde3f800ce6a5342",
    "0xc187229bdd24262d4e360087ea72fe226eb90881",
    "0xc192132e832c5267a56195dacfed3f7323b31b7a",
    "0xc4b0987a626b50d954635a9f907667a03324bd97",
    "0xdac17f958d2ee523a2206206994597c13d831ec7",
    "0xeb420b09f3b12877f7ab915da1c62b2eca64c126",
    "0xf0714b2a3fbdcda6bde2a3dc2c8a9fef8ab0b7c1",
    "0xf164fc0ec4e93095b804a4795bbe1e041497b92a",
    "0xfd0e230aee06a80838734bb5d4bd703841a8bb0e",
];

/// Corresponding values sorted in the same order as SORTED_ADDRESSES
static VALUES_BY_ADDRESS: [u8; 50] = [
    1u8, 11u8, 12u8, 13u8, 10u8, 14u8, 15u8, 16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8,
    25u8, 26u8, 27u8, 28u8, 9u8, 29u8, 5u8, 30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8,
    39u8, 6u8, 40u8, 41u8, 42u8, 4u8, 7u8, 43u8, 2u8, 44u8, 45u8, 46u8, 47u8, 3u8, 48u8, 49u8, 8u8,
    50u8,
];

/// Searches for the contract address's rank (`u8`).
/// Returns Ok(rank) if the address is found in the table, otherwise Err with an error message.
fn get_contract_rank(address: &str) -> Result<u8, &'static str> {
    // 1. Direct lookup in the perfect hash table (O(1))
    if let Some(&rank) = ADDRESS_MAP.get(address) {
        return Ok(rank);
    }

    // 2. Binary search in the sorted array (O(log n))
    match SORTED_ADDRESSES.binary_search_by(|probe| probe.cmp(&address)) {
        Ok(index) => Ok(VALUES_BY_ADDRESS[index]),
        Err(_) => Err("Address not found"),
    }
}

fn main() {
    // Example usage
    let addr = "0xdac17f958d2ee523a2206206994597c13d831ec7"; // USDT contract (present in the table)
    match get_contract_rank(addr) {
        Ok(rank) => println!("Address {} found, rank = {}", addr, rank),
        Err(err) => println!("Error: {}", err),
    }

    let unknown_addr = "0x0000000000000000000000000000000000000000"; // Unlisted address
    match get_contract_rank(unknown_addr) {
        Ok(rank) => println!("Address {} found, rank = {}", unknown_addr, rank),
        Err(err) => println!("Unknown address {}: {}", unknown_addr, err),
    }
}

#[test]
fn test() {
    main()
}
