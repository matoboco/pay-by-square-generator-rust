# Zadanie: PayBySquare QR Generator API (Rust)

## Úvod

Vytvorte REST API službu v jazyku Rust pre generovanie PayBySquare QR kódov podľa slovenského bankového štandardu verzie 1.1.0. PayBySquare je národný štandard pre QR platby schválený Slovenskou bankovou asociáciou.

## Funkčné požiadavky

### 1. Základná funkcionalita

Implementujte Actix-web alebo Axum aplikáciu s nasledujúcimi endpointmi:

#### POST `/pay-by-square-generator/generate-qr`
- Vstup: JSON s platobnými údajmi
- Výstup: PNG obrázok QR kódu (binárne dáta)
- QR kód musí byť obklopený "PAY by square" rámčekom (šablóna `frame.png`)
- Validácia povinných polí: `amount` a (`iban` ALEBO `bank_accounts`)

#### POST `/pay-by-square-generator/generate-code`
- Vstup: JSON s platobnými údajmi (rovnaké ako vyššie)
- Výstup: JSON s PayBySquare kódom ako textovým reťazcom
- Formát: `{"code": "0004G00006F071MBI3LRVO4PS..."}`

#### GET `/pay-by-square-generator/version.txt`
- Výstup: Plain text s číslom verzie z `Cargo.toml`

### 2. Podporované platobné údaje

#### Základné polia (povinné):
- `amount` - suma platby (f64)
- `iban` - IBAN účtu príjemcu (String) ALEBO
- `bank_accounts` - pole bankových účtov (Vec<BankAccount>), každý obsahuje `iban` a voliteľne `swift`

#### Voliteľné základné polia:
- `currency` - mena (default: "EUR")
- `swift` - SWIFT/BIC kód
- `date` - dátum platby (NaiveDate)
- `payment_due_date` - dátum splatnosti (NaiveDate)
- `invoice_id` - identifikátor faktúry (max 10 znakov)
- `beneficiary_name` - meno príjemcu (max 70 znakov)
- `beneficiary_address_1` - adresa riadok 1 (max 70 znakov)
- `beneficiary_address_2` - adresa riadok 2 (max 70 znakov)
- `variable_symbol` - variabilný symbol (max 10 znakov)
- `constant_symbol` - konštantný symbol (max 4 znaky)
- `specific_symbol` - špecifický symbol (max 10 znakov)
- `originators_reference_information` - SEPA referencia (max 35 znakov)
- `note` - správa pre príjemcu (max 140 znakov)

#### Pokročilé funkcie:

**Typy platieb:**
- `payment_options` - Vec<PaymentOption> enum s hodnotami [PaymentOrder, StandingOrder, DirectDebit]

**Trvalý príkaz (Standing Order):**
```rust
#[derive(Serialize, Deserialize)]
struct StandingOrder {
    day: u8,
    month: Vec<u8>,
    periodicity: Periodicity, // enum: Daily, Weekly, Monthly, etc.
    last_date: NaiveDate,
}
```

**Inkaso (Direct Debit):**
```rust
#[derive(Serialize, Deserialize)]
struct DirectDebit {
    scheme: DirectDebitScheme, // enum: Sepa, Other
    debit_type: DirectDebitType, // enum: OneOff, Recurrent
    mandate_id: Option<String>,
    creditor_id: Option<String>,
    max_amount: Option<f64>,
    valid_till_date: Option<NaiveDate>,
}
```

**QR nastavenia:**
- `with_frame` - pridať rámček (default: true)
- `qr_size` - veľkosť QR kódu v px (default: 300)

### 3. PayBySquare algoritmus

PayBySquare kód sa generuje podľa nasledujúceho procesu:

1. **Vytvorenie dátovej štruktúry** - pole hodnôt oddelených tabulátorom (\t)
2. **CRC32 checksum** - vypočítať a pripojiť na začiatok (4 bajty, little-endian)
3. **LZMA kompresia** - s parametrami:
   - LC = 3, LP = 0, PB = 2
   - Dictionary size = 128 KB
   - Header: 2 bajty (veľkosť nekomprimovaných dát)
4. **Pridanie hlavičky** - 4 bajty (by square type, version, document type, reserved)
5. **Base32hex konverzia** - binárne dáta → ASCII znaky (0-9, A-V)
6. **QR kód** - vygenerovať PNG obrázok

### 4. Technické požiadavky

**Architektúra:**
- Modulárna štruktúra: `main.rs` (web server), `lib.rs` (generovacia logika), `models.rs` (dátové štruktúry)
- Oddelené moduly pre každú logickú časť
- Funkcie musia byť použiteľné aj mimo web aplikácie

**Použité crates:**
- `actix-web` alebo `axum` + `tokio` - async web framework
- `serde` + `serde_json` - serializácia/deserializácia
- `lzma-rust` alebo `xz2` - LZMA kompresia
- `qrcode` - generovanie QR kódov
- `crc` alebo `crc32fast` - CRC32 checksum
- `image` - manipulácia s obrázkami (pridanie rámčeka)
- `chrono` - práca s dátumami
- `utoipa` + `utoipa-swagger-ui` - OpenAPI dokumentácia
- `validator` - validácia vstupov

**Rámček:**
- Súbor `frame.png` musí byť v koreňovom adresári alebo embedovaný pomocou `include_bytes!`
- QR kód sa vycentruje na 85% veľkosti rámčeka
- Ak rámček neexistuje, vráti sa čistý QR kód

### 5. Dokumentácia

**OpenAPI 3.0 pomocou utoipa:**
- Automaticky generovaná dokumentácia z Rust kódu pomocou macros
- Všetky endpointy, schémy, príklady použitia
- Popisy v slovenčine kde je to možné

**Swagger UI:**
- Endpoint `/pay-by-square-generator/docs` s interaktívnou dokumentáciou
- Root URL `/` presmeruje na dokumentáciu
- Možnosť "Try it out" priamo z prehliadača

**README.md:**
- Inštrukcie pre build a spustenie
- Príklady použitia všetkých endpointov (curl)
- Dokumentácia všetkých podporovaných polí
- Docker a Kubernetes deployment inštrukcie

### 6. Deployment

**Docker:**
- Multi-stage `Dockerfile`:
  - Builder stage: `rust:alpine` alebo `rust:slim`
  - Runtime stage: `alpine:latest` s minimálnymi závislosťami
- Optimalizovaný binary (release build)
- Všetky potrebné súbory embedované alebo skopírované

**Kubernetes:**
- `k8s-deployment.yaml` s Service, Deployment a Ingress
- Image v Azure Container Registry: `astonwus.azurecr.io/pay-by-square-generator`
- Image pull secret: `astonwus`
- Ingress path: `/pay-by-square-generator`

## Technické špecifikácie

### Štruktúra projektu
```
.
├── src/
│   ├── main.rs                # Web server (Actix/Axum)
│   ├── lib.rs                 # Exportované funkcie
│   ├── models.rs              # Dátové štruktúry (structs, enums)
│   ├── generator.rs           # PayBySquare generovacia logika
│   ├── qr.rs                  # QR kód a rámček
│   └── validation.rs          # Validácia vstupov
├── Cargo.toml                 # Závislosti a konfigurácia
├── frame.png                  # Šablóna PAY by square rámčeka
├── Dockerfile                 # Multi-stage Docker image
├── k8s-deployment.yaml        # Kubernetes manifesty
└── README.md                  # Dokumentácia
```

### Príklad dátových štruktúr (models.rs)
```rust
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PaymentRequest {
    #[validate(range(min = 0.01))]
    pub amount: f64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 34))]
    pub iban: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_accounts: Option<Vec<BankAccount>>,
    
    #[serde(default = "default_currency")]
    pub currency: String,
    
    // ... ostatné polia
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    pub iban: String,
    pub swift: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentOption {
    PaymentOrder,
    StandingOrder,
    DirectDebit,
}

fn default_currency() -> String {
    "EUR".to_string()
}
```

### Exportované funkcie (lib.rs)
```rust
pub fn generate_pay_by_square_code(payment: &PaymentRequest) -> Result<String, Error>;
pub fn generate_qr_image(code: &str, size: u32) -> Result<Vec<u8>, Error>;
pub fn add_frame(qr_data: Vec<u8>, frame_path: &str) -> Result<Vec<u8>, Error>;
pub fn generate_pay_by_square_qr(payment: &PaymentRequest, opts: QrOptions) -> Result<Vec<u8>, Error>;
```

### Cargo.toml (základné závislosti)
```toml
[package]
name = "pay-by-square-generator"
version = "1.0.0"
edition = "2021"

[dependencies]
actix-web = "4"  # alebo axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
validator = { version = "0.18", features = ["derive"] }
qrcode = "0.14"
image = "0.25"
crc = "3"
xz2 = "0.1"  # pre LZMA
utoipa = { version = "4", features = ["actix_extras"] }
utoipa-swagger-ui = { version = "6", features = ["actix-web"] }

[profile.release]
opt-level = "z"  # Optimize for size
lto = true       # Link-time optimization
codegen-units = 1
strip = true     # Strip symbols
```

## Hodnotenie

### Povinné kritériá (musí fungovať):
- ✅ Všetky 3 endpointy funkčné
- ✅ Generovanie QR kódu s jednoduchými údajmi (amount, iban)
- ✅ Validácia vstupov pomocá `validator` crate
- ✅ Error handling pomocou `Result` a vlastných error typov
- ✅ QR kód s rámčekom
- ✅ Správny PayBySquare formát (testovateľný v mobilnej banke)
- ✅ Async/await patterns

### Rozšírené kritériá (bonus):
- ✅ Podpora viacerých bankových účtov
- ✅ Trvalé príkazy a inkasá
- ✅ OpenAPI dokumentácia pomocou `utoipa` + Swagger UI
- ✅ Docker multi-stage build s optimalizáciou veľkosti
- ✅ Kubernetes deployment
- ✅ Idiomatický Rust kód (ownership, borrowing, error handling)
- ✅ Kompletná dokumentácia v README

### Bonusové rozšírenia:
- Unit testy a integration testy
- Benchmarky (criterion)
- Health check endpoint (`/health`)
- Metrics endpoint pre Prometheus (`/metrics`)
- Structured logging (tracing, tracing-subscriber)
- Graceful shutdown
- Rate limiting middleware
- CORS konfigurácia
- CI/CD (GitHub Actions)

## Rust-špecifické požiadavky

### Error Handling
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PayBySquareError {
    #[error("Invalid IBAN format")]
    InvalidIban,
    
    #[error("Compression failed: {0}")]
    CompressionError(String),
    
    #[error("QR generation failed: {0}")]
    QrError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Validácia
- Použitie `validator` crate pre deklaratívnu validáciu
- Custom validátory pre IBAN, SWIFT
- Validácia dĺžky reťazcov podľa PayBySquare špecifikácie

### Bezpečnosť
- Žiadne `unwrap()` v production kóde
- Proper error propagation s `?` operátorom
- Input sanitization
- Buffer overflow protection

### Performance
- Async I/O pre všetky operácie
- Zero-copy kde je to možné
- Lazy evaluation
- Memory pooling pre často alokované objekty

## Referenčné zdroje

- **PayBySquare špecifikácia 1.1.0**: https://www.sbaonline.sk/
- **Actix-web**: https://actix.rs/
- **Axum**: https://docs.rs/axum/
- **utoipa**: https://docs.rs/utoipa/
- **The Rust Book**: https://doc.rust-lang.org/book/

## Časový odhad

- **Základná funkcionalita**: 12-16 hodín
- **Pokročilé funkcie**: 6-8 hodín
- **Dokumentácia a deployment**: 4-6 hodín
- **Optimalizácia a testing**: 4-6 hodín
- **Celkom**: 26-36 hodín

## Dodanie

Odovzdajte:
1. Kompletný zdrojový kód (GitHub repository alebo ZIP)
2. README.md s návodom na build a spustenie
3. Funkčný Docker image (voliteľne aj Docker Hub link)
4. Ukážkové volania API (curl príklady)
5. `cargo test` musí prejsť (ak sú implementované testy)

## Poznámky

- PayBySquare QR kód musí byť funkčný v slovenských bankových aplikáciách
- Dodržujte PayBySquare špecifikáciu 1.1.0 presne
- Kód musí byť idiomatický Rust (clippy warnings na minimum)
- Použite `cargo fmt` pre formátovanie
- Aplikácia musí bežať na porte 3000 (alebo PORT z ENV)
- Release build musí byť optimalizovaný na veľkosť
- Žiadne panic! v production kóde

## Build a spustenie

```bash
# Development
cargo run

# Release (optimalizovaný)
cargo build --release
./target/release/pay-by-square-generator

# Docker
docker build -t pay-by-square-generator .
docker run -p 3000:3000 pay-by-square-generator

# Tests
cargo test

# Linting
cargo clippy
```

---

**Otázky?** Kontaktujte zadávateľa projektu.

**Úspešné dokončenie!** 🦀 🎉
