use anyhow::Result;
use models::{Household, HouseholdRecord, Person};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, BatchWriteItemInput, DynamoDb, DynamoDbClient, PutRequest, QueryInput,
    WriteRequest,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct HouseholdService {
    client: DynamoDbClient,
}

impl HouseholdService {
    pub fn new() -> HouseholdService {
        let region = Region::Custom {
            name: "us-east-1".to_owned(),
            endpoint: "http://localhost:8000".to_owned(),
        };

        HouseholdService {
            client: DynamoDbClient::new(region),
        }
    }

    pub async fn put(self, people: Vec<Person>) -> Result<Household> {
        let household = Household {
            id: Uuid::new_v4(),
            people: people.clone(),
        };

        let put_requests: Vec<WriteRequest> = household
            .people
            .iter()
            .map(|person| WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(&HouseholdRecord {
                        household_id: household.id,
                        contact: person.contact.clone(),
                        name: person.name.clone(),
                        rsvp: person.rsvp.clone(),
                    })
                    .unwrap(),
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

        self.client
            .batch_write_item(batch_write_request_input)
            .await?;
        Ok(household)
    }

    pub async fn get(self, household_id: Uuid) -> Result<Option<Household>> {
        let mut query = HashMap::new();
        query.insert(
            String::from(":household_id"),
            AttributeValue {
                s: Some(household_id.to_string()),
                ..Default::default()
            },
        );

        let query_input = QueryInput {
            table_name: "rsvp_table".to_string(),
            key_condition_expression: Some("household_id = :household_id".to_string()),
            expression_attribute_values: Some(query),
            ..QueryInput::default()
        };

        let response = self.client.query(query_input).await?;
        if let Some(items) = response.items {
            if items.len() > 0 {
                let household_records: Vec<HouseholdRecord> = items
                    .into_iter()
                    .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
                    .collect();
                let household_id: Uuid = household_records[0].household_id;

                let people: Vec<Person> = household_records
                    .into_iter()
                    .map(|record| Person {
                        name: record.name,
                        contact: record.contact,
                        rsvp: record.rsvp,
                    })
                    .collect();

                return Ok(Some(Household {
                    id: household_id,
                    people: people,
                }));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::HouseholdService;
    use models::{Contact, Person};
    use uuid::Uuid;

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
                    value: "hello@example.com".to_string(),
                },
                rsvp: None,
            },
            Person {
                name: "Sally".to_string(),
                contact: Contact::SMS {
                    value: "5555555555".to_string(),
                },
                rsvp: None,
            },
        ];

        let household = service.put(people).await.unwrap();
        assert_eq!(household.people.len(), 2 as usize);
    }

    #[tokio::test]
    async fn it_should_get_a_household() {
        let people = vec![
            Person {
                name: "John".to_string(),
                contact: Contact::Email {
                    value: "hello@example.com".to_string(),
                },
                rsvp: None,
            },
            Person {
                name: "Sally".to_string(),
                contact: Contact::SMS {
                    value: "5555555555".to_string(),
                },
                rsvp: None,
            },
        ];
        let put_household = HouseholdService::new().put(people).await.unwrap();

        match HouseholdService::new().get(put_household.id).await.unwrap() {
            Some(household) => assert_eq!(household.people.len(), 2 as usize),
            None => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_should_not_get_a_household() {
        match HouseholdService::new().get(Uuid::new_v4()).await.unwrap() {
            Some(_household) => assert!(false),
            None => assert!(true),
        }
    }
}
