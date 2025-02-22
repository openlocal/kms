use std::{path::PathBuf, process::Command};

use assert_cmd::prelude::*;
use cosmian_kms_client::{
    cosmian_kmip::kmip::kmip_types::CryptographicAlgorithm, read_object_from_json_ttlv_file,
    KMS_CLI_CONF_ENV,
};
#[cfg(not(feature = "fips"))]
use kms_test_server::{start_default_test_kms_server, ONCE};

#[cfg(not(feature = "fips"))]
use crate::tests::{
    cover_crypt::master_key_pair::create_cc_master_key_pair,
    elliptic_curve::create_key_pair::create_ec_key_pair,
    symmetric::create_key::create_symmetric_key,
};
use crate::{
    actions::shared::{import_key::ImportKeyFormat, utils::KeyUsage},
    error::CliError,
    tests::{
        shared::export::export_key,
        utils::{extract_uids::extract_imported_key_id, recover_cmd_logs},
        PROG_NAME,
    },
};

#[allow(clippy::too_many_arguments)]
pub fn import_key(
    cli_conf_path: &str,
    sub_command: &str,
    key_file: &str,
    key_format: Option<ImportKeyFormat>,
    key_id: Option<String>,
    tags: &[String],
    key_usage_vec: Option<Vec<KeyUsage>>,
    unwrap: bool,
    replace_existing: bool,
) -> Result<String, CliError> {
    let mut cmd = Command::cargo_bin(PROG_NAME)?;
    cmd.env(KMS_CLI_CONF_ENV, cli_conf_path);
    cmd.env("RUST_LOG", "cosmian_kms_cli=info");
    let mut args: Vec<String> = vec!["keys".to_owned(), "import".to_owned(), key_file.to_owned()];
    if let Some(key_id) = key_id {
        args.push(key_id);
    }
    for tag in tags {
        args.push("--tag".to_owned());
        args.push(tag.clone());
    }
    if let Some(key_format) = key_format {
        args.push("--key-format".to_owned());
        let kfs = match key_format {
            ImportKeyFormat::JsonTtlv => "json-ttlv",
            ImportKeyFormat::Pem => "pem",
            ImportKeyFormat::Sec1 => "sec1",
            ImportKeyFormat::Pkcs1Priv => "pkcs1-priv",
            ImportKeyFormat::Pkcs1Pub => "pkcs1-pub",
            ImportKeyFormat::Pkcs8 => "pkcs8",
            ImportKeyFormat::Spki => "spki",
            ImportKeyFormat::Aes => "aes",
            ImportKeyFormat::Chacha20 => "chacha20",
        };
        args.push(kfs.to_string());
    }
    if let Some(key_usage_vec) = key_usage_vec {
        for key_usage in key_usage_vec {
            args.push("--key-usage".to_owned());
            args.push(key_usage.into());
        }
    }
    if unwrap {
        args.push("-u".to_owned());
    }
    if replace_existing {
        args.push("-r".to_owned());
    }
    cmd.arg(sub_command).args(args);
    let output = recover_cmd_logs(&mut cmd);
    if output.status.success() {
        let import_output = std::str::from_utf8(&output.stdout)?;
        let imported_key_id = extract_imported_key_id(import_output)
            .ok_or_else(|| CliError::Default("failed extracting the imported key id".to_owned()))?
            .to_owned();
        return Ok(imported_key_id)
    }
    Err(CliError::Default(
        std::str::from_utf8(&output.stderr)?.to_owned(),
    ))
}

#[cfg(not(feature = "fips"))]
#[tokio::test]
pub async fn test_import_cover_crypt() -> Result<(), CliError> {
    let ctx = ONCE.get_or_try_init(start_default_test_kms_server).await?;

    let uid: String = import_key(
        &ctx.owner_client_conf_path,
        "cc",
        "test_data/ttlv_public_key.json",
        None,
        None,
        &[],
        None,
        false,
        false,
    )?;
    assert_eq!(uid.len(), 36);

    // reimporting the same key  with the same id should fail
    assert!(
        import_key(
            &ctx.owner_client_conf_path,
            "cc",
            "test_data/ttlv_public_key.json",
            None,
            Some(uid.clone()),
            &[],
            None,
            false,
            false,
        )
        .is_err()
    );

    //...unless we force it with replace_existing
    let uid_: String = import_key(
        &ctx.owner_client_conf_path,
        "cc",
        "test_data/ttlv_public_key.json",
        None,
        Some(uid.clone()),
        &[],
        None,
        false,
        true,
    )?;
    assert_eq!(uid_, uid);

    Ok(())
}

#[cfg(not(feature = "fips"))]
#[tokio::test]
pub async fn test_generate_export_import() -> Result<(), CliError> {
    // log_init("cosmian_kms_server=debug,cosmian_kms_utils=debug");
    let ctx = ONCE.get_or_try_init(start_default_test_kms_server).await?;

    // Covercrypt import/export test
    let (private_key_id, _public_key_id) = create_cc_master_key_pair(
        &ctx.owner_client_conf_path,
        "--policy-specifications",
        "test_data/policy_specifications.json",
        &[],
    )?;
    export_import_test(
        &ctx.owner_client_conf_path,
        "cc",
        &private_key_id,
        CryptographicAlgorithm::CoverCrypt,
    )?;

    // Test import/export of an EC Key Pair
    let (private_key_id, _public_key_id) =
        create_ec_key_pair(&ctx.owner_client_conf_path, "nist-p256", &[])?;
    export_import_test(
        &ctx.owner_client_conf_path,
        "ec",
        &private_key_id,
        CryptographicAlgorithm::ECDH,
    )?;

    // generate a symmetric key
    let key_id = create_symmetric_key(&ctx.owner_client_conf_path, None, None, None, &[])?;
    export_import_test(
        &ctx.owner_client_conf_path,
        "sym",
        &key_id,
        CryptographicAlgorithm::AES,
    )?;

    Ok(())
}

#[allow(dead_code)]
pub fn export_import_test(
    cli_conf_path: &str,
    sub_command: &str,
    private_key_id: &str,
    algorithm: CryptographicAlgorithm,
) -> Result<(), CliError> {
    // Export
    export_key(
        cli_conf_path,
        sub_command,
        private_key_id,
        "/tmp/output.export",
        None,
        false,
        None,
        false,
    )?;
    let object = read_object_from_json_ttlv_file(&PathBuf::from("/tmp/output.export"))?;
    let key_bytes = object.key_block()?.key_bytes()?;

    // import and re-export
    let uid: String = import_key(
        cli_conf_path,
        sub_command,
        "/tmp/output.export",
        None,
        None,
        &[],
        None,
        false,
        false,
    )?;
    export_key(
        cli_conf_path,
        sub_command,
        &uid,
        "/tmp/output2.export",
        None,
        false,
        None,
        false,
    )?;
    let object2 = read_object_from_json_ttlv_file(&PathBuf::from("/tmp/output2.export"))?;
    assert_eq!(object2.key_block()?.key_bytes()?, key_bytes);
    assert_eq!(
        object2.key_block()?.cryptographic_algorithm,
        Some(algorithm)
    );
    assert!(object2.key_block()?.key_wrapping_data.is_none());

    Ok(())
}
