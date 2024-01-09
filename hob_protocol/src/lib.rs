pub mod decode;
pub mod encode;
pub mod packet;

#[test]
fn a() {
    use crate::decode::Decoder;
    use crate::encode::Encoder;
    use packet::{DisconnectFailReason, DisconnectPacket, PacketKind};
    use proto_bytes::BytesMut;

    let login: PacketKind = DisconnectPacket {
        reason: DisconnectFailReason::BadPacket,
        hide_disconnect_reason: true,
        message: "aaaa".into(),
    }
    .into();
    let key = [0x42; 32];

    let mut encoder = Encoder::default();
    encoder.setup_cipher(key);

    let ccc = encoder.encode(login);
    println!("{:x?}", ccc);

    // let mut ccc = BytesMut::from_iter(ccc.iter());

    // let mut decoder = Decoder::default();
    // decoder.setup_cipher(key);

    // println!("{:?}",decoder.decode(&mut ccc).unwrap());
}
