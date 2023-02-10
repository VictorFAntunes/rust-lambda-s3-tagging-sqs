use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_sqs::Client as SqsClient;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rust_lambda_s3_tagging_sqs::{add_tag, is_valid_file, single_tag};
use serde::{Deserialize, Serialize};
use tracing::info;

// Define a struct to represent the response of the function
#[derive(Serialize, Debug)]
pub struct Response {
    pub req_id: String,
    pub message: String,
}

// Create a struct to generate the message body
// Could add a don't Deserialize if null. to decrease the size of the message.
#[derive(Serialize, Deserialize, Debug)]
struct ValidationMessageBody {
    workflow: String,
    exc_id: String,
    categories: Vec<String>,
    message: String,
    continue_url: Option<String>,
    abort_url: Option<String>,
}

// Obtain the Success/Failure SQS queue URLs from environment variables
async fn function_handler(
    event: LambdaEvent<S3Event>,
    s3_client: &S3Client,
    sqs_client: &SqsClient,
) -> Result<Response, Error> {
    // Obtain the Success/Failure SQS queue from env.

    let success_queue_url = std::env::var("SUCCESS_QUEUE_URL")
        .map_err(|_| Error::from("Missing SUCCESS_QUEUE_URL environment variable"))?;
    //
    let failure_queue_url = std::env::var("FAILURE_QUEUE_URL")
        .map_err(|_| Error::from("Missing FAILURE_QUEUE_URL environment variable"))?;

    // Because the S3 bucket is using versioning, we need the file key and version number
    // to operate on the correct file. We can get this information from the S3Object struct in the
    // event payload and validate it.

    let event_s3_attributes = event
        .payload
        .records
        .first()
        .ok_or("No records found in event")?
        .s3
        .to_owned();

    //Add a tag "validating" to the file in order to allow for observability from outside the bucket.
    single_tag(&event_s3_attributes, s3_client, "validating").await?;

    // Start by validating the file using the object attributes from the event payload.

    // Check if the file type is .txt for tests
    // Check if the file is not zero bytes
    // Check if the file name without the extension is conformant with a particular code
    let (file_valid, validation_message) = is_valid_file(&event_s3_attributes);

    // If everything is okay, send a message to the success queue with the file identification

    // If one or more things are wrong, compose a general message to send to the failure queue
    // Add a quarantine tag to the file is something is wrong

    if file_valid {
        // File is valid, continue with processing
        info!("{}", &validation_message);

        single_tag(&event_s3_attributes, s3_client, "validated").await?;

        add_tag(&event_s3_attributes, s3_client, "valid").await?;

        let success_message = ValidationMessageBody {
            workflow: "Validation_Workflow".to_string(),
            exc_id: event.context.request_id.to_owned(),
            categories: vec!["CD-TECH".to_string(), "AM-DEVS".to_string()],
            message: validation_message.clone(),
            continue_url: None,
            abort_url: None,
        };

        sqs_client
            .send_message()
            .queue_url(success_queue_url)
            .message_body(serde_json::to_string(&success_message)?)
            .message_group_id("ValidationGroup".to_string())
            .send()
            .await?;

        Ok(Response {
            req_id: event.context.request_id,
            message: validation_message,
        })
    } else {
        info!("File is invalid: {}", &validation_message);
        single_tag(&event_s3_attributes, s3_client, "validated").await?;

        add_tag(&event_s3_attributes, s3_client, "quarentine").await?;

        let failure_message = ValidationMessageBody {
            workflow: "Validation_Workflow".to_string(),
            exc_id: event.context.request_id.to_owned(),
            categories: vec!["CD-TECH".to_string(), "AM-DEVS".to_string()],
            message: validation_message.clone(),
            continue_url: Some("https://example.com/continue".to_string()),
            abort_url: Some("https://example.com/abort".to_string()),
        };
        sqs_client
            .send_message()
            .queue_url(failure_queue_url)
            .message_body(serde_json::to_string(&failure_message)?)
            .message_group_id("ValidationGroup".to_string())
            .send()
            .await?;
        // File is invalid, return error message
        Ok(Response {
            req_id: event.context.request_id,
            message: validation_message,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create the Clients in main so it can be reused while the lambda is up
    //Get config from env
    let config = aws_config::load_from_env().await;
    // Create a new S3 client
    let s3_client = S3Client::new(&config);
    // Create a new SQS client
    let sqs_client = SqsClient::new(&config);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(|event: LambdaEvent<S3Event>| {
        function_handler(event, &s3_client, &sqs_client)
    }))
    .await
}
