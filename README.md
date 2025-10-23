# PayBySquare QR Generator API

REST API service for generating PayBySquare QR codes according to Slovak banking standard version 1.1.0.

## Features

- **Generate QR Codes**: Create PayBySquare QR code images (PNG) with optional frame
- **Generate Codes**: Get PayBySquare code as text string for custom processing
- **OpenAPI Documentation**: Interactive Swagger UI documentation
- **Validation**: Comprehensive input validation with detailed error messages
- **Docker Support**: Multi-stage Docker build for optimized images
- **Kubernetes Ready**: Complete K8s manifests included
- **Production Ready**: Health checks, logging, CORS, graceful shutdown

## Quick Start

### Prerequisites

- Rust 1.75 or later
- Cargo

### Build and Run

```bash
# Development mode
cargo run

# Release mode (optimized)
cargo build --release
./target/release/pay-by-square-generator
```

The API will be available at `http://localhost:3000`

### Docker

```bash
# Build Docker image
docker build -t pay-by-square-generator .

# Run container
docker run -p 3000:3000 pay-by-square-generator
```

### Kubernetes

```bash
# Apply Kubernetes manifests
kubectl apply -f k8s-deployment.yaml

# Check deployment status
kubectl get pods -l app=pay-by-square-generator
```

## API Endpoints

### 1. Generate QR Code Image

**Endpoint**: `POST /pay-by-square-generator/generate-qr`

**Description**: Generates a PayBySquare QR code as a PNG image with optional frame.

**Request Body**:
```json
{
  "amount": 100.50,
  "iban": "SK9611000000002918599669",
  "currency": "EUR",
  "beneficiary_name": "John Doe",
  "variable_symbol": "1234567890",
  "note": "Payment for invoice #123"
}
```

**Response**: PNG image (Content-Type: `image/png`)

**Example**:
```bash
curl -X POST http://localhost:3000/pay-by-square-generator/generate-qr \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 100.50,
    "iban": "SK9611000000002918599669",
    "currency": "EUR",
    "beneficiary_name": "John Doe",
    "variable_symbol": "1234567890",
    "note": "Payment for invoice"
  }' \
  --output qr-code.png
```

### 2. Generate Code String

**Endpoint**: `POST /pay-by-square-generator/generate-code`

**Description**: Generates a PayBySquare code as a text string.

**Request Body**: Same as above

**Response**:
```json
{
  "code": "0004G00006F071MBI3LRVO4PS..."
}
```

**Example**:
```bash
curl -X POST http://localhost:3000/pay-by-square-generator/generate-code \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 100.50,
    "iban": "SK9611000000002918599669",
    "currency": "EUR"
  }'
```

### 3. Version Information

**Endpoint**: `GET /pay-by-square-generator/version.txt`

**Response**: Plain text version number

**Example**:
```bash
curl http://localhost:3000/pay-by-square-generator/version.txt
```

### 4. Health Check

**Endpoint**: `GET /health`

**Response**:
```json
{
  "status": "healthy",
  "service": "pay-by-square-generator"
}
```

## Payment Request Fields

### Required Fields

At least one of the following must be provided:

- `amount` (number): Payment amount (must be > 0)
- `iban` (string): IBAN of beneficiary account
- `bank_accounts` (array): List of bank accounts (alternative to `iban`)

### Optional Fields

#### Basic Information
- `currency` (string): Currency code (default: "EUR")
- `swift` (string): SWIFT/BIC code (max 11 characters)
- `date` (string): Payment date (ISO 8601 format: YYYY-MM-DD)
- `payment_due_date` (string): Payment due date

#### Beneficiary Information
- `beneficiary_name` (string): Name of beneficiary (max 70 characters)
- `beneficiary_address_1` (string): Address line 1 (max 70 characters)
- `beneficiary_address_2` (string): Address line 2 (max 70 characters)

#### Payment Identifiers
- `invoice_id` (string): Invoice identifier (max 10 characters)
- `variable_symbol` (string): Variable symbol (max 10 characters)
- `constant_symbol` (string): Constant symbol (max 4 characters)
- `specific_symbol` (string): Specific symbol (max 10 characters)
- `originators_reference_information` (string): SEPA reference (max 35 characters)
- `note` (string): Message for beneficiary (max 140 characters)

#### Advanced Features

**Payment Options**:
```json
{
  "payment_options": ["PAYMENT_ORDER", "STANDING_ORDER", "DIRECT_DEBIT"]
}
```

**Standing Order**:
```json
{
  "standing_order": {
    "day": 15,
    "month": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    "periodicity": "MONTHLY",
    "last_date": "2024-12-31"
  }
}
```

Periodicity options: `DAILY`, `WEEKLY`, `MONTHLY`, `QUARTERLY`, `HALF_YEARLY`, `YEARLY`

**Direct Debit**:
```json
{
  "direct_debit": {
    "scheme": "SEPA",
    "debit_type": "RECURRENT",
    "mandate_id": "MANDATE123",
    "creditor_id": "CREDITOR456",
    "max_amount": 500.00,
    "valid_till_date": "2024-12-31"
  }
}
```

**Multiple Bank Accounts**:
```json
{
  "bank_accounts": [
    {
      "iban": "SK9611000000002918599669",
      "swift": "GIBASKBX"
    },
    {
      "iban": "SK3112000000198742637541"
    }
  ]
}
```

## Complete Example

```bash
curl -X POST http://localhost:3000/pay-by-square-generator/generate-qr \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 150.00,
    "iban": "SK9611000000002918599669",
    "currency": "EUR",
    "swift": "GIBASKBX",
    "beneficiary_name": "ACME Corporation",
    "beneficiary_address_1": "Main Street 123",
    "beneficiary_address_2": "Bratislava, 81105",
    "variable_symbol": "2024001234",
    "constant_symbol": "0308",
    "note": "Invoice payment - #INV-2024-001234",
    "payment_due_date": "2024-12-31",
    "invoice_id": "INV-001234"
  }' \
  --output payment-qr.png
```

## Interactive Documentation

Access the interactive Swagger UI documentation at:

```
http://localhost:3000/pay-by-square-generator/docs
```

Or visit the root URL which redirects to the documentation:

```
http://localhost:3000/
```

## Frame Template

To use a custom frame around QR codes, place a `frame.png` file in the project root directory. The QR code will be centered at 85% of the frame size.

If no frame is provided, the API will return a plain QR code image.

To create a simple frame, you can use any image editor to create a 600x600 PNG image with the "PAY by square" branding.

## Development

### Project Structure

```
.
├── src/
│   ├── main.rs         # Web server and endpoints
│   ├── lib.rs          # Public API exports
│   ├── models.rs       # Data structures and schemas
│   ├── generator.rs    # PayBySquare algorithm implementation
│   ├── qr.rs           # QR code generation and frame handling
│   ├── validation.rs   # Input validation
│   └── errors.rs       # Error types and handling
├── Cargo.toml          # Dependencies and configuration
├── Dockerfile          # Multi-stage Docker build
├── k8s-deployment.yaml # Kubernetes manifests
├── frame.png           # Optional QR frame template
└── README.md           # This file
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Build Optimization

The release build is optimized for size:

```bash
cargo build --release
```

Build optimizations in `Cargo.toml`:
- `opt-level = "z"` - Optimize for size
- `lto = true` - Link-time optimization
- `codegen-units = 1` - Better optimization
- `strip = true` - Strip symbols

## Environment Variables

- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Logging level (default: info)
  - Options: `error`, `warn`, `info`, `debug`, `trace`

Example:
```bash
PORT=8080 RUST_LOG=debug cargo run
```

## Docker Deployment

### Build and Push to Azure Container Registry

```bash
# Build the image
docker build -t astonwus.azurecr.io/pay-by-square-generator:latest .

# Login to ACR
az acr login --name astonwus

# Push the image
docker push astonwus.azurecr.io/pay-by-square-generator:latest
```

### Run from ACR

```bash
docker run -p 3000:3000 astonwus.azurecr.io/pay-by-square-generator:latest
```

## Kubernetes Deployment

The included `k8s-deployment.yaml` contains:

- **Service**: ClusterIP service exposing port 80
- **Deployment**: 2 replicas with health checks and resource limits
- **Ingress**: NGINX ingress with path `/pay-by-square-generator`

### Deploy to Kubernetes

```bash
# Create image pull secret (if not exists)
kubectl create secret docker-registry astonwus \
  --docker-server=astonwus.azurecr.io \
  --docker-username=<username> \
  --docker-password=<password>

# Deploy
kubectl apply -f k8s-deployment.yaml

# Check status
kubectl get all -l app=pay-by-square-generator

# View logs
kubectl logs -l app=pay-by-square-generator -f
```

### Update Deployment

```bash
# After pushing a new image
kubectl rollout restart deployment/pay-by-square-generator

# Check rollout status
kubectl rollout status deployment/pay-by-square-generator
```

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK` - Success
- `400 Bad Request` - Invalid input (validation errors)
- `500 Internal Server Error` - Server error

Error response format:
```json
{
  "error": "Validation error: Amount must be greater than 0"
}
```

Common validation errors:
- Missing required fields (amount, iban/bank_accounts)
- Invalid IBAN format
- Invalid SWIFT/BIC format
- Field length exceeded
- Invalid amount (must be > 0)

## PayBySquare Algorithm

The implementation follows the PayBySquare specification v1.1.0:

1. **Data Structure**: Tab-separated values with payment information
2. **CRC32 Checksum**: Calculated and prepended (little-endian, 4 bytes)
3. **LZMA Compression**: Compressed with specific parameters
4. **Header**: 4-byte header added (type, version, document type, reserved)
5. **Base32hex Encoding**: Binary data encoded to ASCII (0-9, A-V)
6. **QR Code**: Generated and optionally framed

## Security

- No `unwrap()` calls in production code
- Proper error propagation with `?` operator
- Input validation and sanitization
- Buffer overflow protection
- Non-root user in Docker container

## Performance

- Async I/O for all operations (Actix-web + Tokio)
- Optimized release builds
- Minimal Docker image size
- Resource limits in Kubernetes

## License

This project implements the PayBySquare standard (Slovak Banking Association).

## Support

For issues or questions:
- Check the interactive documentation at `/pay-by-square-generator/docs`
- Review the Project.md specification
- Contact the project maintainer

## Acknowledgments

- PayBySquare specification: Slovak Banking Association (SBA)
- Built with Rust and Actix-web
- QR code generation: `qrcode` crate
- Image processing: `image` crate
