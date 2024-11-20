use tonic::transport::Server;

use reflection::common::{get_env_var_str, init_cors_layer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get required environment variables
    let host = get_env_var_str("HOST");

    // configure gRPC reflection service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::websites::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::commerce::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::commerce::v2::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::media::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::payment::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::report::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::types::country::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::types::currency::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::types::query::v1::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            service_apis::sited_io::types::v1::FILE_DESCRIPTOR_SET,
        )
        .build_v1()
        .unwrap();

    let cors_layer = init_cors_layer();

    println!("gRPC+web server listening on {}", host);

    Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .add_service(tonic_web::enable(reflection_service))
        .serve(host.parse().unwrap())
        .await?;

    Ok(())
}
