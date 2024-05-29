use cumulus_primitives_core::ParaId;
use parachain_template_runtime::{
    constants::currency::{DOLLARS, EXISTENTIAL_DEPOSIT},
    AccountId, AuraId, Balance, Signature,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

const TOTAL_INITIAL_ISSUANCE: Balance = 100_000_000 * DOLLARS;

const INITIAL_XCAVATE_SUDO_SIGNATORIES: [&str; 2] = [
    "1uWNn87BVmATvKRHW6ptAdhTBaYgYBakeKhJnRYCCPJJGaY",
    "1xGrVAzbHfUfxFMRhUsDKWFcJNf1XMSNE2UqRjkYyFhrnqt",
];

const INITIAL_XCAVATE_COLLATORS: [&str; 2] = [
    "5CkQqtZSMxaKzJxtELKvcAkf5FoWLE3EZfyETzim7MhtXSRB",
    "5GjYdjr6q96athBjxZsZtDk6tTbuhvXuTbjGc8cH8aKMceRR",
];

const INITIAL_ISSUANCE_PER_SIGNATORY: Balance = 500 * DOLLARS;

const INITIAL_ISSUANCE_PER_COLLATOR: Balance = 200 * DOLLARS;


pub fn get_xcavate_session_keys(keys: AuraId) -> parachain_template_runtime::SessionKeys {
	parachain_template_runtime::SessionKeys { aura: keys }
}

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<parachain_template_runtime::RuntimeGenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain
/// in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we
/// have just one key).
pub fn template_session_keys(keys: AuraId) -> parachain_template_runtime::SessionKeys {
    parachain_template_runtime::SessionKeys { aura: keys }
}

pub fn xcavate_config() -> ChainSpec {
    // Give your base currency a XCAV name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XCAV".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 8888.into());

    let mut signatories: Vec<_> = INITIAL_XCAVATE_SUDO_SIGNATORIES
        .iter()
        .chain(vec![id].iter())
        .map(|ss58| AccountId::from_ss58check(ss58).unwrap())
        .collect();
    signatories.sort();

    let sudo_account = pallet_multisig::Pallet::<parachain_template_runtime::Runtime>::multi_account_id(
        &signatories[..],
        2,
    )

    let collators: Vec<_> = INITIAL_XCAVATE_COLLATORS
		.iter()
		.map(|ss58| AccountId::from_ss58check(ss58).unwrap())
		.collect();

    let mut balances = vec![];

    for collator in collators.clone() {
		balances
			.push((collator, INITIAL_ISSUANCE_PER_COLLATOR));
	}

    for signatory in INITIAL_XCAVATE_SUDO_SIGNATORIES.iter() {
		let account_id = AccountId::from_ss58check(signatory).unwrap();
		balances.push((account_id, pendulum::INITIAL_ISSUANCE_PER_SIGNATORY));
	}



    ChainSpec::builder(
        parachain_template_runtime::WASM_BINARY
            .expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "polkadot".into(),
            // You MUST set this to the correct network!
            para_id: 3376u32,
        },
    )
    .with_name("Xcavate Protocol")
    .with_id("xcavate")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(xcavate_genesis(
        // initial collators.
        collators.clone(),
        balances.clone(),
        sudo_account.clone(),
        3376.into(),
    ))
    .build()
}

// pub fn local_testnet_config() -> ChainSpec {
//     // Give your base currency a XCAV name and decimal places
//     let mut properties = sc_chain_spec::Properties::new();
//     properties.insert("tokenSymbol".into(), "XCAV".into());
//     properties.insert("tokenDecimals".into(), 12.into());
//     properties.insert("ss58Format".into(), 8888.into());

//     #[allow(deprecated)]
//     ChainSpec::builder(
//         parachain_template_runtime::WASM_BINARY
//             .expect("WASM binary was not built, please build it!"),
//         Extensions {
//             relay_chain: "polkadot".into(),
//             // You MUST set this to the correct network!
//             para_id: 4003,
//         },
//     )
//     .with_name("Local Testnet")
//     .with_id("local_testnet")
//     .with_chain_type(ChainType::Local)
//     .with_genesis_config_patch(testnet_genesis(
//         // initial collators.
//         vec![
//             (
//                 get_account_id_from_seed::<sr25519::Public>("Alice"),
//                 get_collator_keys_from_seed("Alice"),
//             ),
//             (
//                 get_account_id_from_seed::<sr25519::Public>("Bob"),
//                 get_collator_keys_from_seed("Bob"),
//             ),
//         ],
//         vec![

//         ],
//         get_account_id_from_seed::<sr25519::Public>("Alice"),
//         1000.into(),
//     ))
//     .with_protocol_id("template-local")
//     .with_properties(properties)
//     .build()
// }

fn xcavate_genesis(
    invulnerables: Vec<AccountId>,
    mut balances: Vec<(AccountId, Balance)>,
    sudo_account: AccountId,
    id: ParaId,
) -> serde_json::Value {

    let mut genesis_issuance = TOTAL_INITIAL_ISSUANCE;
	for balance in balances.clone() {
		genesis_issuance -= balance.1;
	}

	balances.push((sudo_account.clone(), genesis_issuance));

    serde_json::json!({
        "balances": {
            "balances": balances,
        },
        "parachainInfo": {
            "parachainId": id,
        },
        "collatorSelection": {
            "invulnerables": invulnerables,
            "candidacyBond": EXISTENTIAL_DEPOSIT * 16,
        },
        "session": {
            "keys": invulnerables
                .into_iter()
                .map(|account| {
                    (
                        account.clone(),                 // account id
                        account.clone(),                 // validator id
                        get_xcavate_session_keys(Into::<[u8; 32]>::into(account).unchecked_into()),  // session keys
                    )
                })
            .collect::<Vec<_>>(),
        },
        "treasury": {},
        "polkadotXcm": {
            "safeXcmVersion": Some(SAFE_XCM_VERSION),
        },
        "sudo": { "key": Some(sudo_account) }
    })
}
