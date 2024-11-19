use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use serde_json::json;

use crate::db::DbPool;
use crate::models::{File, NewFile};

pub async fn upload_file(mut payload: Multipart, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    if let Ok(Some(mut field)) = payload.try_next().await {
        let filename = field.content_disposition()
        .get_filename()
        .unwrap_or("unknown")
        .to_string();

        let content_type = field.content_type().unwrap().to_string();
        let mut data = Vec::new();

        while let Some(chunk) = field.next().await {
            data.extend_from_slice(&chunk?);
        }

        let file_id = Uuid::new_v4().to_string();
        let size = data.len() as i32;

        let new_file = NewFile {
            id: &file_id,
            filename: &filename,
            content_type: &content_type,
            size,
            data: &data,
        };

        let conn = &mut pool.get().expect("Failed to get db connection");
        diesel::insert_into(crate::schema::files::table)
        .values(&new_file)
        .execute(conn)
        .expect("Error saving file");

        Ok(HttpResponse::Ok().json(json!({
            "id": file_id,
            "url": format!("/files/{}", file_id)
        })))
    } else {
        Ok(HttpResponse::BadRequest().body("No file provided"))
    }
}

pub async fn download_file(path: web::Path<String>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let file_id = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get db connection");

    let file = crate::schema::files::table
    .find(file_id)
    .first::<File>(conn)
    .optional()
    .expect("Error loading file");

    match file {
        Some(file) => Ok(HttpResponse::Ok()
        .content_type(file.content_type)
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", file.filename)))
        .body(file.data)),
        None => Ok(HttpResponse::NotFound().body("File not found"))
    }
}
