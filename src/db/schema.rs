table! {
    title (id) {
        id -> BigInt,
        name -> Varchar,
        created_at ->  Datetime,
        updated_at -> Datetime,
    }
}


table!{
     text (id) {
        id -> BigInt,
        title_id -> BigInt,
        parent_id -> Integer,
        body -> Text,
        author -> Varchar,
        created_at ->  Datetime,
        updated_at -> Datetime,
    }
}

