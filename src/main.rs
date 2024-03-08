use actix_web::{web,App,HttpRequest,HttpServer,Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("hello {}!", &name)
}

#[tokio::maint]
async fn main () -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/",web::get().to(greetings))
            .route("/{name}", web::get().to(greet))
    })
      .blind("127.0.0.1:8000")?
      .run()
      .await
}


#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        //this requires changing the return type of `health_check`
        //from `impl Responder` to `HttoResponse` to compile
        //You also need to import it with `use actix_web::HttpResponse`!
        assert!(response.status().is_success())
    }
}

