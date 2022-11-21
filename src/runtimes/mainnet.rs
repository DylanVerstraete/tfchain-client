#[subxt::subxt(runtime_metadata_path = "artifacts/mainnet.scale")]
pub mod mainnet {
    #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
    use ::sp_std::vec::Vec;
}
pub use mainnet::runtime_types::frame_system::AccountInfo;
pub use mainnet::runtime_types::pallet_balances::AccountData;
pub use mainnet::runtime_types::pallet_smart_contract::types::Contract;
pub use mainnet::runtime_types::pallet_tfgrid::{
    farm::FarmName,
    interface::{InterfaceIp, InterfaceMac, InterfaceName},
    pub_config::{Domain, GW4, GW6, IP4, IP6},
    pub_ip::{GatewayIP, PublicIP},
    twin::TwinIp,
    types::Twin as TwinData,
};
pub use mainnet::runtime_types::tfchain_support::types::{
    Farm as FarmData, Interface, Node as NodeData, PublicConfig, PublicIP as PublicIpData, IP,
};
use sp_core::{crypto::AccountId32, H256};
use subxt::{tx::PairSigner, Error};

pub type Twin = TwinData<TwinIp, sp_core::crypto::AccountId32>;

pub type PublicIpOf = PublicIpData<PublicIP, GatewayIP>;
pub type Farm = FarmData<FarmName, PublicIpOf>;

pub type IPv4 = IP<IP4, GW4>;
pub type IPv6 = IP<IP6, GW6>;
pub type PublicConfigOf = PublicConfig<IPv4, Option<IPv6>, Option<Domain>>;
pub type InterfaceOf = Interface<InterfaceName, InterfaceMac, Vec<InterfaceIp>>;
pub type Node = NodeData<PublicConfigOf, InterfaceOf>;

use crate::client::TfchainClient;

pub use mainnet::tft_bridge_module::events::BurnTransactionReady;
pub use mainnet::tft_bridge_module::events::BurnTransactionSignatureAdded;
pub use mainnet::tft_bridge_module::events::MintTransactionProposed;

pub async fn create_twin(cl: &TfchainClient, ip: String) -> Result<H256, Error> {
    let create_twin_tx = mainnet::tx()
        .tfgrid_module()
        .create_twin(ip.as_bytes().to_vec());
    let signer = PairSigner::new(cl.pair.clone());
    cl.api
        .tx()
        .sign_and_submit_default(&create_twin_tx, &signer)
        .await
}

pub async fn sign_terms_and_conditions(
    cl: &TfchainClient,
    document_link: String,
    document_hash: String,
) -> Result<H256, Error> {
    let create_twin_tx = mainnet::tx().tfgrid_module().user_accept_tc(
        document_link.as_bytes().to_vec(),
        document_hash.as_bytes().to_vec(),
    );
    let signer = PairSigner::new(cl.pair.clone());
    cl.api
        .tx()
        .sign_and_submit_default(&create_twin_tx, &signer)
        .await
}

pub async fn get_twin_by_id(cl: &TfchainClient, id: u32) -> Result<Option<Twin>, Error> {
    cl.api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().twins(id), None)
        .await
}

pub async fn get_contract_by_id(cl: &TfchainClient, id: u64) -> Result<Option<Contract>, Error> {
    cl.api
        .storage()
        .fetch(
            &mainnet::storage().smart_contract_module().contracts(id),
            None,
        )
        .await
}

pub async fn get_node_by_id(cl: &TfchainClient, id: u32) -> Result<Option<Node>, Error> {
    cl.api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().nodes(id), None)
        .await
}

pub async fn get_farm_by_id(cl: &TfchainClient, id: u32) -> Result<Option<Farm>, Error> {
    cl.api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().farms(id), None)
        .await
}

pub type SystemAccountInfo = Option<AccountInfo<u32, AccountData<u128>>>;

pub async fn get_balance(
    cl: &TfchainClient,
    account: AccountId32,
) -> Result<SystemAccountInfo, Error> {
    cl.api
        .storage()
        .fetch(&mainnet::storage().system().account(account), None)
        .await
}