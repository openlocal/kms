use std::{
    path::PathBuf,
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

use actix_server::ServerHandle;
use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
use cosmian_kms_client::{
    client_bail, client_error,
    cosmian_kmip::crypto::{secret::Secret, symmetric::AES_256_GCM_KEY_LENGTH},
    write_json_object_to_file, ClientConf, ClientError, KmsClient,
};
use cosmian_kms_server::{
    config::{ClapConfig, DBConfig, HttpConfig, HttpParams, JwtAuthConfig, ServerParams},
    core::extra_database_params::ExtraDatabaseParams,
    kms_server::start_kms_server,
};
use tokio::sync::OnceCell;
use tracing::trace;

use crate::test_jwt::{get_auth0_jwt_config, AUTH0_TOKEN};

// use super::extract_uids::extract_database_secret;
// use crate::{
//     actions::shared::utils::write_json_object_to_file, cli_bail, error::CliError, tests::PROG_NAME,
// };

/// In order to run most tests in parallel,
/// we use that to avoid to try to start N KMS servers (one per test)
/// with a default configuration.
/// Otherwise we get: "Address already in use (os error 98)"
/// for N-1 tests.
pub static ONCE: OnceCell<TestsContext> = OnceCell::const_new();

pub struct TestsContext {
    pub owner_client_conf_path: String,
    pub user_client_conf_path: String,
    pub owner_client_conf: ClientConf,
    pub server_handle: ServerHandle,
    pub thread_handle: JoinHandle<Result<(), ClientError>>,
}

impl TestsContext {
    pub async fn stop_server(self) -> Result<(), ClientError> {
        self.server_handle.stop(false).await;
        self.thread_handle
            .join()
            .map_err(|_e| client_error!("failed joining th stop thread"))?
    }
}

/// Start a test KMS server in a thread with the default options:
/// JWT authentication and encrypted database, no TLS
pub async fn start_default_test_kms_server() -> Result<TestsContext, ClientError> {
    start_test_server_with_options(9990, false, true, true).await
}

/// Start a KMS server in a thread with the given options
pub async fn start_test_server_with_options(
    port: u16,
    use_jwt_token: bool,
    use_https: bool,
    use_client_cert: bool,
) -> Result<TestsContext, ClientError> {
    let server_params =
        generate_server_params(port, use_jwt_token, use_https, use_client_cert).await?;

    // Create a (object owner) conf
    let (owner_client_conf_path, mut owner_client_conf) = generate_owner_conf(&server_params)?;
    let kms_client = owner_client_conf.initialize_kms_client()?;

    println!(
        "Starting KMS test server at URL: {} with server params {:?}",
        owner_client_conf.kms_server_url, &server_params
    );

    let (server_handle, thread_handle) =
        start_test_kms_server(server_params).expect("Can't start KMS server");

    // wait for the server to be up
    wait_for_server_to_start(&kms_client)
        .await
        .expect("server timeout");

    // Configure a database and create the kms json file
    let database_secret = kms_client.new_database().await?;

    // Rewrite the conf with the correct database secret
    owner_client_conf.kms_database_secret = Some(database_secret);
    write_json_object_to_file(&owner_client_conf, &owner_client_conf_path)
        .expect("Can't write owner CLI conf path");

    // generate a user conf
    let user_client_conf_path =
        generate_user_conf(port, &owner_client_conf).expect("Can't generate user conf");

    Ok(TestsContext {
        owner_client_conf_path,
        user_client_conf_path,
        owner_client_conf,
        server_handle,
        thread_handle,
    })
}

/// Start a test KMS server with the given config in a separate thread
fn start_test_kms_server(
    server_params: ServerParams,
) -> Result<(ServerHandle, JoinHandle<Result<(), ClientError>>), ClientError> {
    let (tx, rx) = mpsc::channel::<ServerHandle>();

    let thread_handle = thread::spawn(move || {
        // allow others `spawn` to happen within the KMS Server future
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(start_kms_server(server_params, Some(tx)))
            .map_err(|e| ClientError::UnexpectedError(e.to_string()))
    });
    trace!("Waiting for test KMS server to start...");
    let server_handle = rx
        .recv_timeout(Duration::from_secs(25))
        .expect("Can't get test KMS server handle after 25 seconds");
    trace!("... got handle ...");
    Ok((server_handle, thread_handle))
}

/// Wait for the server to start by reading the version
async fn wait_for_server_to_start(kms_client: &KmsClient) -> Result<(), ClientError> {
    // Depending on the running environment, the server could take a bit of time to start
    // We try to query it with a dummy request until be sure it is started.
    let mut retry = true;
    let mut timeout = 5;
    let mut waiting = 1;
    while retry {
        print!("...checking if the server is up...");
        let result = kms_client.version().await;
        if result.is_err() {
            timeout -= 1;
            retry = timeout >= 0;
            if retry {
                println!("The server is not up yet, retrying in {waiting}s... ({result:?}) ",);
                thread::sleep(std::time::Duration::from_secs(waiting));
                waiting *= 2;
            } else {
                println!("The server is still not up, stop trying");
                client_bail!("Can't start the kms server to run tests");
            }
        } else {
            println!("UP!");
            retry = false;
        }
    }
    Ok(())
}

async fn generate_server_params(
    port: u16,
    use_jwt_token: bool,
    use_https: bool,
    use_client_cert: bool,
) -> Result<ServerParams, ClientError> {
    // This create root dir
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Configure the server
    let clap_config = ClapConfig {
        auth: if use_jwt_token {
            get_auth0_jwt_config()
        } else {
            JwtAuthConfig::default()
        },
        db: DBConfig {
            database_type: Some("sqlite-enc".to_string()),
            clear_database: true,
            ..Default::default()
        },
        http: if use_https {
            if use_client_cert {
                HttpConfig {
                    port,
                    https_p12_file: Some(
                        root_dir.join("certificates/server/kmserver.acme.com.p12"),
                    ),
                    https_p12_password: Some("password".to_string()),
                    authority_cert_file: Some(root_dir.join("certificates/server/ca.crt")),
                    ..Default::default()
                }
            } else {
                HttpConfig {
                    port,
                    https_p12_file: Some(
                        root_dir.join("certificates/server/kmserver.acme.com.p12"),
                    ),
                    https_p12_password: Some("password".to_string()),
                    ..Default::default()
                }
            }
        } else {
            HttpConfig {
                port,
                ..Default::default()
            }
        },
        ..Default::default()
    };
    ServerParams::try_from(clap_config)
        .await
        .map_err(|e| ClientError::Default(format!("failed initializing the server config: {e}")))
}

fn generate_owner_conf(server_params: &ServerParams) -> Result<(String, ClientConf), ClientError> {
    // This create root dir
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Create a conf
    let owner_client_conf_path = format!("/tmp/owner_kms_{}.json", server_params.port);

    // Generate a CLI Conf.
    // We will update it later by appending the database secret
    let owner_client_conf = ClientConf {
        kms_server_url: if matches!(server_params.http_params, HttpParams::Https(_)) {
            format!("https://0.0.0.0:{}", server_params.port)
        } else {
            format!("http://0.0.0.0:{}", server_params.port)
        },
        accept_invalid_certs: true,
        kms_access_token: if server_params.identity_provider_configurations.is_some() {
            Some(AUTH0_TOKEN.to_string())
        } else {
            None
        },
        ssl_client_pkcs12_path: if server_params.client_cert.is_some() {
            #[cfg(not(target_os = "macos"))]
            let p = root_dir.join("certificates/owner/owner.client.acme.com.p12");
            #[cfg(target_os = "macos")]
            let p = root_dir.join("certificates/owner/owner.client.acme.com.old.format.p12");
            Some(
                p.to_str()
                    .ok_or(ClientError::Default(
                        "Can't convert path to string".to_string(),
                    ))?
                    .to_string(),
            )
        } else {
            None
        },
        ssl_client_pkcs12_password: if server_params.client_cert.is_some() {
            Some("password".to_string())
        } else {
            None
        },
        // We use the private key since the private key is the public key with additional information.
        ..Default::default()
    };
    // write the conf to a file
    write_json_object_to_file(&owner_client_conf, &owner_client_conf_path)
        .expect("Can't write owner CLI conf path");

    Ok((owner_client_conf_path, owner_client_conf))
}

/// Generate a user configuration for user.client@acme.com and return the file path
fn generate_user_conf(port: u16, owner_client_conf: &ClientConf) -> Result<String, ClientError> {
    // This create root dir
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut user_conf = owner_client_conf.clone();
    user_conf.ssl_client_pkcs12_path = {
        #[cfg(not(target_os = "macos"))]
        let p = root_dir.join("certificates/user/user.client.acme.com.p12");
        #[cfg(target_os = "macos")]
        let p = root_dir.join("certificates/user/user.client.acme.com.old.format.p12");
        Some(
            p.to_str()
                .ok_or(ClientError::Default(
                    "Can't convert path to string".to_string(),
                ))?
                .to_string(),
        )
    };
    user_conf.ssl_client_pkcs12_password = Some("password".to_string());

    // write the user conf
    let user_conf_path = format!("/tmp/user_kms_{port}.json");
    write_json_object_to_file(&user_conf, &user_conf_path)?;

    // return the path
    Ok(user_conf_path)
}

/// Generate an invalid configuration by changing the database secret  and return the file path
#[must_use]
pub fn generate_invalid_conf(correct_conf: &ClientConf) -> String {
    // Create a new database key
    let db_key = Secret::<AES_256_GCM_KEY_LENGTH>::new_random()
        .expect("Failed to generate rand bytes for generate_invalid_conf");

    let mut invalid_conf = correct_conf.clone();
    // and a temp file
    let invalid_conf_path = "/tmp/invalid_conf.json".to_string();
    // Generate a wrong token with valid group id
    let secrets = b64
        .decode(
            correct_conf
                .kms_database_secret
                .as_ref()
                .expect("missing database secret")
                .clone(),
        )
        .expect("Can't decode token");
    let mut secrets =
        serde_json::from_slice::<ExtraDatabaseParams>(&secrets).expect("Can't deserialized token");
    secrets.key = db_key; // bad secret
    let token = b64.encode(serde_json::to_string(&secrets).expect("Can't encode token"));
    invalid_conf.kms_database_secret = Some(token);
    write_json_object_to_file(&invalid_conf, &invalid_conf_path)
        .expect("Can't write CONF_PATH_BAD_KEY");
    invalid_conf_path
}
