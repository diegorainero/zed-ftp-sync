use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::Path;
use suppaftp::FtpStream;
use suppaftp::Mode;
use tokio::fs;
use zed::{Command, Extension, LanguageServerId, Result as ZedResult, Worktree};
use zed_extension_api as zed;

struct FtpSyncExtension {
    config: Option<FtpConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FtpConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    remote_path: String,
    local_path: String,
    auto_sync: bool,
    file_extensions: Vec<String>,
}

impl Default for FtpConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 21,
            username: "user".into(),
            password: "password".into(),
            remote_path: "/var/www/html".into(),
            local_path: ".".into(),
            auto_sync: true,
            file_extensions: vec!["php".into(), "html".into(), "css".into(), "js".into()],
        }
    }
}

impl Extension for FtpSyncExtension {
    fn new() -> Self {
        Self { config: None }
    }

    // Metodo richiesto dal trait
    fn language_server_command(
        &mut self,
        _ls: &LanguageServerId,
        _wt: &Worktree,
    ) -> ZedResult<Command> {
        Err("language_server_command non supportato".into())
    }
}

zed::register_extension!(FtpSyncExtension);

impl FtpSyncExtension {
    fn load_config(&self) -> Result<FtpConfig> {
        let config_path = ".zed-ftp-config.json";
        if Path::new(config_path).exists() {
            let content = std::fs::read_to_string(config_path)?;
            let config: FtpConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Err(anyhow!("File di configurazione non trovato"))
        }
    }

    fn save_config(&self, config: &FtpConfig) -> Result<()> {
        let config_path = ".zed-ftp-config.json";
        let content = serde_json::to_string_pretty(config)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }

    fn create_default_config(&self) -> Result<()> {
        let default_config = FtpConfig::default();
        self.save_config(&default_config)?;
        println!("Creato file di configurazione di default: .zed-ftp-config.json");
        Ok(())
    }

    fn should_sync_file(&self, path: &str, config: &FtpConfig) -> bool {
        if let Some(extension) = Path::new(path).extension() {
            if let Some(ext_str) = extension.to_str() {
                return config.file_extensions.contains(&ext_str.to_string());
            }
        }
        false
    }

    fn get_current_file(&self) -> Option<String> {
        // Questa funzione dovrebbe ottenere il file correntemente aperto in Zed
        // Per ora ritorna None, ma nell'implementazione reale useresti l'API di Zed
        None
    }

    fn open_configuration_dialog(&self) -> Result<()> {
        println!("Per configurare il plugin, modifica il file .zed-ftp-config.json nella root del progetto");
        println!("Esempio di configurazione:");
        let example_config = FtpConfig::default();
        let json = serde_json::to_string_pretty(&example_config)?;
        println!("{}", json);
        Ok(())
    }
}

async fn upload_file(config: &FtpConfig, local_path: &str) -> Result<()> {
    println!("Caricamento di {} su {}...", local_path, config.host);

    // Connetti al server FTP
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", config.host, config.port))?;

    // Login
    ftp_stream.login(&config.username, &config.password)?;

    // Imposta modalità passiva
    ftp_stream.set_mode(Mode::Passive);

    // Leggi il file locale
    let file_content = fs::read(local_path).await?;

    // Calcola il percorso remoto
    let remote_file_path = format!(
        "{}/{}",
        config.remote_path.trim_end_matches('/'),
        Path::new(local_path).file_name().unwrap().to_str().unwrap()
    );

    let mut cursor = Cursor::new(file_content);

    // Carica il file
    ftp_stream.put_file(&remote_file_path, &mut cursor)?;

    // Chiudi la connessione
    ftp_stream.quit()?;

    println!(
        "✓ File {} caricato con successo su {}",
        local_path, remote_file_path
    );
    Ok(())
}

async fn upload_all_files(config: &FtpConfig) -> Result<()> {
    println!("Caricamento di tutti i file del progetto...");

    // Trova tutti i file con le estensioni specificate
    let mut files_to_upload = Vec::new();
    find_files_recursively(
        &config.local_path,
        &config.file_extensions,
        &mut files_to_upload,
    )?;

    // Connetti al server FTP
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", config.host, config.port))?;
    ftp_stream.login(&config.username, &config.password)?;
    ftp_stream.set_mode(Mode::Passive);

    // Carica ogni file
    for file_path in files_to_upload {
        let file_content = fs::read(&file_path).await?;
        let relative_path = file_path.strip_prefix(&config.local_path)?;
        let remote_file_path = format!(
            "{}/{}",
            config.remote_path.trim_end_matches('/'),
            relative_path.to_str().unwrap()
        );

        // Crea le directory se necessario
        if let Some(parent) = Path::new(&remote_file_path).parent() {
            let _ = create_remote_directories(&mut ftp_stream, parent.to_str().unwrap()).await;
        }

        let mut cursor = Cursor::new(file_content);
        ftp_stream.put_file(&remote_file_path, &mut cursor)?;
        println!("✓ {} -> {}", file_path.display(), remote_file_path);
    }

    ftp_stream.quit()?;
    println!("Tutti i file sono stati caricati con successo!");
    Ok(())
}

async fn create_remote_directories(ftp_stream: &mut FtpStream, dir_path: &str) -> Result<()> {
    let parts: Vec<&str> = dir_path.split('/').filter(|s| !s.is_empty()).collect();
    let mut current_path = String::new();

    for part in parts {
        current_path.push('/');
        current_path.push_str(part);

        // Prova a creare la directory (ignora errori se già exists)
        let _ = ftp_stream.mkdir(&current_path);
    }

    Ok(())
}

fn find_files_recursively(
    dir: &str,
    extensions: &[String],
    files: &mut Vec<std::path::PathBuf>,
) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_files_recursively(path.to_str().unwrap(), extensions, files)?;
        } else if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if extensions.contains(&ext_str.to_string()) {
                    files.push(path);
                }
            }
        }
    }
    Ok(())
}
