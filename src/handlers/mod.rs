use crate::{db, errors::MyError, models::Link};
use actix_web::{web, web::Redirect, Error, HttpRequest, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn shorten(
    req: HttpRequest,
    db_pool: web::Data<Pool>,
    body: String,
) -> Result<HttpResponse, Error> {
    let link_info = Link::new(body);

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_link = db::add_link(&client, link_info).await?;

    Ok(HttpResponse::Ok().body(format!(
        "{}://{}/l/{}",
        req.connection_info().scheme(),
        req.connection_info().host(),
        new_link.slug
    )))
}

pub async fn redirect(
    db_pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<web::Redirect, Error> {
    let slug = path.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let link = db::find_link_by_slug(&client, slug).await?;

    Ok(Redirect::to(link.url).temporary())
}
