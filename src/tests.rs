use crate::protocol::packet::RequestNetworkSetting;

#[test]
fn packet_id_macro() {
    let pkt = RequestNetworkSetting{client_protocol:594};
    assert_eq!(pkt.get_id(),193);
}
