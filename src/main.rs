use actix_web::{
    get,
    middleware::{self, Logger},
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use serde_derive::Deserialize;
use std::str;

#[derive(Debug, Deserialize)]
pub struct Params {
    #[allow(non_camel_case_types)]
    id: u64,
}

mod parsemesh;
use parsemesh::rbxmesh_parser;

#[get("/asset")]
async fn asset(params: web::Query<Params>) -> impl Responder {
    let resp = reqwest::get(format!(
        "https://assetdelivery.roblox.com/v2/assetId/{}",
        params.id
    ))
    .await
    .unwrap()
    .json::<serde_json::Value>()
    .await
    .unwrap();

    #[cfg(debug_assertions)]
    println!("{:#?}", resp.get("assetTypeId"));

    if resp.get("assetTypeId").is_none() {
        return HttpResponse::Found()
            .insert_header((
                "Location",
                format!("https://assetdelivery.roblox.com/v1/asset?id={}", params.id),
            ))
            .finish();
    }
    let mesh_asset_id = 4;

    if resp.get("assetTypeId").unwrap() == mesh_asset_id {
        let client = reqwest::Client::new();
        let asset_buffer = client
            .get(
                resp.get("locations").unwrap()[0]
                    .get("location")
                    .unwrap()
                    .as_str()
                    .unwrap(),
            )
            .header("User-Agent", "Roblox/WinInet")
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        use std::time::Instant;
        let now: Instant;

        #[cfg(debug_assertions)]
        {
            now = Instant::now();
        }

        let file = rbxmesh_parser::parse(&mut asset_buffer.to_vec());

        #[cfg(debug_assertions)]
        {
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
        }

        if file.is_some() {
            return HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(file.unwrap());
        }
    }

    HttpResponse::Found()
        .insert_header((
            "Location",
            format!("https://assetdelivery.roblox.com/v1/asset?id={}", params.id),
        ))
        .finish()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Assetdelivery service started... made by sushi :D");

    /*
    let file = std::fs::read("input.mesh").unwrap();

    let file = rbxmesh_parser::parse(&mut file.to_vec());

    if file.is_some() {
        std::fs::write("2.00.mesh", file.unwrap()).unwrap();
    }  dirty parser tester */

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(asset)
            .wrap(middleware::NormalizePath::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
