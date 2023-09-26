use crate::protocol::mcpe::{
    packet::{PlayStatusPacket, RequestNetworkSettingPacket},
    transforms::framer::encode,
};
use anyhow::{Ok, Result};

#[test]
fn packet_id_macro() {
    let pkt = RequestNetworkSettingPacket {
        client_protocol: 594,
    };
    assert_eq!(pkt.get_id(), 193);
}
#[test]
fn write_play_status() -> Result<()> {
    let play_status = PlayStatusPacket::FailedClient;
    assert_eq!(encode(play_status.into(), false)?, vec![5, 2, 0, 0, 0, 1]);
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
    assert_eq!(n, result[0..8]);
}
