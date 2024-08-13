# Water Bucket Challenge

## Overview

The **Water Bucket Challenge** is a classic algorithmic problem where the goal is to measure exactly `Z` gallons of water using two buckets with capacities `X` and `Y` gallons. This project provides a highly optimized and scalable solution implemented in Rust, exposing the functionality through a RESTful API built with Actix-Web. The infrastructure for this project is managed using Infrastructure as Code (IaC) with Terraform, available in the [water-bucket-challenge-iac](https://github.com/hirapius/water-bucket-challenge-iac) repository.

## Features

- **Efficient Algorithm**: Uses a breadth-first search (BFS) approach to find the optimal sequence of steps.
- **Caching**: Employs the `cached` library to improve performance by storing results of common inputs.
- **Robust Error Handling**: Validates input thoroughly and provides meaningful error messages in a structured format.
- **Modular Design**: Clean, scalable architecture with well-separated concerns.
- **Security Validators**: Ensures safe and valid inputs through custom validators.

## Project Structure

```
src/
├── main.rs                  # Application entry point and Actix-Web setup
├── routes/
│   └── mod.rs               # Route initialization
│   └── bucket_routes.rs     # API routes for the water bucket challenge
├── services/
│   └── mod.rs               # Service layer
│   └── bucket_service.rs    # Core logic for solving the water bucket problem
├── models/
│   └── mod.rs               # Data models
│   └── bucket.rs            # Request and response models
│   └── error.rs             # Error response model
├── tests/
│   └── mod.rs               # Unit tests for the service layer
├── validators/
│   └── mod.rs               # Validation logic
│   └── security.rs          # Input validation and security checks
Cargo.toml                   # Project dependencies
Dockerfile                   # Docker setup for containerization
.github/
└── workflows/
    └── deploy.yml           # GitHub Actions for CI/CD
    └── test.yml           # GitHub Actions for Tests
```

## API Usage

The API exposes a single endpoint `/solve` that accepts a POST request with a JSON payload:

### Sample Payload

```json
{
  "x_capacity": 3,
  "y_capacity": 5,
  "z_amount_wanted": 4
}
```

### Sample Response

```json
{
  "solution": [
    {
      "step": 1,
      "bucket_x": 3,
      "bucket_y": 0,
      "action": "fill_bucket_x"
    },
    {
      "step": 2,
      "bucket_x": 0,
      "bucket_y": 3,
      "action": "transfer_from_bucket_x_to_y"
    },
    {
      "step": 3,
      "bucket_x": 3,
      "bucket_y": 2,
      "action": "empty_bucket_x"
    }
  ],
  "status": "solved"
}
```

### Error Handling

If the request is invalid, the API will return an error message. For example:

```json
{
  "error": "the_target_amount_cannot_be_greater_than_both_bucket_capacities"
}
```

## Infrastructure as Code (IaC)

The infrastructure for this project is managed using Terraform and is available in the [water-bucket-challenge-iac](https://github.com/hirapius/water-bucket-challenge-iac) repository. This setup includes:

- **ECS Fargate Cluster**: For running the Dockerized application.
- **Application Load Balancer (ALB)**: For distributing traffic to the ECS tasks.
- **AWS WAF**: For protecting the application with security rules and rate limiting.
- **Global Accelerator**: For improving application availability and performance.

## CI/CD with GitHub Actions

This project uses GitHub Actions for Continuous Integration and Continuous Deployment (CI/CD). The workflow is defined in the `.github/workflows/deploy.yml` file, which:

- **Builds** the Docker image from the Dockerfile.
- **Pushes** the image to Amazon ECR.
- **Deploys** the image to the ECS Fargate service.

### Required GitHub Secrets

Ensure that the following secrets are configured in your GitHub repository:

- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `AWS_REGION`
- `ECR_REPOSITORY`
- `ECS_CLUSTER_NAME`
- `ECS_SERVICE_NAME`
- `IMAGE_TAG` (Optional, defaults to commit SHA)

## API Documentation

This project utilizes [utoipa](https://docs.rs/utoipa/latest/utoipa/) to generate OpenAPI documentation automatically for the API endpoints. The documentation is served using Swagger UI, making it easy to explore and test the API.

### Accessing Swagger UI

To view the API documentation, follow these steps:

1. Start the server by running the following command:
   ```bash
   cargo run
   ```

2. Open your web browser and navigate to:
   ```
   http://localhost:8080/swagger-ui/
   ```

   Here, you can interact with the API directly through the Swagger UI, exploring endpoints, testing requests, and viewing responses.

### How It Works

- The API documentation is generated automatically based on the annotations provided in the code using `utoipa`.
- The `utoipa` crate, combined with `utoipa-swagger-ui`, allows seamless integration of Swagger UI with Actix-Web, providing a clear interface to interact with your API.

For more information about `utoipa`, you can refer to the official [utoipa documentation](https://docs.rs/utoipa/latest/utoipa/).

## Automated HTTP Tests

This project includes a `tests.py` script designed to automate HTTP requests to the API endpoints. These tests help ensure that the API behaves as expected and handle various scenarios, including valid and invalid requests.

### Running the HTTP Tests

To run the automated HTTP tests:

1. Ensure the server is running:
   ```bash
   cargo run
   ```

2. Execute the `tests.py` script to perform the HTTP requests:
   ```bash
   python tests.py
   ```

### What the Tests Cover

- **Valid Requests**: The script tests valid inputs to ensure the API returns the correct responses and solves the water bucket problem as expected.
- **Invalid Requests**: The script also sends invalid inputs to verify that the API handles errors correctly and returns appropriate error messages.
- **Edge Cases**: Additional tests cover edge cases to validate the robustness of the API.

These tests provide a quick and automated way to verify the API's functionality during development and after changes to the codebase.

## Contact

For any inquiries, please contact Hirapius at hirapius@jupiter.seg.br.

