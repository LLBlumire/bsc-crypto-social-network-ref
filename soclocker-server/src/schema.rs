#![allow(non_snake_case)]

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
