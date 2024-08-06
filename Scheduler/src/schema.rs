table! {
    sessions (id) {
        id -> Uuid,
        title -> Text,
        speaker -> Text,
        time -> Timestamp,
        duration -> Integer,
        description -> Text,
    }
}