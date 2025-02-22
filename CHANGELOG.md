# Changelog

All notable changes to this project will be documented in this file.

## [4.16.0] - 2024-05-06

### Bug Fixes

- Fixed import of symmetric key tag to '_kk' from '_sk'

### Features

- Add support for LUKS via PKCS#11 module
- Add support for CKM_RSA_PKCS (PKCS#1 v1.5) for RSA encryption/decryption

## [4.15.2] - 2024-05-03

### Features

- Create Gmail key pairs and identities with `ckms` via Gmail API ([#243](https://github.com/Cosmian/kms/pull/243))

### Bug Fixes

- Comment out mermaid configuration

## [4.15.1] - 2024-05-02

### Features

- Add Google Workspace CSE endpoints for **encrypted Gmail** ([#192](https://github.com/Cosmian/kms/pull/192))

### Bug Fixes

- RUSTSEC-2024-0336 ([#244](https://github.com/Cosmian/kms/pull/244))
- Remove everything related to GCP images build ([#241](https://github.com/Cosmian/kms/pull/241))

### Documentation

- Oauth2 OIDC doc fixes

## [4.15.0] - 2024-04-08

### Bug Fixes

- Add license to KMS GCP image ([#235](https://github.com/Cosmian/kms/pull/235))
- Re-enable the validation of JWT Issuer URI
- Fix CSE error status code, propagating the right status code instead of generic server code error

### Features

- Handle many identity providers in jwt authentication
- New command line argument `--key-usage` to define key or certificate usage on import
- Exhaustive verification that the key used to perform cryptographic operations is allowed to do
  them
- KMIP object creation can now precisely define the usage of the key it describes

## [4.14.2] - 2024-04-05

### Ci

- Add standalone workflow to test KMS in Cosmian
  VM ([#233](https://github.com/Cosmian/kms/pull/233))

### Features

- Rebase KMS GCP image on Cosmian VM 1.1.0-rc.4

## [4.14.1] - 2024-04-04

### Ci

- Remove optimization RUSTFLAGS ([#227](https://github.com/Cosmian/kms/pull/227))

### Features

- Rebase KMS GCP image on Cosmian VM 1.1.0-rc.3

## [4.14.0] - 2024-03-27

### Bug Fixes

- Fixed double quotes problem in cosmian vm test (CI)
- Fixed trailing null byte bug for biguint/bytes
  conversions ([#224](https://github.com/Cosmian/kms/pull/224))
- Make the CLI compile on Windows and macOS (without openssl
  installed) ([#209](https://github.com/Cosmian/kms/pull/209))

### Features

- Support Veracrypt PKCS11 provider library ([#208](https://github.com/Cosmian/kms/pull/208))

### Testing

- Update `test_kms.py` to use covercrypt `14.0.0` ([#217](https://github.com/Cosmian/kms/pull/217))

## [4.13.5] - 2024-03-20

### Bug Fixes

- Add missing image_licenses in packer for GCP ([#219](https://github.com/Cosmian/kms/pull/219))

## [4.13.4] - 2024-03-18

### Ci

- Push GCP images based on Cosmian VM 1.1.0-rc2

## [4.13.3] - 2024-03-11

### Bug fixes

- `ckms` needs `kmip` dependency with
  features `openssl` ([#202](https://github.com/Cosmian/kms/pull/202))

### Ci

- Push GCP images on cosmian-dev and cosmian-public only
  once ([#203](https://github.com/Cosmian/kms/pull/203))

## [4.13.2] - 2024-03-09

### Ci

- Filter reboot test on GCP/RHEL instance.

## [4.13.1] - 2024-03-08

### Ci

- Add build of GCP images (ubuntu/redhat) [#191](https://github.com/Cosmian/kms/pull/191).

## [4.13.0] - 2024-03-08

### Features

- Save KMIP Attributes in a proper column of `Objects`
  table [#166](https://github.com/Cosmian/kms/pull/166):
  - Remove all custom tags `_cert_spki`, `_cert_cn`, `_cert_issuer` and `_cert_sk`
- Add support for CoverCrypt `rekey`, `prune`, and `Policy` editing
  methods [#179](https://github.com/Cosmian/kms/pull/179):
  - Add CLI commands to perform these actions
- Accurate CryptographicUsageMask for KMIP creation (RSA and EC
  keys) [#189](https://github.com/Cosmian/kms/pull/189)
  and [#187](https://github.com/Cosmian/kms/pull/187).

### Refactor

- Rework utils/crypto [#178](https://github.com/Cosmian/kms/pull/178).

### Ci

- Add build on RHEL9 [#196](https://github.com/Cosmian/kms/pull/196).
- Add build of GCP images (ubuntu/redhat) [#191](https://github.com/Cosmian/kms/pull/191).

### Bug fixes

- Fixing inconsistent crypto consts [#190](https://github.com/Cosmian/kms/pull/190).
- Fix interpolation in error macros [#184](https://github.com/Cosmian/kms/pull/184).
- Move internal KMIP Objects into `Box` to avoid stack memory
  overflow [#200](https://github.com/Cosmian/kms/pull/200).

## [4.12.0] - 2024-02-08

### Features

- Generalize the refresh of JWKS in the middleware [#150](https://github.com/Cosmian/kms/pull/150).
- CI speed up [#173](https://github.com/Cosmian/kms/pull/173).
- Add support for Microsoft Double Key Encryption (DKE)
  endpoints [#170](https://github.com/Cosmian/kms/pull/170).
- Re-organized crypto package by algorithm, removed duplicated
  code [#170](https://github.com/Cosmian/kms/pull/170).
- Add support for FIPS mode for the ckms client [#170](https://github.com/Cosmian/kms/pull/170).
- Documented TOML configuration file for the KMS
  server [#170](https://github.com/Cosmian/kms/pull/170).
- Overall improvements to the documentation on algorithms and FIPS
  mode [#170](https://github.com/Cosmian/kms/pull/170).

## [4.11.3] - 2024-01-26

### Features

- CLI: allow multiple operations to be supplied at once for access
  control [#155](https://github.com/Cosmian/kms/pull/155).

### Miscellaneous Tasks

- Business Source License 1.1

## [4.11.2] - 2024-01-23

### Ci

- Test and deliver in CI missing FIPS binary (fips.so and openssl.cnf for
  FIPS) [#152](https://github.com/Cosmian/kms/issues/153)

## [4.11.1] - 2024-01-18

### Bug Fixes

- Load correct openssl provider on run

### Ci

- Pypi now requires a token to publish [#148](https://github.com/Cosmian/kms/issues/148)

## [4.11.0] - 2024-01-17

### Bug Fixes

- Fix AES decryption: tag invalid size [#133](https://github.com/Cosmian/kms/issues/133)
- Remove bootstrap server leftovers [#142](https://github.com/Cosmian/kms/issues/142)

### Features

- X509 v3 extensions support [#120](https://github.com/Cosmian/kms/issues/120)
- Dynamic salt for password derivation, resolving
  issue [#124](https://github.com/Cosmian/kms/issues/124) [#128](https://github.com/Cosmian/kms/issues/128)
- Support Cosmian VM [#129](https://github.com/Cosmian/kms/issues/129)
- Make rsa oaep aes a generalized encryption system for use in all kms and not only for key
  wrapping [#130](https://github.com/Cosmian/kms/issues/130)
- ECIES implementation for Hybrid Encryption [#134](https://github.com/Cosmian/kms/issues/134)
- Add pyo3 methods for
  symmetric `create_key`, `encrypt`, `decrypt` [#135](https://github.com/Cosmian/kms/issues/135)
- Add RSA keys create key pair [#137](https://github.com/Cosmian/kms/issues/137)
- Upgrade Rust toolchain to 2024-01-09 [#141](https://github.com/Cosmian/kms/issues/141)
- Support keypair generation for curve448 Montgomery and edwards
  forms [#143](https://github.com/Cosmian/kms/issues/143)

## [4.10.1] - 2023-12-12

### Documentation

- Fix mkdocs formatting

## [4.10.0] - 2023-12-11

### Features

- Support for certificate generation using the Certify KMIP operation and a PKCS#10 or a public key
- Support for most standardized encoding formats on import.export: PKCS#8, PKCS#1, SEC1, X509,
  PKCS#12
- Improvements to the Locate functionality for attributes and tags
- Support for the Get Attributes KMIP operation
- Database: support for atomic operations
- Replaced part of Rust Crypto with openssl for more standardized module support
- Deactivated automatic certificate verification, which will be reallocated
  for the future `Validate` KMIP operation support [#102](https://github.com/Cosmian/kms/issues/102)
- Deactivated the non KMIP compliant certificate "quick create feature",
  which can now be achieved using the `Certify` KMIP
  operation [#103](https://github.com/Cosmian/kms/issues/103)

### Bug Fixes

- Redis-Findex: `create` now checks for the pre-existence of the object
- Better KMIP compliance:
  - improved KeyBlock definition [#76](https://github.com/Cosmian/kms/issues/76)
  - enforced KMIP default export formats [#78](https://github.com/Cosmian/kms/issues/78)
  - aligned `Unique Identifier` to KMIP definition but only the `TextString` variant is supported.
  - Use od standards attributes instead of Vendor attributes wherever possible

## [4.9.1] - 2023-11-14

### Bug Fixes

- KMIP server operations only support DER format for certificates (#89)

## [4.9.0] - 2023-11-10

### Bug Fixes

- fix: migrate to num-bigint-dig for bigint (#85)

### Ci

- Test KMS inside an SGX machine

### Features

- Update Covercrypt version to support Policy V2 ([#63])
- Generalize bulk operations using KMIP `Messages` structure

## [4.8.2] - 2023-10-31

### Bug Fixes

- Save certs as DER instead of PEM for KMIP compliance

## [4.8.1] - 2023-10-12

### Bug Fixes

- Fix for [#64](https://github.com/Cosmian/kms/issues/64)

## [4.8.0] - 2023-10-07

### Bug Fixes

- Fix container build on tags
- Serialize the header for each chunk for Covercrypt bulk encryption (#59)

### Features

- KMS running inside TEE (SGX or SEV)
  - review the `verify` subcommand
  - force checking the leaf TLS certificate when querying a KMS running inside a TEE
  - verify RA-TLS certificate before querying the bootstrap server
  - review the TLS certificate generation using the key tied to the TEE
  - remove libsgx and create a new dependance to tee_attestation crate
  - update KMS server argument regarding the TEE and certbot
  - review documentation regarding the KMS usage inside a TEE
- Activate tracing in CLI tests when binary is instrumented ([#56])

### Ci

- Trigger public_documentation build on tags

## [4.7.0] - 2023-10-02

### Features

- Added the wildcard user `*` to grant access rights to all users on an object
- About certificates:
  - add validation of the complete chain instead of the leaf and parent certificates
  - add verifications before using a certificate:
    - check that each certificate is not expired (both chain and leaf certificates)
    - check that no certificate is revoked (both chain and leaf certificates)
    - check that each certificate has a valid signature (both chain and leaf certificates)
    - check that certificate CRL signature is valid
  - add RSA X509 certificate support
  - add Covercrypt bulk encryption
- KMS CLI `ckms`:
  - can import the Mozilla Common CA Database (CCADB)
  - can import a PKCS12 certificate (splitting in 2 KMIP objects: X509 certificate and private
      key)

### Bug Fixes

- Improved database data structures using Maps and Sets instead of Vectors where uniqueness is
  required
- Enable bootstrap server for non "enclaves" servers

## [4.6.0] - 2023-09-01

### Bug Fixes

- Filter Locate request by object type

### Documentation

- Remove merge leftovers

### Features

- bootstrap: the KMS server now supports bootstrap mode to facilitate the secure input of secret
  components, including the database encryption secret and the HTTPS certificate key, directly into
  the encrypted machine memory, through a secure connection
- Add certificate support:
  - in cosmian_kms_server:
    - implement `Certify` KMIP operation
    - in addition, the KMS server will automatically add:
      - the system tag `_cert` on `Certificate` object
      - the system tag `_cert_uid=<certificate_uid>` where `certificate_uid` is used as the
              link between public/private key objects and the related certificate object
      - the system tag `_cert_spki=<Subject Public Key Info>` on `Certificate` object where
              SPKI refers
              to [RFC 5280](https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.7). The SPKI
              value identifies uniquely the underlying certificate
      - the system tag `_cert_ca=<Subject Common Name>` on CA `Certificate` object
    - import X509 certificate as PEM and also import private key as PEM
    - export generic KMIP key `wrapped` by X509 certificate
    - import a wrapped key with unwrapping on the fly
    - encrypt with X509 certificate and decrypt with PKCS8 private key
  - in `ckms`, add:
    - create/destroy certificate
    - export/import certificate
    - revoke certificate
- Add the export wrapping with X509 certificate encryption

## [4.5.0] - 2023-08-21

### Bug Fixes

- Documentation

### Features

- tagging: the KMS server now supports user tagging of objects to facilitate their management.
  Specify as many user tags as needed when creating and importing objects.

In addition, the user server will automatically add a system tag based on the object type:

- `_sk`: for a private key
- `_pk`: for a public key
- `_kk`: for a symmetric key
- `_uk`: for a Covercrypt user decryption key

Use the tags to export objects, locate them, or request data encryption and decryption.

- Added `locate` to the `ckms`client
- Added `Redis-Findex` backend support so that the KMS can encrypt the KMS server data and indexes
  at the application level.
- Added JWE support

## [4.4.3] - 2023-07-17

### Bug Fixes

- Remove RUSTFLAGS for docker container and python package

### Features

- Update sqlx to 0.7 + bitflags and base64-url to version 2

## [4.4.2] - 2023-06-13

### Features

- Support glibc v2.17 when building ckms and cosmian_kms_server

## [4.4.1] - 2023-06-09

### Bug Fixes

- Incorrect docker tag

### Documentation

- Add description on docker usage

## [4.4.0] - 2023-06-06

### Features

- Added the ability to manipulate EC and Symmetric Keys
- Added ECIES encryption (using Curve 25519) and AES GCM encryption
- Added support for policy specifications
- Reworked Revoke and Destroy to be closer to KMIP definitions
- Revoking and Destroying a public/private key, revokes or destroy all the related keys
- Upgrading of SQLX
- Upgrading of PKCS12 support
- Removal of Eyre in the CLI
- Use of cloudproof_rust as a dependency rather than Covercrypt and Crypto Core directly to avoid
  version conflicts
- Authentication:
  - support for more JWT providers
  - support for certificate authentication
- Removal of global static conf and use of proper injection (was hindering testing)
- Authorization: re-factor of endpoints and fix delegation issues around revoke and destroy

### Documentation

- Add link to package.cosmian.com

### Ci

- Add github ci
- Publish python kms packages

### Refactor

- Refactored the server to simplify traits and separate the operations into smaller files

---

## [4.3.4] - 2023-03-09

### Features

- Python KMS client (pyo3): export `database_secret` and `insecure` as parameters

### Testing

- Update cover crypt in python tests

---

## [4.3.3] - 2023-03-02

### Features

- Use CoverCrypt v11

### Refactor

- Removed `mysql` crate used for EdgelessDB (compatible with client SSL connection)
- Use workspace dependencies to ease maintenance

---

## [4.3.2] - 2023-02-17

### Documentation

- mkdocs-merge could not run with `emoji_index` url

---

## [4.3.1] - 2023-02-16

### Ci

- Remove unused docker builds

---

## [4.3.0] - 2023-02-15

### Documentation

- Improves the installation doc and details the important options
- Makes wording coherent between doc and code

### Features

- adds native HTTP/S support by providing certificates
- improves encrypted SQLite support

### Miscellaneous Tasks

- removes multiple features on the KMS server and makes them command-line options.
- There is now a single docker (115MB) that covers all cases (except SGX, which will come later)
- removes the use of crypto_base and makes use of cover-crypt 10.0

---

## [4.2.0] - 2023-01-30

### Features

- Use CoverCrypt v9.0 (post-quantum cryptography) and AbePolicy v3.0
  - Write `Policy` as bytes and `AccessPolicy` as string in Vendor Attributes

### Ci

- Fix tag matching in `python_publish`

---

## [4.1.0] - 2023-01-19

### Added

- add python interface

### Changed

- CoverCrypt v9 / CryptoCore v5 / AbePolicy v3.0
- CLI Policy interface (format changed with AbePolicy v2.1)

---

## [4.0.1] - 2022-11-29

### Changed

- add authentication data in header too in encrypt operation

---

## [4.0.0] - 2022-11-28

### Added

### Changed

- data to encrypt with CoverCrypt is not a JSON anymore but a custom binary format (
  see `DataToEncrypt` struct)
- decrypted data with CoverCrypt is now a custom binary format (see `DecryptedData` struct)

### Fixed

### Removed

---

## [3.0.2] - 2022-11-16

### Added

### Changed

- Support `cover_crypt` 7.1.0

### Fixed

### Removed

---

## [3.0.1] - 2022-11-14

### Added

### Changed

### Fixed

- Rename KMS URL

### Removed

- Remove AVX flag

---

## [3.0.0] - 2022-11-09

### Added

### Changed

- Update `cover_crypt` to 7.0.1
- Update crates dependencies

### Fixed

### Removed

- GPSW support
- TFHE support
- DMCFE support

---

## [2.3.3] - 2022-10-25

### Added

- Handle CORS request

### Changed

- Update installation documentation

### Fixed

### Removed

---
