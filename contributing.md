
## Using the OpenAPI Generator to generate new models

#### Start container
```sh
podman pull docker://openapitools/openapi-generator-online 
```

#### Start container at port 8888 and save the container id
```sh
CID=$(podman run -d -p 8888:8080 openapitools/openapi-generator-online)
sleep 10

# Execute an HTTP request to generate a Ruby client
curl -X POST --header 'Content-Type: application/json' --header 'Accept: application/json' -d '{"openAPIUrl": "https://api.authress.io/", "options": { "useSingleRequestParameter": true, "packageName": "authress", "packageVersion": "99.99.99" } }' 'http://localhost:8888/api/gen/clients/rust'


# RESPONSE: { "link":"http://localhost:8888/api/gen/download/c2d483.3.4672-40e9-91df-b9ffd18d22b8" }
```

### Download the generated zip file
```sh

wget RESPONSE_LINK

# Unzip the file
unzip SHA

# Shutdown the openapi generator image
podman stop $CID && podman rm $CID
```

### Common review items
* Add authentication to the configuration class.
* Url encode all url inputs, frequently the generator doesn't do that
* Remove any unnecessary validations from object and parameter injection, often there are some even when properties are allowed to be null
* The service client code to generate a JWT from private key needs to be added
* Top level tags from the API should accessible from the base class: `authressClient.accessRecords.getRecords(...)`
* Test