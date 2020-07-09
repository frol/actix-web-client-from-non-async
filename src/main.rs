fn main() {
    let body = actix::System::builder().build().block_on(async {
        let client = actix_web::client::Client::new();

        // Create request builder, configure request and send
        let mut response = client
            .get("https://www.rust-lang.org/")
            .header("User-Agent", "Actix-web")
            .send()
            .await
            .unwrap();

        // server http response
        println!("Response: {:?}", response);

        // read response body
        let body = response.body().await.unwrap();
        println!("Downloaded: {:?} bytes", body.len());
        body
    });

    println!("Body: {:?}", body);
}
