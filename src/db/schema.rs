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
        sequence -> Integer,
        body -> Text,
        created_at ->  Datetime,
        updated_at -> Datetime,
    }
}

