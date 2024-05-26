// @generated automatically by Diesel CLI.

diesel::table! {
    battles (id) {
        id -> Varchar,
        monster_a -> Varchar,
        monster_b -> Varchar,
        winner -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    monsters (id) {
        id -> Varchar,
        name -> Varchar,
        image_url -> Varchar,
        attack -> Int4,
        defense -> Int4,
        hp -> Int4,
        speed -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(battles -> monsters (winner));

diesel::allow_tables_to_appear_in_same_query!(battles, monsters,);
