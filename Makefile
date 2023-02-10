.PHONY: build
build-VerificationLambda:
	cargo lambda build --release #Generate Artifacts in the Background.
	echo $(ARTIFACTS_DIR)
	cp ./target/lambda/rust-lambda-s3-tagging-sqs/bootstrap $(ARTIFACTS_DIR) # Copy the artifact to the dir expected by SAM