use self::proto::echo_client::EchoClient;
use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("echo");
}

impl Default for EchoClient<Channel> {
    fn default() -> Self {
        let base_url = "http://localhost:50051".to_string();
        let wasm_client = Channel::builder(base_url.parse().unwrap()).connect_lazy();

        EchoClient::new(wasm_client)
    }
}
