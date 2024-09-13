use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct People {
    people: Vec<Person>,
}

#[derive(Serialize, Deserialize)]
struct Optionals {
    option: Option<i64>,
}

fn typed_example(data: &str) -> Result<(), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

#[test]
fn it_works_with_simple_data() {
    let data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;
    let result = typed_example(data);
    assert!(result.is_ok());
}

#[test]
fn it_errs_if_data_is_wrong() {
    let data = r#"{
                    "name": "John Doe",
                    "age": "43",
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;
    let result = typed_example(data);
    assert!(result.is_err());
}

#[test]
fn it_nests_structures() {
    let data = r#"{
        "people": [
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            },
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }
        ]
    }"#;
    let result: Result<People, Error> = serde_json::from_str(data);
    assert!(result.is_ok());
}

#[test]
fn it_expands_vectors() {
    let data = r#"[
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        },
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
    ]"#;
    let result: Result<Vec<Person>, Error> = serde_json::from_str(data);
    assert!(result.is_ok());
}

#[test]
fn optional_values_can_be_left_out() {
    let data = r#"
        {
            "optional": 1
        }
    "#;
    let result: Result<Optionals, Error> = serde_json::from_str(data);
    assert!(result.is_ok());
    let data = r#"{}"#;
    let result: Result<Optionals, Error> = serde_json::from_str(data);
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert!(structure.option.is_none());
}
