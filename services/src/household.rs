use rusoto::DynamoDBClient::{DynamoDb, DynamoDBClient};
use crate::models::Household;
use uuid::Uuid;
use anyhow::Result;

pub struct HouseholdService {
    client: DynamoDbClient
}

impl HouseholdService {

    pub fn new() -> HouseholdService {
        HouseholdService {
            client: DynamoDbClient::new("us-east-1")
        }
    }

    pub async fn put(household: Household) -> Result<Household> {

    }

    pub async fn get(uuid: Uuid) -> Result<Household> {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_create_a_service() {
        let householdService = HouseholdService::new();
    }
}