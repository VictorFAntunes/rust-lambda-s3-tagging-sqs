mod generate_tags;

use crate::generate_tags::GenerateTags;
use aws_lambda_events::s3::S3Entity;
use aws_sdk_s3::model::Tagging;
use aws_sdk_s3::output::PutObjectTaggingOutput;
use aws_sdk_s3::Client as S3Client;
use lambda_runtime::Error;
use std::path::Path;

fn check_file_extension(s3_entity: &S3Entity) -> Option<String> {
    // Get the key of the object
    let key = match &s3_entity.object.key {
        Some(k) => k,
        None => return Some("Missing object key".to_string()),
    };

    // Get the file extension
    let file_extension = match Path::new(key).extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => return Some("Missing file extension".to_string()),
    };

    // Check if the file extension is .txt
    if file_extension != "txt" {
        return Some("Invalid file extension, should be .txt".to_string());
    }
    None
}

fn check_file_size(s3_entity: &S3Entity) -> Option<String> {
    // Get the size of the object
    let size = match s3_entity.object.size {
        Some(s) => s,
        None => return Some("Missing object size".to_string()),
    };

    // Check if the file size is not zero
    if size <= 0 {
        return Some("Invalid size, it should be greater than 0".to_string());
    }
    None
}

fn check_file_name(s3_entity: &S3Entity) -> Option<String> {
    let key = match &s3_entity.object.key {
        Some(k) => k,
        None => return Some("Missing object key".to_string()),
    };
    let file_name_without_ext = Path::new(key).file_stem().unwrap().to_str().unwrap();
    let parts: Vec<&str> = file_name_without_ext.split("-").collect();
    if parts.len() != 4 {
        return Some("Invalid file name format, it should be formated as a Prod ID".to_string());
    }

    for part in parts {
        if !part.chars().all(|c| c.is_numeric()) {
            return Some("Invalid file name format, it should be a numeric code".to_string());
        }
    }
    None
}

pub fn is_valid_file(s3_entity: &S3Entity) -> (bool, String) {
    let mut error_messages = Vec::new();

    // Check the file extension
    if let Some(error) = check_file_extension(s3_entity) {
        error_messages.push(error);
    }
    // Check the file size
    if let Some(error) = check_file_size(s3_entity) {
        error_messages.push(error);
    }
    // Check the file name format
    if let Some(error) = check_file_name(s3_entity) {
        error_messages.push(error);
    }

    // If there are no error messages, the file is valid
    if error_messages.is_empty() {
        return (true, "File is valid".to_string());
    }
    // Otherwise, join the error messages and return them
    (false, error_messages.join(", "))
}

pub async fn single_tag(
    event_s3_attributes: &S3Entity,
    s3_client: &S3Client,
    tag_name: &str,
) -> Result<PutObjectTaggingOutput, Error> {
    let bucket_name = event_s3_attributes
        .bucket
        .name
        .as_ref()
        .ok_or("Missing bucket name")?;
    let object_key = &event_s3_attributes
        .object
        .key
        .as_ref()
        .ok_or("Missing object key")?
        //handle the possibility of a file with an uwanted space, s3 adds a + to the event.
        .replace("+", " ");
    let object_version_id = event_s3_attributes
        .object
        .version_id
        .as_ref()
        .ok_or("Object has no version ID defined, is versioning enabled in the bucket?")?;

    let output = s3_client
        .put_object_tagging()
        .bucket(bucket_name)
        .key(object_key)
        .version_id(object_version_id
        )
        .tagging(Tagging::tag_as_true(tag_name))
        .send()
        .await
        .map_err(|e| {
            let original_error = e.into_service_error().to_string();
            Error::from(format!(
                "Original Error: {} Caused: Could not add tag {} to Object s3://{}/{} versionId: {}",
                original_error, tag_name, bucket_name, object_key, object_version_id
            ))
        })?;
    Ok(output)
}

pub async fn add_tag(
    event_s3_attributes: &S3Entity,
    s3_client: &S3Client,
    tag_name: &str,
) -> Result<PutObjectTaggingOutput, Error> {
    let bucket_name = event_s3_attributes
        .bucket
        .name
        .as_ref()
        .ok_or("Missing bucket name")?;
    let object_key = &event_s3_attributes
        .object
        .key
        .as_ref()
        .ok_or("Missing object key")?
        //handle the possibility of a file with an uwanted space, s3 adds a + to the event.
        .replace("+", " ");
    let object_version_id = event_s3_attributes
        .object
        .version_id
        .as_ref()
        .ok_or("Object has no version ID defined, is versioning enabled in the bucket?")?;

    let input: Tagging = s3_client
        .get_object_tagging()
        .bucket(bucket_name)
        .key(object_key)
        .version_id(object_version_id)
        .send()
        .await
        .map_err(|e| {
            let original_error = e.into_service_error().to_string();
            Error::from(format!(
                "Original Error: {}; Could not get tags from to Object s3://{}/{} versionId: {}",
                original_error, bucket_name, object_key, object_version_id
            ))
        })?
        .add_true_tag(tag_name);

    let output = s3_client
        .put_object_tagging()
        .bucket(bucket_name)
        .key(object_key)
        .version_id(object_version_id)
        .tagging(input)
        .send()
        .await
        .map_err(|e| {
            let original_error = e.into_service_error().to_string();
            Error::from(format!(
                "Original Error: {}; Could not add tag {} to Object s3://{}/{} versionId: {}",
                original_error, tag_name, bucket_name, object_key, object_version_id
            ))
        })?;
    Ok(output)
}
