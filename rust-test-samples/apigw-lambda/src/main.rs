use lambda_http::{run, service_fn, http::StatusCode, IntoResponse, Request, Error, Response};
use aws_sdk_s3::{Client};
use serde::{Serialize};
use serde_json::json;

#[derive(Serialize, Debug)]
struct Message {
    list: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    const REGION: &str = "eu-west-1";
    let shared_config = aws_config::from_env().region(REGION).load().await;
    let client = Client::new(&shared_config);

    run(service_fn(|_request: Request| {
        handler(&client)
    }))
    .await?;

    Ok(())

}

async fn handler(client: &Client) -> Result<impl IntoResponse, Error> {

    let s3_buckets = client.list_buckets().send().await?;
    let s3_result = match s3_buckets.buckets {
        Some(_) => s3_buckets.buckets(),
        None => return Ok(Response::builder().status(500).body("s3 call failed".into())?),
    };
    
    let buckets = s3_result.unwrap_or_default();
    let mut buckets_list = vec![];

    for bucket in buckets {
        buckets_list.push(bucket.name().unwrap_or_default());
    }

    let response = Message {list: buckets_list.join(" | ")};

    Ok(Response::builder().status(StatusCode::OK).body(json!(&response).to_string())?)

}


/*#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_dynamodb::{Client, Config, Credentials, Region};
    use aws_smithy_client::{erase::DynConnector, test_connection::TestConnection};
    use aws_smithy_http::body::SdkBody;
    use std::collections::HashMap;

    // Helper function to create a mock AWS configuration
    async fn get_mock_config(conn: &TestConnection<SdkBody>) -> Config {
        let cfg = aws_config::from_env()
            .region(Region::new("eu-west-1"))
            .http_connector(DynConnector::new(conn.clone()))
            .credentials_provider(Credentials::new(
                "access_key",
                "privatekey",
                None,
                None,
                "dummy",
            ))
            .load()
            .await;

        Config::new(&cfg)
    }

    /// Helper function to generate a sample DynamoDB request
    fn get_request_builder() -> http::request::Builder {
        http::Request::builder()
            .header("content-type", "application/x-amz-json-1.0")
            .uri(http::uri::Uri::from_static(
                "https://dynamodb.eu-west-1.amazonaws.com/",
            ))
    }

    #[tokio::test]
    async fn test_put_item() {
        // Mock DynamoDB client
        //
        // `TestConnection` takes a vector of requests and responses, allowing us to
        // simulate the behaviour of the DynamoDB API endpoint. Since we are only
        // making a single request in this test, we only need to provide a single
        // entry in the vector.
        let conn = TestConnection::new(vec![(
            get_request_builder()
                .header("x-amz-target", "DynamoDB_20120810.PutItem")
                .body(SdkBody::from(
                    r#"{"TableName":"test","Item":{"id":{"S":"1"},"payload":{"S":"test1"}}}"#,
                ))
                .unwrap(),
            http::Response::builder()
                .status(200)
                .body(SdkBody::from(
                    r#"{"Attributes": {"id": {"S": "1"}, "payload": {"S": "test1"}}}"#,
                ))
                .unwrap(),
        )]);
        let client = Client::from_conf(get_mock_config(&conn).await);

        let table_name = "test_table";

        // Mock API Gateway request
        let mut path_parameters = HashMap::new();
        path_parameters.insert("id".to_string(), vec!["1".to_string()]);

        let request = http::Request::builder()
            .method("PUT")
            .uri("/1")
            .body(Body::Text("test1".to_string()))
            .unwrap()
            .with_path_parameters(path_parameters);

        // Send mock request to Lambda handler function
        let response = put_item(&client, table_name, request)
            .await
            .unwrap();
        
        // Assert that the response is correct
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), &Body::Text("item saved".to_string()));
    }
}*/