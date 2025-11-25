use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, Error, SignatureScheme};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use url::Url;

const DEFAULT_GEMINI_PORT: u16 = 1965;

// Custom certificate verifier that accepts all certificates (TOFU model for Gemini)
#[derive(Debug)]
struct AcceptAllCertsVerifier;

impl ServerCertVerifier for AcceptAllCertsVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::ED25519,
        ]
    }
}

#[derive(Debug)]
pub struct GeminiResponse {
    pub status: u8,
    pub meta: String,
    pub body: Vec<u8>,
}

impl GeminiResponse {
    pub fn is_success(&self) -> bool {
        self.status >= 20 && self.status < 30
    }

    pub fn is_redirect(&self) -> bool {
        self.status >= 30 && self.status < 40
    }

    pub fn redirect_url(&self) -> Option<&str> {
        if self.is_redirect() {
            Some(&self.meta)
        } else {
            None
        }
    }
}

pub fn fetch(url_str: &str) -> Result<GeminiResponse, String> {
    let url = Url::parse(url_str).map_err(|e| format!("Invalid URL: {}", e))?;

    if url.scheme() != "gemini" {
        return Err(format!("Not a gemini:// URL: {}", url_str));
    }

    let host = url
        .host_str()
        .ok_or_else(|| "URL has no host".to_string())?;
    let port = url.port().unwrap_or(DEFAULT_GEMINI_PORT);

    // Setup TLS configuration with custom verifier (Gemini uses TOFU model)
    let config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(AcceptAllCertsVerifier))
        .with_no_client_auth();

    let server_name =
        ServerName::try_from(host.to_string()).map_err(|e| format!("Invalid hostname: {}", e))?;

    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name)
        .map_err(|e| format!("TLS connection setup failed: {}", e))?;

    // Connect to server
    let addr = format!("{}:{}", host, port);
    let mut sock =
        TcpStream::connect(&addr).map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

    // Complete TLS handshake
    while conn.is_handshaking() {
        conn.complete_io(&mut sock)
            .map_err(|e| format!("TLS handshake failed: {}", e))?;
    }

    // Send Gemini request: URL + CRLF
    let request = format!("{}\r\n", url_str);
    conn.writer()
        .write_all(request.as_bytes())
        .map_err(|e| format!("Failed to write request: {}", e))?;

    // Flush the TLS stream
    conn.complete_io(&mut sock)
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Read response
    let mut reader = BufReader::new(IoAdapter {
        conn: &mut conn,
        sock: &mut sock,
    });

    // Read status line: <STATUS><SPACE><META><CR><LF>
    let mut status_line = String::new();
    reader
        .read_line(&mut status_line)
        .map_err(|e| format!("Failed to read status line: {}", e))?;

    // Parse status and meta
    let status_line = status_line.trim_end();
    if status_line.len() < 3 {
        return Err(format!("Invalid status line: {}", status_line));
    }

    let status: u8 = status_line[0..2]
        .parse()
        .map_err(|e| format!("Invalid status code: {}", e))?;

    let meta = if status_line.len() > 3 {
        status_line[3..].to_string()
    } else {
        String::new()
    };

    // Read body (everything after status line)
    let mut body = Vec::new();
    reader
        .read_to_end(&mut body)
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    Ok(GeminiResponse { status, meta, body })
}

// Adapter to use rustls ClientConnection with BufReader
struct IoAdapter<'a> {
    conn: &'a mut rustls::ClientConnection,
    sock: &'a mut TcpStream,
}

impl<'a> Read for IoAdapter<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Process TLS data and read decrypted content
        while self.conn.wants_read() {
            match self.conn.read_tls(self.sock) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    self.conn
                        .process_new_packets()
                        .map_err(std::io::Error::other)?;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }

        self.conn.reader().read(buf)
    }
}
