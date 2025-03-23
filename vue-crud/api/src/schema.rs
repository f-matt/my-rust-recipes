use diesel::table;

table! {
    rust_foos (id) {
        id -> Integer,
        description -> Varchar,
    }
}

