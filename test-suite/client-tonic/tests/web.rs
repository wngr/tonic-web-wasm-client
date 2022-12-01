use client::proto::{echo_client::EchoClient, EchoRequest};


#[tokio::test]
async fn test_echo() {
    let mut client = EchoClient::default();

    let response = client
        .echo(EchoRequest {
            message: "John".to_string(),
        })
        .await
        .expect("success response")
        .into_inner();

    assert_eq!(response.message, "echo(John)");
}

#[tokio::test]
async fn test_echo_stream() {
    let mut client = EchoClient::default();

    let mut stream_response = client
        .echo_stream(EchoRequest {
            message: "John".to_string(),
        })
        .await
        .expect("success stream response")
        .into_inner();

    for i in 0..3 {
        let response = stream_response.message().await.expect("stream message");
        assert!(response.is_some(), "{}", i);
        let response = response.unwrap();

        assert_eq!(response.message, "echo(John)");
    }

    let response = stream_response.message().await.expect("stream message");
    assert!(response.is_none());
}

#[tokio::test]
async fn test_infinite_echo_stream() {
    let mut client = EchoClient::default();

    let mut stream_response = client
        .echo_infinite_stream(EchoRequest {
            message: "John".to_string(),
        })
        .await
        .expect("success stream response")
        .into_inner();

    for i in 0..3 {
        let response = stream_response.message().await.expect("stream message");
        assert!(response.is_some(), "{}", i);
        let response = response.unwrap();

        assert_eq!(response.message, format!("echo(John, {})", i + 1));
    }

    let response = stream_response.message().await.expect("stream message");
    assert!(response.is_some());
}

#[tokio::test]
async fn test_echo_pending() {
    let mut client = EchoClient::default();

    client
        .echo_stream(EchoRequest {
            message: "SILENCE!".to_string(),
        })
        .await
        .expect("success stream response")
        .into_inner();
}
