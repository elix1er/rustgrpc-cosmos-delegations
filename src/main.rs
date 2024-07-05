#[cfg(test)]
mod tests;

use cosmos_sdk_proto::cosmos::{
    base::query::v1beta1::PageRequest,
    staking::v1beta1::{
        query_client::QueryClient as StakingQueryClient, DelegationResponse,
        QueryValidatorDelegationsRequest,
    },
};
use csv::Writer;
use std::fs::create_dir_all;
use std::net::AddrParseError;
use thiserror::Error;
use tonic::transport::Channel;

#[derive(Error, Debug)]
enum AppError {
    #[error("Failed to fetch validator delegations: {0}")]
    FetchDelegationsError(#[from] tonic::Status),
    #[error("Failed to create CSV: {0}")]
    CsvCreationError(#[from] csv::Error),
    #[error("Failed to create directory: {0}")]
    DirectoryCreationError(#[from] std::io::Error),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] AddrParseError),
}

async fn fetch_validator_delegations(
    staking_client: &mut StakingQueryClient<Channel>,
) -> Result<Vec<DelegationResponse>, AppError> {
    println!("Fetching validator delegations");
    let request = tonic::Request::new(QueryValidatorDelegationsRequest {
        validator_addr: "corevaloper1q69hxadh5pl9d6q0a35y3mfx8ls5quxuqgla8t".to_string(),
        pagination: Some(PageRequest {
            count_total: true,
            limit: 10000,
            ..Default::default()
        }),
    });

    let response = staking_client.validator_delegations(request).await?;
    let response_inner = response.into_inner();

    println!("Pagination: {:?}", response_inner.pagination);
    Ok(response_inner.delegation_responses)
}

fn create_csv(delegations: Vec<DelegationResponse>) -> Result<(), AppError> {
    create_dir_all("output")?;
    let mut wtr = Writer::from_path("output/validator_delegations.csv")?;
    wtr.write_record(&[
        "Delegator Address",
        "Validator Address",
        "Shares",
        "Balance",
    ])?;
    // Write delegation snapshot data
    for delegation_response in delegations {
        if let Some(delegation) = delegation_response.delegation {
            if let Some(balance) = delegation_response.balance {
                wtr.write_record(&[
                    &delegation.delegator_address,
                    &delegation.validator_address,
                    &delegation.shares.to_string(),
                    &balance.amount,
                ])?;
            }
        }
    }
    wtr.flush()?;
    println!("CSV file created: Output/validator_delegations.csv");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let channel = Channel::from_static("https://full-node.mainnet-1.coreum.dev:9090")
        .connect()
        .await?;
    let mut staking_client = StakingQueryClient::new(channel);
    let delegations = fetch_validator_delegations(&mut staking_client).await?;
    create_csv(delegations)?;
    Ok(())
}
