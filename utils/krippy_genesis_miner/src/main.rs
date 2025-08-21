use bascoin_consensus_core::{
    block::Block,
    config::genesis::{
        DEVNET_GENESIS,
        GENESIS,
        GenesisBlock,
        SIMNET_GENESIS,
        TESTNET11_GENESIS,
        TESTNET_GENESIS,
        KRIPPY_GENESIS,
        NIPPY_GENESIS,
    },
};
use bascoin_hashes::Hash;
use bascoin_core::time::unix_now;
use bascoin_math::Uint256;
use log::info;

/// Finds a nonce for the genesis block that results in a hash meeting the difficulty target.
/// This is a simple mining function for genesis block generation.
fn mine_genesis_block(mut genesis: GenesisBlock) -> (Hash, u64) {
    info!("Mining genesis block for network: {:?}", genesis.coinbase_payload);
    let target = Uint256::from_compact_target_bits(genesis.bits);

    let mut nonce = 0u64;
    loop {
        genesis.nonce = nonce;
        let block = Block::from(&genesis);
        let hash = block.header.hash;

        if Uint256::from_le_bytes(hash.as_bytes()) <= target {
            info!("Found genesis block with nonce: {}, hash: {}", nonce, hash);
            return (hash, nonce);
        }

        nonce += 1;

        if nonce % 100000 == 0 {
            info!("Current nonce: {}", nonce);
        }
    }
}

fn main() {
    env_logger::init();

    // Mainnet Genesis
    let mut mainnet_genesis = GENESIS;
    mainnet_genesis.timestamp = unix_now(); // Set current timestamp
    let (mainnet_hash, mainnet_nonce) = mine_genesis_block(mainnet_genesis.clone());
    info!("\nMAINNET GENESIS:");
    info!("  hash: Hash::from_bytes({:#x?}),", mainnet_hash.as_bytes());
    info!("  nonce: {:#x},", mainnet_nonce);
    info!("  timestamp: {},", mainnet_genesis.timestamp);

    // Testnet Genesis
    let mut testnet_genesis = TESTNET_GENESIS;
    testnet_genesis.timestamp = unix_now(); // Set current timestamp
    let (testnet_hash, testnet_nonce) = mine_genesis_block(testnet_genesis.clone());
    info!("\nTESTNET GENESIS:");
    info!("  hash: Hash::from_bytes({:#x?}),", testnet_hash.as_bytes());
    info!("  nonce: {:#x},", testnet_nonce);
    info!("  timestamp: {},", testnet_genesis.timestamp);

    // Testnet11 Genesis
    let mut testnet11_genesis = TESTNET11_GENESIS;
    testnet11_genesis.timestamp = unix_now(); // Set current timestamp
    let (testnet11_hash, testnet11_nonce) = mine_genesis_block(testnet11_genesis.clone());
    info!("\nTESTNET11 GENESIS:");
    info!("  hash: Hash::from_bytes({:#x?}),", testnet11_hash.as_bytes());
    info!("  nonce: {:#x},", testnet11_nonce);
    info!("  timestamp: {},", testnet11_genesis.timestamp);

    // Simnet Genesis
    let mut simnet_genesis = SIMNET_GENESIS;
    simnet_genesis.timestamp = unix_now(); // Set current timestamp
    let (simnet_hash, simnet_nonce) = mine_genesis_block(simnet_genesis.clone());
    info!("\nSIMNET GENESIS:");
    info!("  hash: Hash::from_bytes({:#x?}),", simnet_hash.as_bytes());
    info!("  nonce: {:#x},", simnet_nonce);
    info!("  timestamp: {},", simnet_genesis.timestamp);

    // Devnet Genesis
    let mut devnet_genesis = DEVNET_GENESIS;
    devnet_genesis.timestamp = unix_now(); // Set current timestamp
    let (devnet_hash, devnet_nonce) = mine_genesis_block(devnet_genesis.clone());
    info!("\nDEVNET GENESIS:");
    info!("  hash: Hash::from_bytes({:#x?}),", devnet_hash.as_bytes());
    info!("  nonce: {:#x},", devnet_nonce);
    info!("  timestamp: {},", devnet_genesis.timestamp);

    // Krippy Genesis
    let mut krippy_genesis = KRIPPY_GENESIS;
    krippy_genesis.timestamp = unix_now(); // Set current timestamp
    let (krippy_hash, krippy_nonce) = mine_genesis_block(krippy_genesis.clone());
    info!("\nKRIPPY GENESIS:");
    info!("  hash: Hash::from_bytes({:#x?}),", krippy_hash.as_bytes());
    info!("  nonce: {:#x},", krippy_nonce);
    info!("  timestamp: {},", krippy_genesis.timestamp);

    // Nippy Genesis
    let mut nippy_genesis = NIPPY_GENESIS;
    nippy_genesis.timestamp = unix_now(); // Set current timestamp
    let (nippy_hash, nippy_nonce) = mine_genesis_block(nippy_genesis.clone());
    info!("\nNIPPY GENESIS:");
    info!("  hash: Hash::from_bytes({:#x?}),", nippy_hash.as_bytes());
    info!("  nonce: {:#x},", nippy_nonce);
    info!("  timestamp: {},", nippy_genesis.timestamp);
}
