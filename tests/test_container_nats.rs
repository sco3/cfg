use std::time::Duration;

use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};

#[tokio::test]
async fn test_nats() {
    println!("Start Nats container");

    let image = GenericImage::new("nats", "latest")
        .with_exposed_port(4222.tcp())
        .with_wait_for(WaitFor::Duration {
            length: Duration::from_millis(10),
        })
        .start()
        .await
        .unwrap_or_else(|e| panic!("Failed to start container: {:?}", e));

    let port = image
        .get_host_port_ipv4(4222)
        .await
        .unwrap_or_else(|e| panic!("Failed to get port: {:?}", e));

    println!("Nats port: {}", port);

    //TODO
}
