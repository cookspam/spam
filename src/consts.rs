use solana_program::{keccak::Hash, pubkey, pubkey::Pubkey};

/// The unix timestamp after which mining can begin.
pub const START_AT: i64 = 1712070600;

/// The reward rate to intialize the program with.
pub const INITIAL_REWARD_RATE: u64 = 10u64.pow(3u32);

/// The mining difficulty to initialize the program with.
pub const INITIAL_DIFFICULTY: Hash = Hash::new_from_array([
    0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
]);

/// The decimal precision of the SPAM token.
/// Using SI prefixes, the smallest indivisible unit of SPAM is a nanoSPAM.
/// 1 nanoSPAM = 0.000000001 SPAM = one billionth of an SPAM
pub const TOKEN_DECIMALS: u8 = 9;

/// One SPAM token, denominated in units of nanoSPAM.
pub const ONE_SPAM: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The duration of an epoch, in units of seconds.
pub const EPOCH_DURATION: i64 = 60;

/// The target quantity of SPAM to be mined per epoch, in units of nanoSPAM.
/// Inflation rate ≈ 1 SPAM / epoch (min 0, max 2)
pub const TARGET_EPOCH_REWARDS: u64 = ONE_SPAM;

/// The maximum quantity of SPAM that can be mined per epoch, in units of nanoSPAM.
pub const MAX_EPOCH_REWARDS: u64 = ONE_SPAM.saturating_mul(2);

/// The quantity of SPAM each bus is allowed to issue per epoch.
pub const BUS_EPOCH_REWARDS: u64 = MAX_EPOCH_REWARDS.saturating_div(BUS_COUNT as u64);

/// The number of bus accounts, for parallelizing mine operations.
pub const BUS_COUNT: usize = 8;

/// The smoothing factor for reward rate changes. The reward rate cannot change by more or less
/// than a factor of this constant from one epoch to the next.
pub const SMOOTHING_FACTOR: u64 = 2;

// Assert MAX_EPOCH_REWARDS is evenly divisible by BUS_COUNT.
static_assertions::const_assert!(
    (MAX_EPOCH_REWARDS / BUS_COUNT as u64) * BUS_COUNT as u64 == MAX_EPOCH_REWARDS
);

/// The seed of the bus account PDA.
pub const BUS: &[u8] = b"bus";

/// The seed of the mint account PDA.
pub const MINT: &[u8] = b"mint";

/// The seed of proof account PDAs.
pub const PROOF: &[u8] = b"proof";

/// The seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// Noise for deriving the mint PDA.
pub const MINT_NOISE: [u8; 16] = [
    210, 212, 200, 30, 119, 36, 136, 231, 233, 213, 138, 58, 17, 208, 176, 157
];

/// The addresses of the bus accounts.
pub const BUS_ADDRESSES: [Pubkey; BUS_COUNT] = [
    pubkey!("DzLpPA3uYgTzSnCJDamwKhKzYyKKPraN1SJdv3hboBMB"),
    pubkey!("2Zn77yZspohsPkLP9zcWX3dxuQ69dTRNyJciVEDENJh3"),
    pubkey!("4p8nEz7XMayiAkHYCrgs5WPWv4DUAxzcKpzX4X1Lyf61"),
    pubkey!("5g6DanqLyEwEm2zrbJCR67g4NwGMNwPcF6gB9AqbxncJ"),
    pubkey!("8ktdXVusqMvNHkZmUnSoRy2kjQEsVsGC387K9vXL2Q6"),
    pubkey!("DrKC38wdpumpkJwPLEa7yky9su1v82Ng2kNPy7UMt5fa"),
    pubkey!("CM6ergyxwT2kKaGD2EMXwgi8KBKDa5sCZESWRhhqRT1z"),
    pubkey!("F9kpy13nmNkxGUA5riGbAkLkR6Ky62LgiydUD5AfTEKm"),
];

/// The address of the mint account.
pub const MINT_ADDRESS: Pubkey = pubkey!("spamwgqKEBE2BtsfE2QesxpmYZZKp3LfHsEdF1MLpfU");

/// The address of the treasury account.
pub const TREASURY_ADDRESS: Pubkey = pubkey!("3amHhT6cLgvfjKWbka6DYjs9zS5pLFnmYw1g8C6DPa4x");
