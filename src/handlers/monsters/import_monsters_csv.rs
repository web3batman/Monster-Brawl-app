use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse};
use futures::TryStreamExt;
use std::io::Write;
use tempfile::NamedTempFile;

use crate::{
    domain::models::monster::Monster,
    infra::{db::database::Database, repositories::monsters},
    Response,
};

#[post("/monsters/import_monsters_csv")]
pub async fn import_monsters_csv(
    db: web::Data<Database>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut file_name: Option<String> = None;
    let mut temp_file: Option<NamedTempFile> = None;
    let mut new_monsters: Vec<Monster> = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        if let Some(name) = content_disposition.get_filename() {
            file_name = Some(name.to_string());
            temp_file = Some(NamedTempFile::new().unwrap());

            while let Some(chunk) = field.try_next().await? {
                temp_file.as_mut().unwrap().write_all(&chunk).unwrap();
            }
        } else {
            return Ok(HttpResponse::BadRequest().json(Response {
                status: "error".to_string(),
                message: "No file name provided".to_string(),
            }));
        }
    }

    if let Some(_file_name) = file_name {
        if let Some(temp_file) = temp_file {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_path(temp_file.path())
                .unwrap();

            for result in reader.deserialize::<Monster>() {
                match result {
                    Ok(monster) => {
                        new_monsters.push(monster);
                    }
                    Err(_) => {
                        return Ok(HttpResponse::BadRequest().json(Response {
                            status: "error".to_string(),
                            message: "Incomplete data, check your file.".to_string(),
                        }));
                    }
                }
            }

            if new_monsters.is_empty() {
                return Ok(HttpResponse::BadRequest().json(Response {
                    status: "error".to_string(),
                    message: "No valid monsters found in the CSV file".to_string(),
                }));
            }

            let results: Vec<Result<Monster, String>> = new_monsters
                .iter()
                .map(
                    |new_monster| match monsters::create_monster(&db, new_monster.clone()) {
                        Ok(monster) => Ok(monster),
                        Err(err) => Err(err.to_string()),
                    },
                )
                .collect();

            let (successes, _errors): (Vec<_>, Vec<_>) =
                results.into_iter().partition(Result::is_ok);

            let successful_monsters: Vec<Monster> =
                successes.into_iter().map(Result::unwrap).collect();

            if successful_monsters.is_empty() {
                return Ok(HttpResponse::InternalServerError().json(Response {
                    status: "error".to_string(),
                    message: "Failed to create any monsters".to_string(),
                }));
            } else {
                return Ok(HttpResponse::Ok().json(successful_monsters));
            }
        }
    }

    Ok(HttpResponse::BadRequest().json(Response {
        status: "error".to_string(),
        message: "No file uploaded".to_string(),
    }))
}
