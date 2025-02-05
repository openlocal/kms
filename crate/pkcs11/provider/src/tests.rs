use cosmian_kmip::{
    crypto::symmetric::create_symmetric_key_kmip_object,
    kmip::{
        kmip_data_structures::{KeyBlock, KeyMaterial, KeyValue},
        kmip_objects::Object,
        kmip_types::{CryptographicAlgorithm, KeyFormatType},
    },
};
use cosmian_kms_client::{import_object, KmsClient};
use cosmian_pkcs11_module::traits::Backend;
use kms_test_server::{start_default_test_kms_server, ONCE};

use crate::{backend::CkmsBackend, error::Pkcs11Error, kms_object::get_kms_objects_async};

#[tokio::test]
async fn test_kms_client() -> Result<(), Pkcs11Error> {
    let ctx = ONCE
        .get_or_try_init(start_default_test_kms_server)
        .await
        .unwrap();

    let kms_client = ctx.owner_client_conf.initialize_kms_client()?;
    create_keys(&kms_client).await?;

    let keys = get_kms_objects_async(
        &kms_client,
        &["disk-encryption".to_string()],
        KeyFormatType::Raw,
    )
    .await?;
    assert_eq!(keys.len(), 2);
    let mut labels = keys
        .iter()
        .flat_map(|k| k.other_tags.clone())
        .collect::<Vec<String>>();
    labels.sort();
    assert_eq!(labels, vec!["vol1".to_string(), "vol2".to_string()]);
    Ok(())
}

fn initialize_backend() -> Result<CkmsBackend, Pkcs11Error> {
    cosmian_logger::log_utils::log_init("fatal,cosmian_kms_client=debug");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let owner_client_conf = rt.block_on(async {
        let ctx = ONCE
            .get_or_try_init(start_default_test_kms_server)
            .await
            .unwrap();

        let kms_client = ctx.owner_client_conf.initialize_kms_client().unwrap();
        create_keys(&kms_client).await.unwrap();
        load_p12().await.unwrap();
        ctx.owner_client_conf.clone()
    });

    CkmsBackend::instantiate(owner_client_conf.initialize_kms_client()?)
}

async fn create_keys(kms_client: &KmsClient) -> Result<(), Pkcs11Error> {
    let vol1 = create_symmetric_key_kmip_object(&[1, 2, 3, 4], CryptographicAlgorithm::AES);
    let _vol1_id = import_object(
        kms_client,
        Some("vol1".to_string()),
        vol1,
        None,
        false,
        true,
        ["disk-encryption", "vol1"],
    )
    .await?;

    let vol2 = create_symmetric_key_kmip_object(&[4, 5, 6, 7], CryptographicAlgorithm::AES);
    let _vol2_id = import_object(
        kms_client,
        Some("vol2".to_string()),
        vol2,
        None,
        false,
        true,
        ["disk-encryption", "vol2"],
    )
    .await?;

    Ok(())
}

async fn load_p12() -> Result<String, Pkcs11Error> {
    let ctx = ONCE
        .get_or_try_init(start_default_test_kms_server)
        .await
        .unwrap();

    let kms_client = ctx.owner_client_conf.initialize_kms_client()?;
    let p12_bytes = include_bytes!("../test_data/certificate.p12");

    let p12_sk = Object::PrivateKey {
        key_block: KeyBlock {
            key_format_type: KeyFormatType::PKCS12,
            key_compression_type: None,
            key_value: KeyValue {
                key_material: KeyMaterial::ByteString(zeroize::Zeroizing::new(p12_bytes.to_vec())),
                attributes: None,
            },
            // According to the KMIP spec, the cryptographic algorithm is not required
            // as long as it can be recovered from the Key Format Type or the Key Value.
            // Also it should not be specified if the cryptographic length is not specified.
            cryptographic_algorithm: None,
            // See comment above
            cryptographic_length: None,
            key_wrapping_data: None,
        },
    };

    let p12_id = import_object(
        &kms_client,
        Some("test.p12".to_string()),
        p12_sk,
        None,
        false,
        true,
        ["disk-encryption", "luks_volume"],
    )
    .await?;
    Ok(p12_id)
}

#[test]
fn test_backend() -> Result<(), Pkcs11Error> {
    let backend = initialize_backend()?;

    // data objects
    let data_objects = backend.find_all_data_objects()?;
    assert_eq!(data_objects.len(), 2);
    let mut labels = data_objects
        .iter()
        .map(|dao| dao.label().clone())
        .collect::<Vec<String>>();
    labels.sort();
    assert_eq!(labels, vec!["vol1".to_string(), "vol2".to_string()]);

    // RSA certificate
    let certificates = backend.find_all_certificates()?;
    assert_eq!(certificates.len(), 1);
    assert_eq!(certificates[0].label(), "luks_volume");

    // RSA private key
    let private_keys = backend.find_all_private_keys()?;
    assert_eq!(private_keys.len(), 1);

    Ok(())
}
