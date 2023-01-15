use aws_config::meta::region::RegionProviderChain;
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
    
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    
    run(service_fn(|_request: Request| {
        handler(&client)
    }))
    .await
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
        buckets_list.push(bucket
                            .name().
                            unwrap_or_default());
    }

    let response = Message {list: buckets_list.join(" | ")};

    Ok(Response::builder().status(StatusCode::OK).body(json!(&response).to_string())?)

}

#[cfg(test)]
mod tests {
    use super::*;

    const buckets_list = "test1 | test2";
    const empty_list = "";

    //add S3 client mock and fake the methods used
    /*#[async_trait]
    trait TestMethods{
        async fn list_buckets(){

        }

        async fn send(){
            
        }
    }

    struct S3Buckets{

    }

    impl S3Buckets{

    }*/

    // Helper function to create a mock AWS configuration
    async fn get_mock_config() -> Config {
        let cfg = aws_config::from_env()
            .region(Region::new("eu-west-1"))
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

    async fn get_s3_client(config: &Config) -> Client {
        Client::new(&config);
    }

    #[tokio::test]
    async fn test_handler_200_populated() {
        const client = get_s3_client(get_mock_config());
        // Send mock request to Lambda handler function
        let response = handler(&client)
            .await
            .unwrap();
        
        // Assert that the response is 200 and returns the list of buckets
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), &Body::Text(&response_body));
    }

    #[tokio::test]
    async fn test_handler_200_empty() {
        const client = get_s3_client(get_mock_config());
        // Send mock request to Lambda handler function
        let response = handler(&client)
            .await
            .unwrap();
        
        // Assert that the response is 200 and returns an empty buckets list
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), &Body::Text(&empty_list));
    }

    #[tokio::test]
    async fn test_handler_500() {
        const client = get_s3_client(get_mock_config());
        // Send mock request to Lambda handler function
        let response = handler(&client)
            .await
            .unwrap();
        
        // Assert that the response is 500 and the body text returns the right error msg
        assert_eq!(response.status(), 500);
        assert_eq!(response.body(), &Body::Text("s3 call failed".into()));
    }

}