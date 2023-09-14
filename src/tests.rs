use crate::protocol::mcpe::{
    crypto::es384::{ES384Header, ES384PrivateKey},
    packet::{PlayStatus, RequestNetworkSetting},
    transforms::framer::encode,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[test]
fn packet_id_macro() {
    let pkt = RequestNetworkSetting {
        client_protocol: 594,
    };
    assert_eq!(pkt.get_id(), 193);
}
#[test]
fn write_play_status() -> Result<()> {
    let mut buf: Vec<u8> = Vec::new();
    let play_status = PlayStatus::FailedClient;
    play_status.read_to_buffer(&mut buf)?;
    println!("{:?}", buf);
    println!("{:?}", encode(play_status.into()));
    Ok(())
}
#[test]
fn sign_jwt() {
    let secret = ES384PrivateKey::generate();
    let secret_pem = secret.to_pem().unwrap();
    let pub_key_pem = secret.public_key().to_pem().unwrap();

    let header = ES384Header {
        alg:"ES384".to_owned(),
        x5u:"MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAECRXueJeTDqNRRgJi/vlRufByu/2G0i2Ebt6YMar5QX/R0DIIyrJMcUpruK4QveTfJSTp3Shlq4Gk34cD/4GUWwkv0DVuzeuB+tXija7HBxii03NHDbPAD0AKnLr2wdAp".to_owned()
    };
    let claim = NoCustomClaim {
        salt: base64::encode("🧂"),
    };
    let token = secret.sign(header, claim).unwrap();

    println!("{}\n{}\n{}", token, pub_key_pem, secret_pem)
    //https://jwt.io/
}

#[derive(Serialize, Deserialize)]
struct NoCustomClaim {
    salt: String,
}
