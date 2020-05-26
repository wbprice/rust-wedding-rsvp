use anyhow::{bail, Result};
use models::Person;
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, BatchWriteItemInput, DynamoDb, DynamoDbClient, PutRequest, QueryInput,
    WriteRequest,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct HouseholdService {
    client: Box<DynamoDbClient>,
}

impl HouseholdService {
    fn client() -> DynamoDbClient {
        let region = Region::Custom {
            name: "us-east-1".to_owned(),
            endpoint: "http://localhost:8000".to_owned(),
        };

        DynamoDbClient::new(region)
    }

    pub async fn create(self, people: Vec<Person>) -> Result<Vec<Person>> {
        let client = self::client();
        if people.is_empty() {
            bail!("A household must have at least one person")
        }

        let put_requests: Vec<WriteRequest> = people
            .iter()
            .map(|person| WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(person).unwrap(),
                }),
                ..WriteRequest::default()
            })
            .collect();

        let mut request_items: HashMap<String, Vec<WriteRequest>> = HashMap::new();
        request_items.insert("rsvp_table".to_string(), put_requests);

        let batch_write_request_input = BatchWriteItemInput {
            request_items,
            ..BatchWriteItemInput::default()
        };

        self.client
            .batch_write_item(batch_write_request_input)
            .await?;

        Ok(people)
    }

    pub async fn update(self, household_id: Uuid, people: Vec<Person>) -> Result<Vec<Person>> {
        // Only records that exist can be updated
        match self.read(household_id).await? {
            Some(_) => {
                self.create(people.clone()).await?;
                Ok(people)
            },
            None => {
                bail!("That household doesn't exist")
            }
        }
    }

    pub async fn read(self, household_id: Uuid) -> Result<Option<Vec<Person>>> {
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
            if items.is_empty() {
                return Ok(None);
            }

            let people: Vec<Person> = items
                .into_iter()
                .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
                .collect();

            return Ok(Some(people));
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
    async fn it_should_create_a_household() {
        let service = HouseholdService::new();
        let household_id = Uuid::new_v4();
        let people = vec![
            Person {
                household_id,
                name: "John".to_string(),
                contact: Contact::Email {
                    value: "hello@example.com".to_string(),
                },
                rsvp: None,
                dietary_restrictions: None,
                dish_preference: None,
            },
            Person {
                household_id,
                name: "Sally".to_string(),
                contact: Contact::SMS {
                    value: "5555555555".to_string(),
                },
                rsvp: None,
                dietary_restrictions: None,
                dish_preference: None,
            },
        ];

        let guests = service.create(people).await.unwrap();
        assert_eq!(guests.len(), 2 as usize);
    }

    #[tokio::test]
    async fn it_should_get_a_household() {
        let household_id = Uuid::new_v4();
        let people = vec![
            Person {
                household_id,
                name: "John".to_string(),
                contact: Contact::Email {
                    value: "hello@example.com".to_string(),
                },
                rsvp: None,
                dietary_restrictions: None,
                dish_preference: None,
            },
            Person {
                household_id,
                name: "Sally".to_string(),
                contact: Contact::SMS {
                    value: "5555555555".to_string(),
                },
                rsvp: None,
                dietary_restrictions: None,
                dish_preference: None,
            },
        ];
        HouseholdService::new().create(people).await.unwrap();

        match HouseholdService::new().read(household_id).await.unwrap() {
            Some(guests) => assert_eq!(guests.len(), 2 as usize),
            None => assert!(false),
        }
    }

    #[tokio::test]
    async fn it_should_not_get_a_household() {
        match HouseholdService::new().read(Uuid::new_v4()).await.unwrap() {
            Some(_household) => assert!(false),
            None => assert!(true),
        }
    }

    #[tokio::test]
    async fn it_should_update_a_household() {
        let household_id = Uuid::new_v4();
        let people = vec![
            Person {
                household_id,
                name: "John".to_string(),
                contact: Contact::Email {
                    value: "hello@example.com".to_string(),
                },
                rsvp: None,
                dietary_restrictions: None,
                dish_preference: None,
            },
            Person {
                household_id,
                name: "Sally".to_string(),
                contact: Contact::SMS {
                    value: "5555555555".to_string(),
                },
                rsvp: None,
                dietary_restrictions: None,
                dish_preference: None,
            },
        ];

        let new_people = vec![
            Person {
                household_id,
                name: "John".to_string(),
                contact: Contact::Email {
                    value: "hello@example.com".to_string(),
                },
                rsvp: Some(true),
                dietary_restrictions: None,
                dish_preference: None,
            },
            Person {
                household_id,
                name: "Sally".to_string(),
                contact: Contact::SMS {
                    value: "5555555555".to_string(),
                },
                rsvp: Some(true),
                dietary_restrictions: None,
                dish_preference: None,
            }
        ];

        HouseholdService::new().create(people).await.unwrap();
        HouseholdService::put(new_people).await.unwrap();

    }
}
