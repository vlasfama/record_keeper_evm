// @generated automatically by Diesel CLI.
diesel::table! {
    user_records (id) {
        id -> Int4,
        users -> Jsonb,
        random_counter -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int2,
        user_age -> Int2,
        user_credit_balance -> Int2,
        user_is_member -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(user_records, users,);
