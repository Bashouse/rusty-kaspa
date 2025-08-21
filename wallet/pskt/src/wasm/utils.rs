use kaspa_consensus_core::constants::*;
use kaspa_consensus_core::network::NetworkType;
use separator::{separated_float, separated_int, separated_uint_with_output, Separatable};

#[inline]
pub fn sompi_to_bascoin(sompi: u64) -> f64 {
    sompi as f64 / SOMPI_PER_BASCOIN as f64
}

#[inline]
pub fn bascoin_to_sompi(bascoin: f64) -> u64 {
    (bascoin * SOMPI_PER_BASCOIN as f64) as u64
}

#[inline]
pub fn sompi_to_bascoin_string(sompi: u64) -> String {
    sompi_to_bascoin(sompi).separated_string()
}

#[inline]
pub fn sompi_to_bascoin_string_with_trailing_zeroes(sompi: u64) -> String {
    separated_float!(format!("{:.8}", sompi_to_bascoin(sompi)))
}

pub fn bascoin_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "BAS",
        NetworkType::Testnet => "TBAS",
        NetworkType::Simnet => "SBAS",
        NetworkType::Devnet => "DBAS",
    }
}

#[inline]
pub fn sompi_to_bascoin_string_with_suffix(sompi: u64, network_type: &NetworkType) -> String {
    let bas = sompi_to_bascoin_string(sompi);
    let suffix = bascoin_suffix(network_type);
    format!("{bas} {suffix}")
}
