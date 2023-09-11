use anyhow::Result;
use crate::protocol::mcpe::{packet::{RequestNetworkSetting, PlayStatus}, transforms::framer::encode};

#[test]
fn packet_id_macro() {
    let pkt = RequestNetworkSetting {
        client_protocol: 594,
    };
    assert_eq!(pkt.get_id(), 193);
}
#[test]
fn write_play_status() -> Result<()>{
    let mut buf:Vec<u8> = Vec::new();
    let play_status = PlayStatus::FailedClient;
    play_status.read_to_buffer(&mut buf)?;
    println!("{:?}",buf);
    println!("{:?}",encode(play_status.into()));
    Ok(())
}
