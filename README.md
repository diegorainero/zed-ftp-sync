# Plugin FTP Sync per Zed

Un plugin per Zed Editor che sincronizza automaticamente i file modificati via FTP sul server remoto.

## Funzionalità

- ✅ Sincronizzazione automatica dei file quando vengono modificati
- ✅ Upload manuale del file corrente
- ✅ Upload di tutti i file del progetto
- ✅ Configurazione tramite file JSON
- ✅ Supporto per filtri per estensione file
- ✅ Creazione automatica delle directory remote
- ✅ Modalità passiva FTP

## Installazione

### 1. Preparazione dell'ambiente

Assicurati di avere Rust installato:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Creazione del plugin

1. Crea una nuova directory per il plugin:
```bash
mkdir zed-ftp-sync
cd zed-ftp-sync
```

2. Copia i file `Cargo.toml` e `src/lib.rs` nelle rispettive posizioni
3. Crea la struttura delle directory:
```bash
mkdir src
# Copia lib.rs in src/lib.rs
# Copia Cargo.toml nella root
```

### 3. Compilazione

```bash
cargo build --release
```

### 4. Installazione in Zed

1. Trova la directory dei plugin di Zed:
   - Linux: `~/.config/zed/extensions/`
   - macOS: `~/Library/Application Support/Zed/extensions/`
   - Windows: `%APPDATA%/Zed/extensions/`

2. Crea una directory per il plugin:
```bash
mkdir -p ~/.config/zed/extensions/ftp-sync
```

3. Copia il file compilato:
```bash
cp target/release/libzed_ftp_sync.so ~/.config/zed/extensions/ftp-sync/
# Su macOS: libzed_ftp_sync.dylib
# Su Windows: zed_ftp_sync.dll
```

## Configurazione

### 1. File di configurazione

Crea un file `.zed-ftp-config.json` nella root del tuo progetto:

```json
{
  "host": "tuo-server.com",
  "port": 21,
  "username": "tuo-username",
  "password": "tua-password",
  "remote_path": "/var/www/html",
  "local_path": ".",
  "auto_sync": true,
  "file_extensions": [
    "php",
    "html",
    "css",
    "js",
    "json",
    "md"
  ]
}
```

### 2. Parametri di configurazione

- **host**: Indirizzo del server FTP
- **port**: Porta del server FTP (di solito 21)
- **username**: Nome utente FTP
- **password**: Password FTP
- **remote_path**: Percorso remoto dove caricare i file
- **local_path**: Percorso locale del progetto (di solito ".")
- **auto_sync**: Se true, i file vengono caricati automaticamente quando modificati
- **file_extensions**: Array delle estensioni file da sincronizzare

## Utilizzo

### Comandi disponibili

Il plugin aggiunge questi comandi alla palette dei comandi di Zed:

1. **FTP Sync: Upload Current File** (`ftp-sync:upload-current-file`)
   - Carica il file attualmente aperto

2. **FTP Sync: Upload All Files** (`ftp-sync:upload-all`)
   - Carica tutti i file del progetto con le estensioni specificate

3. **FTP Sync: Toggle Auto Sync** (`ftp-sync:toggle-auto-sync`)
   - Attiva/disattiva la sincronizzazione automatica

4. **FTP Sync: Configure** (`ftp-sync:configure`)
   - Mostra informazioni sulla configurazione

### Utilizzo dei comandi

1. Apri la palette dei comandi in Zed (Cmd/Ctrl + Shift + P)
2. Digita "FTP Sync" per vedere tutti i comandi disponibili
3. Seleziona il comando desiderato

### Sincronizzazione automatica

Quando `auto_sync` è impostato su `true`, il plugin:
- Monitora i file modificati
- Carica automaticamente i file che corrispondono alle estensioni specificate
- Mostra messaggi di stato nella console

## Sicurezza

⚠️ **Importante**: Il file di configurazione contiene credenziali in chiaro. Assicurati di:

1. Aggiungere `.zed-ftp-config.json` al tuo `.gitignore`:
```bash
echo ".zed-ftp-config.json" >> .gitignore
```

2. Utilizzare credenziali FTP dedicate con permessi limitati
3. Considerare l'uso di SFTP per connessioni più sicure (richiede modifiche al codice)

## Troubleshooting

### Errori comuni

1. **"Connection refused"**
   - Verifica host e porta
   - Assicurati che il server FTP sia raggiungibile

2. **"Login failed"**
   - Controlla username e password
   - Verifica che l'account FTP sia attivo

3. **"Permission denied"**
   - Verifica che l'utente FTP abbia permessi di scrittura
   - Controlla il percorso remoto

4. **File non caricati automaticamente**
   - Verifica che `auto_sync` sia `true`
   - Controlla che l'estensione del file sia nell'array `file_extensions`

### Log e debugging

Il plugin stampa messaggi di stato e errori nella console. Per vedere i log:
1. Apri la console di Zed (se disponibile)
2. Oppure controlla il terminal da cui hai avviato Zed

## Sviluppo

Per modificare il plugin:

1. Modifica il codice in `src/lib.rs`
2. Ricompila: `cargo build --release`
3. Sostituisci il file nella directory dei plugin di Zed
4. Riavvia Zed per applicare le modifiche

## Contributi

Sentiti libero di:
- Segnalare bug
- Proporre nuove funzionalità
- Inviare pull request
- Migliorare la documentazione

## Licenza

MIT License - vedi il file LICENSE per i dettagli.
