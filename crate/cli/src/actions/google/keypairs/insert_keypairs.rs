use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::KEYPAIRS_ENDPOINT;
use crate::{actions::google::gmail_client::GmailClient, error::CliError};

/// Creates and uploads a client-side encryption S/MIME public key certificate chain and private key
/// metadata for a user.
#[derive(Parser)]
#[clap(verbatim_doc_comment)]
pub struct InsertKeypairsAction {
    /// The requester's primary email address
    #[clap(long = "user-id", short = 'u', required = true)]
    user_id: String,

    /// Input directory with wrapped key files, with email as basename
    #[clap(long = "inkeydir", short = 'k', required = true)]
    inkeydir: PathBuf,

    /// Input directory with p7 pem certs with extension p7pem, with email as basename
    #[clap(long = "incertdir", short = 'c', required = true)]
    incertdir: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct KeyFile {
    kacls_url: String,
    wrapped_private_key: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct KeyPairInfo {
    pkcs7: String,
    privateKeyMetadata: Vec<PrivateKeyMetadata>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PrivateKeyMetadata {
    kaclsKeyMetadata: KaclsKeyMetadata,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct KaclsKeyMetadata {
    kaclsUri: String,
    kaclsData: String,
}

impl InsertKeypairsAction {
    fn get_input_files(indir: &PathBuf, ext: &str) -> Result<Vec<PathBuf>, CliError> {
        let full_names: Vec<PathBuf> = fs::read_dir(indir)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();

        let all_files: Vec<PathBuf> = full_names.into_iter().filter(|f| f.is_file()).collect();

        let input_files: Vec<PathBuf> = all_files
            .into_iter()
            .filter(|f| f.extension().map_or(false, |e| e == ext))
            .collect();

        Ok(input_files)
    }

    fn get_email_to_file(
        files: &[PathBuf],
        ext: &str,
    ) -> Result<HashMap<String, PathBuf>, CliError> {
        let mut email_file_map = HashMap::new();

        for file in files {
            let fname = file
                .file_name()
                .ok_or_else(|| {
                    CliError::Conversion(format!("cannot get file name from input file {file:?}",))
                })?
                .to_string_lossy();
            let (email, xtn) = match fname.rfind('.') {
                Some(idx) => fname.split_at(idx),
                None => continue,
            };

            if xtn.is_empty() || xtn[1..] != *ext {
                continue;
            }

            email_file_map.insert(email.to_string(), file.clone());
        }

        Ok(email_file_map)
    }

    async fn post_keypairs(
        gmail_client: &GmailClient,
        email_cert_file_map: &HashMap<String, PathBuf>,
        email: &str,
        key_file: &PathBuf,
    ) -> Result<(), CliError> {
        println!("Processing {email:?}.");

        let read_to_string = |path: &PathBuf| -> Result<String, CliError> {
            let mut f = File::open(path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        };

        let key_file = read_to_string(key_file)?;
        let key_file = serde_json::from_str::<KeyFile>(&key_file)?;
        let key_pair_info = KeyPairInfo {
            pkcs7: read_to_string(&email_cert_file_map[email])?,
            privateKeyMetadata: vec![PrivateKeyMetadata {
                kaclsKeyMetadata: KaclsKeyMetadata {
                    kaclsUri: key_file.kacls_url,
                    kaclsData: key_file.wrapped_private_key,
                },
            }],
        };

        let response = gmail_client
            .post(KEYPAIRS_ENDPOINT, serde_json::to_string(&key_pair_info)?)
            .await?;
        let res = GmailClient::handle_response(response).await;
        match res {
            Ok(()) => println!("Keypairs inserted for {email:?}."),
            Err(error) => println!("Error inserting keypairs for {email:?} : {error:?}"),
        }
        Ok(())
    }

    pub async fn run(&self, conf_path: &PathBuf) -> Result<(), CliError> {
        let gmail_client = GmailClient::new(conf_path, &self.user_id).await?;

        let wrapped_key_files = Self::get_input_files(&self.inkeydir, "wrap")?;
        let p7_cert_files = Self::get_input_files(&self.incertdir, "p7pem")?;

        let email_key_file_map = Self::get_email_to_file(&wrapped_key_files, "wrap")?;
        let email_cert_file_map = Self::get_email_to_file(&p7_cert_files, "p7pem")?;

        println!("wrapped_key_files: {wrapped_key_files:?}.");
        println!("p7_cert_files: {p7_cert_files:?}.");

        for (email, key_file) in &email_key_file_map {
            if !email_cert_file_map.contains_key(email) {
                println!("Skipping {email:?}, missing cert file.");
                continue;
            }
            Self::post_keypairs(&gmail_client, &email_cert_file_map, email, key_file).await?;
        }
        Ok(())
    }
}
