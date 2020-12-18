table! {
    people (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        age -> Nullable<Int4>,
        city -> Nullable<Text>,
        country -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
