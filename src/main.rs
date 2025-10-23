use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use pay_by_square_generator::{
    generate_code_only, generate_pay_by_square_qr, CodeResponse, PaymentRequest, QrOptions,
};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Embed frame.png at compile time
const FRAME_DATA: Option<&[u8]> = if cfg!(feature = "embed-frame") {
    Some(include_bytes!("../frame.png"))
} else {
    None
};

#[derive(OpenApi)]
#[openapi(
    paths(generate_qr, generate_code, version),
    components(schemas(
        PaymentRequest,
        pay_by_square_generator::BankAccount,
        pay_by_square_generator::PaymentOption,
        pay_by_square_generator::StandingOrder,
        pay_by_square_generator::DirectDebit,
        pay_by_square_generator::DirectDebitScheme,
        pay_by_square_generator::DirectDebitType,
        pay_by_square_generator::Periodicity,
        CodeResponse,
    )),
    tags(
        (name = "pay-by-square-generator", description = "PayBySquare QR code generator API")
    ),
    info(
        title = "PayBySquare Generator API",
        version = "1.0.0",
        description = "REST API for generating PayBySquare QR codes according to Slovak banking standard v1.1.0",
    )
)]
struct ApiDoc;

/// Generates a PayBySquare QR code image (PNG)
#[utoipa::path(
    post,
    path = "/pay-by-square-generator/generate-qr",
    tag = "pay-by-square-generator",
    request_body = PaymentRequest,
    responses(
        (status = 200, description = "QR code image generated successfully", content_type = "image/png"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/pay-by-square-generator/generate-qr")]
async fn generate_qr(payment: web::Json<PaymentRequest>) -> impl Responder {
    let opts = QrOptions {
        with_frame: true,
        qr_size: 300,
    };

    match generate_pay_by_square_qr(&payment, opts, FRAME_DATA) {
        Ok(png_data) => HttpResponse::Ok().content_type("image/png").body(png_data),
        Err(e) => e.error_response(),
    }
}

/// Generates a PayBySquare code as text string
#[utoipa::path(
    post,
    path = "/pay-by-square-generator/generate-code",
    tag = "pay-by-square-generator",
    request_body = PaymentRequest,
    responses(
        (status = 200, description = "Code generated successfully", body = CodeResponse),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/pay-by-square-generator/generate-code")]
async fn generate_code(payment: web::Json<PaymentRequest>) -> impl Responder {
    match generate_code_only(&payment) {
        Ok(code) => HttpResponse::Ok().json(CodeResponse { code }),
        Err(e) => e.error_response(),
    }
}

/// Returns the application version
#[utoipa::path(
    get,
    path = "/pay-by-square-generator/version.txt",
    tag = "pay-by-square-generator",
    responses(
        (status = 200, description = "Version information", content_type = "text/plain")
    )
)]
#[get("/pay-by-square-generator/version.txt")]
async fn version() -> impl Responder {
    let version = env!("CARGO_PKG_VERSION");
    HttpResponse::Ok().content_type("text/plain").body(version)
}

/// Health check endpoint
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "pay-by-square-generator"
    }))
}

/// Root redirect to documentation
#[get("/")]
async fn root_redirect() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/pay-by-square-generator/docs"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let bind_address = format!("0.0.0.0:{}", port);

    println!("🚀 Starting PayBySquare Generator API");
    println!(
        "📖 Documentation: http://localhost:{}/pay-by-square-generator/docs",
        port
    );
    println!("🔍 Health check: http://localhost:{}/health", port);
    println!("🎯 Listening on: {}", bind_address);

    HttpServer::new(|| {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(root_redirect)
            .service(health)
            .service(generate_qr)
            .service(generate_code)
            .service(version)
            .service(
                SwaggerUi::new("/pay-by-square-generator/docs/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
