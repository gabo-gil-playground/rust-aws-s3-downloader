# rust-aws-s3-downloader
Downloader service that exposes implemented end-points to interact with Amazon AWS S3 features for download files purposes.


## Assumptions
### Required configuration
* AWS client has been configured and ```aws login``` process is completed 
  * https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
  * https://docs.aws.amazon.com/cli/latest/userguide/getting-started-quickstart.html
* AWS environment variables are properly set (optional)
  * export AWS_ACCESS_KEY_ID="..."
  * export AWS_SECRET_ACCESS_KEY="..."
  * export AWS_SESSION_TOKEN="..." 

## End-points
### Health
#### health check end-point
Request sample:
```
curl --request GET --url https://localhost:8097/health
```
Response sample:
```
{"status":"server is running"}
``` 

### Amazon S3
#### downloader end-point
Request sample:
```
curl --request POST \
  --url http://localhost:8097/api/v1/download/zip \
  --header 'Content-Type: application/json' \
  --data '{
      "bucket_name": "some-s3-bucket-name", 
      "full_path": "path/to/sub_folder"
  }'
```
Response sample:
```
raw response with generate zip file
``` 
### Swagger
#### API documentation end-point
Swagger documentation pages with exposed API end-points - URL sample:
```
TBC
```

## Docker
TBC


## Testing
### Unit tests
#### execute unit test cases
Unit test cases are included as part of each .rs file.

You can run one by one from IDE or you also can run the whole unit test suite from command line:
```
cargo unit-test-only
```
**important**: this is a custom alias based on **cargo xtask** project configurations.
More details about how to use aliases at documentation section


### Integration tests
#### execute integration test cases
Integration test cases are included at **/tests** folder.

You can run one by one from IDE or you also can run the whole integration test suite from command line:
```
cargo integration-test-only
```
**important**: this is a custom alias based on **cargo xtask** project configurations.
More details about how to use aliases at documentation section


### Coverage
#### CI report (lcov format)
```
cargo coverage
```
**important**: this is a custom alias based on **cargo xtask** project configurations.
More details about how to use aliases at documentation section
#### DEV report (HTML format)
```
cargo coverage-dev
```
**important**: this is a custom alias based on **cargo xtask** project configurations.
More details about how to use aliases at documentation section


## Documentation
[TBC](TBC)
