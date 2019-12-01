table! {
    article (id) {
        id -> Nullable<Integer>,
        title -> Text,
        brief -> Text,
        content -> Text,
        category -> Text,
        labels -> Text,
        date -> Timestamp,
    }
}

table! {
    comment (id) {
        id -> Nullable<Integer>,
        article_id -> Integer,
        name -> Text,
        email -> Text,
        website -> Text,
        content -> Text,
        avatar -> Text,
        date -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(article, comment,);
