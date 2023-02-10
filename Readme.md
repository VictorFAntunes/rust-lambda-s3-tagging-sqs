# Rust-based S3 Object Validation with AWS Lambda

This project showcases a basic implementation of a Lambda function written in Rust, using the AWS SDK for Rust (aws-sdk-rust) to interact with various AWS services. The main objective of the function is to validate the metadata of a sample .txt file placed in an S3 bucket and to provide feedback on its validity on some arbitrary criteria.

## Functionality

The function is triggered by an S3 event and receives the event object as its input. It performs several operations, including:

1 - Deserializing the event using the S3Event library.

2 - Extracting the relevant information from the S3 event, specifically the object attributes.

3 - Adding the "validating" tag to the object to indicate that a validation operation is in progress.

4 - Performing validation checks on the .txt file to ensure it is not zero bytes, conforms to a specific numerical code, and is indeed a .txt file.

    a) If the validation is successful, the function adds the "valid" tag to the object, creates a success message with the validation result and object identification, and sends the message to a success SQS queue.

    b) If the validation is unsuccessful, the function adds the "quarantine" tag to the object, creates a failure message with the validation result and object identification, and sends the message to a failure SQS queue.

5 - Returning a response indicating the result of the validation.

## Benefits of using Rust in AWS Lambda

Rust is a fast and efficient language that is well-suited for deployment in AWS Lambda. Its fast cold-start times and robust AWS support make it an excellent choice for handling time-sensitive operations, such as file validation in this case.

This code serves as a basic example of how Rust can be utilized in AWS Lambda functions to perform efficient and scalable validation on S3 objects.

## Easily Deploying to AWS with SAM

This Rust-based Lambda function can be easily deployed to AWS using the AWS Serverless Application Model (SAM). Here's what you need to do:

1 - Install [Rust](https://www.rust-lang.org/learn/get-started), [cargo-lambda](https://www.cargo-lambda.info), the [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html#install-sam-cli-instructions).

2 - Clone this repository and navigate to its root directory.

3 - In the terminal, run the following commands to build and deploy the function using SAM:

```
sam build
sam deploy guided (--profile <PROFILE_NAME> (optional))
```


Note: The --profile flag allows you to specify the AWS profile you want to use for deployment.

Once the deployment is complete, the function will be live and available to be triggered by S3 events in your AWS account.
To delete the deployed environment, simply run:

```
sam delete
```

Note: The S3 bucket created during the deployment is versioned, so make sure to delete all versions of the files used for testing before deleting the environment.



