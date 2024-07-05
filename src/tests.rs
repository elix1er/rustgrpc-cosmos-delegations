use crate::fetch_validator_delegations;
use cosmos_sdk_proto::cosmos::staking::v1beta1::query_client::QueryClient as StakingQueryClient;
use tonic::transport::Channel;

#[tokio::test]
async fn test_fetch_validator_delegations() {
    let endpoint = Channel::builder(
        "https://full-node.mainnet-1.coreum.dev:9090"
            .parse()
            .unwrap(),
    );
    let channel = endpoint.connect().await.expect("Failed to connect");
    let mut staking_client = StakingQueryClient::new(channel);

    let result = fetch_validator_delegations(&mut staking_client).await;
    assert!(result.is_ok(), "Failed to fetch validator delegations");

    let delegations = result.unwrap();
    assert!(!delegations.is_empty(), "No delegations returned");

    // Check the first delegation
    let first_delegation = &delegations[0];
    assert!(first_delegation.delegation.is_some(), "Delegation is None");
    assert!(first_delegation.balance.is_some(), "Balance is None");

    let delegation = first_delegation.delegation.as_ref().unwrap();
    assert!(
        !delegation.delegator_address.is_empty(),
        "Delegator address is empty"
    );
    assert!(
        !delegation.validator_address.is_empty(),
        "Validator address is empty"
    );
    assert!(!delegation.shares.is_empty(), "Shares is empty");

    let balance = first_delegation.balance.as_ref().unwrap();
    assert_eq!(balance.denom, "ucore", "Unexpected denom");
    assert!(!balance.amount.is_empty(), "Amount is empty");

    println!("Fetched {} delegations", delegations.len());
}
