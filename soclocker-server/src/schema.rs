//! This file automatically generates the configuration for the database and how
//! it is interacted with by the Diesel crate.
#![allow(non_snake_case, missing_docs)]

table! {
    Users (ID) {
        ID -> Integer,
        PublicKey -> Text,
        Username -> Text,
    }
}

table! {
    Auth (PublicKey) {
        PublicKey -> Text,
        ExpectedToken -> Text,
        Timeout -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(Users, Auth);
