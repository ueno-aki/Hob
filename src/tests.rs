use crate::protocol::mcpe::{
    crypto::cipher::Aes256Ctr64BE,
    packet::{PlayStatus, RequestNetworkSetting},
    transforms::framer::encode,
};
use aes::cipher::{KeyIvInit, StreamCipher};
use anyhow::{Ok, Result};

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
    println!("{:?}", encode(play_status.into(), false));
    Ok(())
}
#[test]
fn write_play_status_encrypted() -> Result<()> {
    let ss: [u8; 32] = [
        97, 160, 94, 73, 44, 96, 222, 15, 164, 99, 156, 254, 55, 25, 124, 119, 215, 168, 192, 40,
        163, 56, 0, 101, 223, 190, 165, 130, 206, 171, 178, 160,
    ];
    let iv: [u8; 16] = [ss.clone()[0..12].to_vec(), vec![0, 0, 0, 2]]
        .concat()
        .try_into()
        .unwrap();
    let mut cipher = Aes256Ctr64BE::new(&ss.into(), &iv.into());
    let mut pkt = [99, 101, 98, 0, 2, 0, 82, 84, 252, 137, 166, 12, 166, 205];
    cipher.apply_keystream(&mut pkt);
    println!("{:?}", pkt);

    let plain = [99, 101, 98, 0, 2, 0];
    let counter: u64 = 0;
    let mut digest = hmac_sha256::Hash::new();
    digest.update(counter.to_be_bytes());
    digest.update(plain);
    digest.update(ss);
    let result = digest.finalize();
    println!("{result:?}");
    Ok(())
}
#[test]
fn compute_checksum() {
    let n: [u8; 8] = [174, 130, 246, 122, 1, 253, 36, 57];
    let plain: [u8; 4] = [99, 100, 1, 0];
    let ss: [u8; 32] = [
        85, 80, 17, 129, 34, 26, 176, 129, 54, 165, 3, 18, 177, 65, 59, 221, 149, 204, 183, 187,
        138, 58, 220, 198, 235, 193, 47, 92, 116, 162, 26, 176,
    ];
    let counter: u64 = 0;
    let mut digest = hmac_sha256::Hash::new();
    digest.update(counter.to_be_bytes());
    digest.update(plain);
    digest.update(ss);
    let result = digest.finalize();
    println!("{result:?}");
    assert_eq!(n, result[0..8]);
}
