use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};

use crate::internal_error;
use crate::models::{Foo, NewFoo};
use crate::schema::rust_foos::dsl::*;

use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

pub async fn get_foos(State(pool): State<Pool>) -> Result<Json<Vec<Foo>>, (StatusCode, String)> {
    let connection = pool.get().await.map_err(internal_error)?;
    let results = connection
        .interact(|connection| rust_foos.select(Foo::as_select())
        .load(connection))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(results))
}

pub async fn insert_foo(
    State(pool): State<Pool>, 
    Json(new_foo): Json<NewFoo>,) -> Result<Json<Foo>, (StatusCode, String)> {
    let connection = pool.get().await.map_err(internal_error)?;
    let response = connection
        .interact(|connection| {
            diesel::insert_into(rust_foos)
                .values(new_foo)
                .returning(Foo::as_returning())
                .get_result(connection)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(response))
}

pub async fn update_foo(
    State(pool): State<Pool>, 
    Json(foo): Json<Foo>,) -> Result<Json<Foo>, (StatusCode, String)> {
    let connection = pool.get().await.map_err(internal_error)?;

    let response = connection
        .interact(move |connection| {
            diesel::update(rust_foos.filter(id.eq(foo.id))).set(&foo).get_result(connection)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(response))
}

pub async fn delete_foo(
    State(pool): State<Pool>,
    Json(foo): Json<Foo>) -> Result<Json<usize>, (StatusCode, String)> {
        let connection = pool.get().await.map_err(internal_error)?;

        let response = connection
            .interact(move |connection| {
                diesel::delete(rust_foos.filter(id.eq(foo.id))).execute(connection)
            })
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;
        Ok(Json(response))
    }
