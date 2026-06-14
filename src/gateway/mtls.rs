use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;
use tracing::info;

pub fn build_mtls_acceptor(
    cert_path: &str,
    key_path: &str,
    ca_cert_path: &str,
) -> Result<TlsAcceptor, Box<dyn std::error::Error>> {
    let cert_bytes = std::fs::read(cert_path)?;
    let key_bytes = std::fs::read(key_path)?;
    let ca_bytes = std::fs::read(ca_cert_path)?;

    let certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut cert_bytes.as_slice())
        .collect::<Result<Vec<_>, _>>()?;
    let key = rustls_pemfile::private_key(&mut key_bytes.as_slice())?
        .ok_or("no private key found")?;
    let mut root_store = rustls::RootCertStore::empty();
    let ca_certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut ca_bytes.as_slice())
        .collect::<Result<Vec<_>, _>>()?;
    for ca in ca_certs {
        root_store.add(ca)?;
    }

    let config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(
            rustls::server::WebPkiClientVerifier::builder(root_store.into()).build()?,
        )
        .with_single_cert(certs, PrivateKeyDer::try_from(key)?)?;

    info!("mTLS acceptor configured with custom X.509 meter anchors");
    Ok(TlsAcceptor::from(Arc::new(config)))
}
