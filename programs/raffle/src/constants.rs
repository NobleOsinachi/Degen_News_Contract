use anchor_lang::prelude::Pubkey;

pub const POOL_SEED: &str = "pool";

// pub const ADMIN_KEY: Pubkey = anchor_lang::solana_program::pubkey!("DBadiSE9HsUhKqSmcNnsVtzUuwAeLtGFVCKY6LC1W9us"); 
pub const ADMIN_KEY: Pubkey = anchor_lang::solana_program::pubkey!("3ttYrBAp5D2sTG2gaBjg8EtrZecqBQSBuFRhsqHWPYxX"); 
pub const MAX_BUYER_COUNT: usize = 100;
pub const MAX_TOTAL_TICKET: u32 = 1000000000;
pub const MAX_RAFFLE_ID_LEN: usize = 50;
// pub const PAY_TOKEN: Pubkey = anchor_lang::solana_program::pubkey!("DmmF2EWXf6emVJ4FmcS5SPBnKftX5kX4WFxaxVXy3NU2");  //mainnet
pub const PAY_TOKEN: Pubkey = anchor_lang::solana_program::pubkey!("55u5jMiJtwsvyo834R2mmcrxMGu7x2KvbrguJNbHFnEJ");  //devnet
pub const DECIMAL: u64 = 1000000000;
pub const SLOTHASH_PUBKEY: Pubkey = anchor_lang::solana_program::pubkey!("SysvarS1otHashes111111111111111111111111111"); 
// pub const COLLECTION_KEY: Pubkey = anchor_lang::solana_program::pubkey!("CaYsVNkgS5yBMK1BVTmbpapumjbyXNsBFZ2W1zbbk374"); //mainnet
pub const COLLECTION_KEY: Pubkey = anchor_lang::solana_program::pubkey!("91jNpHwSpuUFY8tyEa2zrAjHnufQd8QtDaoptbxqiXVT"); //devnet