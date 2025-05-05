use std::time::Duration;

use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};

#[tokio::test]
async fn test_nats() {
    let image = GenericImage::new("nats", "latest")
        .with_exposed_port(4222.tcp())
        .with_wait_for(WaitFor::Duration {
            length: Duration::from_millis(10),
        })
        .start()
        .await;
    println!("-= ready =-");
    match image {
        Ok(cnt) => {
            let port_result = cnt.get_host_port_ipv4(4222).await;
            match port_result {
                Ok(port) => {
                    println!("{}", port);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
