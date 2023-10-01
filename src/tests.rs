use crate::protocol::mcpe::{
    packet::{PlayStatusPacket, RequestNetworkSettingPacket, ResourcePacksStackPacket},
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
fn write_res_stack() -> Result<()> {
    let res_stack = ResourcePacksStackPacket {
        must_accept:true,
        behavior_packs:vec![],
        resource_packs:vec![],
        game_version:"1.20.30".to_owned(),
        experiments:vec![],
        is_experimental:false
    };
    println!("{:?}",encode(res_stack.into(), false)?);
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
