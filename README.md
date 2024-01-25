# Introduction

For the Zama Merkle Chalenge, the solution implements two main components: zama-client and zama-server. The solution secures uploading and downloading files to and from a server while ensuring data integrity using Merkle Tree cryptography.

## Setting up

To get started with the Zama Merkle Tree project, follow these steps:

0. Rename the `.env.sample` to `.env`for both zama-client and zama-server
   `mv ./zama-client/.env.sample ./zama-client/.env && mv ./zama-server/.env.sample ./zama-server/.env`

1. Grant execution permissions to the necessary scripts:
   `chmod +x zama-build.sh zama-upload.sh zama-download.sh`

2. Build Docker images and run the zama-server container:
   `./zama-build.sh`

3. Upload files to the server using the zama-client:
   `./zama-upload.sh {folder_path}`

4. Download and verify files from the server using the zama-client:
   `./zama-download.sh {folder_path} {file_name}`

## Architecture

This solution for the challenge is designed to have two application; zama-client and zama-server.

### zama-client (built in rust)

The client is a cli rust application, that is composed of the following Modules:

- `cli` module acts as the entry point for user interactions, parsing and executing commands from the command line. It interfaces with other modules to execute upload and download operations.
- `merkle` module is essential for constructing and verifying the Merkle tree. It handles the creation of leaf and branch nodes and overall tree structure. It also includes logic for verifying a specific node using the generated Merkle proof.
- `api` module manages communication with the zama-server through various API calls. It includes functions for uploading files, downloading files, and obtaining Merkle proofs for specific files.
- `utils` module provides utility functions crucial for the client's functionality. It includes functions for saving and reading files to/from disk and hash256 functionalities required for Merkle tree operations.
- `config` module handles general configuration parameters through environment variables, allowing for easy configuration adjustments without modifying the core logic.

### zama-server (built in go)

The running server is built with go. The server mainly handles three endpoints

- `POST : v1/api/files/upload` : The endpoint is to upload a set of files and it create a merkle tree based on their contents
- `GET : v1/api/files/{file_name}` : For a specific file, the endpoint is returns a json consists of {file_name, merkle_proof, merkle_root_hash}
- `GET : v1/api/files/{file_name}/download` : The endpoint downloads the file itself.

The go server has the following packages:

- `merkle` package is dedicated to building the Merkle tree on the server side. It generates Merkle proofs needed for file verification, ensuring the integrity and security of uploaded files.
- `controller` package houses the main route handlers responsible for handling incoming requests and generating appropriate responses. It acts as the interface between the server's API and the underlying logic.
- `routes` package defines and organizes all the routes related to file upload, download, and Merkle proof retrieval. Each route is associated with specific functionality, making the code modular and easy to maintain.
- `utils` package contains utility functions related to file system operations. It handles reading file content, obtaining file paths, and saving files. Additionally, it incorporates hash functionalities essential for Merkle tree operations.
- `config` package manages environment variables and general configuration settings for the server, centralizing configuration management for easier adjustments.

### Interaction Flow

1. **Upload Files:**

- The zama-client executes the `upload` command, triggering the `cli` module.
- The `cli` module interacts with the `api` module to make a `POST` request to the server's `v1/api/files/upload` endpoint.
- The server's `controller` package handles the upload request, utilizing the `merkle` package to build the Merkle tree.
- The server responds with the Merkle root hash.

2. **Download and Verify Files:**

- The zama-client executes the `download` command, triggering the `cli` module.
- The `cli` module interacts with the `api` module to make a `GET` request to the server's `v1/api/files/{file_name}` endpoint.
- The server's `controller` package handles the download request, providing the requested file, Merkle proof, and Merkle root hash.
- The client uses the received Merkle proof to verify the integrity of the downloaded file against the persisted Merkle root hash.

### Future Improvements

To further enhance the architecture:

- Implement more sophisticated error handling modules/packages for both the client and server.
- Explore optimizations in the communication protocol, possibly implementing a more efficient serialization format for data transfer.
- Consider scalability aspects, such as handling large file sets and optimizing Merkle tree generation for performance.
- Conduct thorough testing and profiling to identify and address potential bottlenecks.
- Enhance the user interface by providing clear feedback during upload and download operations.
