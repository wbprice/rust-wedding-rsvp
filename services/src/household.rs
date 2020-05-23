use std::collections::HashMap;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, WriteRequest, PutRequest, BatchWriteItemInput, BatchWriteItemOutput};
use models::{Household, Person};
use uuid::Uuid;
use anyhow::Result;

pub struct HouseholdService {
    client: DynamoDbClient
}

impl HouseholdService {

    pub fn new() -> HouseholdService {
        let region = Region::Custom {
            name: "us-east-1".to_owned(),
            endpoint: "http://localhost:8000".to_owned(),
        };

        HouseholdService {
            client: DynamoDbClient::new(region)
        }
    }

    pub async fn put(self, people: Vec<Person>) -> Result<BatchWriteItemOutput> {
        let household = Household {
            id: Uuid::new_v4(),
            people: people.clone()
        };

        let put_requests: Vec<WriteRequest> = household
            .people
            .iter()
            .map(|person| WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(&person).unwrap(),
                }),
                ..WriteRequest::default()
            })
            .collect();

        let mut request_items: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        request_items.insert("rsvp_table".to_string(), put_requests);

        let batch_write_request_input = BatchWriteItemInput {
            request_items: request_items,
            ..BatchWriteItemInput::default()
        };

        Ok(self.client.batch_write_item(batch_write_request_input).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::HouseholdService;
    use models::{Person, Contact};

    #[tokio::test]
    async fn it_should_create_a_service() {
        let _household_service = HouseholdService::new();
        assert!(true);
    }

    #[tokio::test]
    async fn it_should_create_a_household() {
        let service = HouseholdService::new();
        let people = vec![
            Person {
                name: "John".to_string(),
                contact: Contact::Email {
                    value: "hello@example.com".to_string()
                },
                rsvp: None
            },
            Person {
                name: "Sally".to_string(),
                contact: Contact::SMS {
                    value: "5555555555".to_string()
                },
                rsvp: None
            }
        ];

        let household = service.put(people).await;
        match household {
            Ok(result) => {
                dbg!(result);
                assert!(true)
            },
            Err(err) => {
                dbg!(err);
                assert!(false)
            }
        };
    }
}