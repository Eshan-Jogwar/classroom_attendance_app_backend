use axum::{extract::Request, middleware::Next, response::Response};

pub async fn debugger_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    
    println!("API Hit: [{}] {}", method, path);


    let response = next.run(request).await;
    
    response
}
