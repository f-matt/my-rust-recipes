use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::rust_foos;

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = rust_foos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Foo {
    pub id: i32,
    pub description: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rust_foos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFoo {
    pub description: String,
}

