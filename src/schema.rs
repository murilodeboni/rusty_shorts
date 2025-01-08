// @generated automatically by Diesel CLI.

diesel::table! {
    employee (emp_id) {
        emp_id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 1]
        gender -> Nullable<Bpchar>,
        birthdate -> Nullable<Date>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        salary -> Nullable<Int4>,
    }
}

diesel::table! {
    urls (id) {
        id -> Int4,
        slug -> Varchar,
        original_url -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    employee,
    urls,
);
