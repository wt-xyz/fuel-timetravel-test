use std::time::Duration;

use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/timestamp-travel-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, Provider) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let provider = wallet.provider().unwrap().clone();

    let id = Contract::load_from(
        "./out/debug/timestamp-travel.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into(), provider)
}

#[tokio::test]
async fn can_get_timestamp() {
    let (_instance, _id, _provider) = get_contract_instance().await;

    let timestamp = _instance.methods().get_timestamp().call().await;
    println!("timestamp: {}", timestamp.unwrap().value);
}

#[tokio::test]
async fn should_return_new_timestamp_after_fast_forward() {
    let (_instance, _id, provider) = get_contract_instance().await;

    let timestamp = _instance
        .methods()
        .get_timestamp()
        .call()
        .await
        .unwrap()
        .value;

    fast_forward_time(&provider, Duration::from_secs(100))
        .await
        .unwrap();

    let new_timestamp = _instance
        .methods()
        .get_timestamp()
        .call()
        .await
        .unwrap()
        .value;
    println!("timestamp: {}", timestamp);
    println!("new_timestamp: {}", new_timestamp);
    assert!(new_timestamp == timestamp + 100);
}

#[tokio::test]
async fn can_check_if_timestamp_is_later() {
    let (_instance, _id, _provider) = get_contract_instance().await;

    let timestamp = _instance
        .methods()
        .get_timestamp()
        .call()
        .await
        .unwrap()
        .value;

    fast_forward_time(&_provider, Duration::from_secs(100))
        .await
        .unwrap();

    // check if timestamp is later than current timestamp, if it isn't it will reject
    let is_later = _instance
        .methods()
        .current_timestamp_later_than(timestamp)
        .call()
        .await
        .unwrap();

    assert!(is_later.value);
}

// fast forward by a certain duration
pub async fn fast_forward_time(provider: &Provider, increased_duration: Duration) -> Result<()> {
    let current_time = provider.latest_block_time().await.unwrap().unwrap();
    let date_time = chrono::DateTime::from_timestamp(
        current_time.timestamp() + increased_duration.as_secs() as i64,
        0,
    );

    provider.produce_blocks(3, date_time).await?;

    Ok(())
}
