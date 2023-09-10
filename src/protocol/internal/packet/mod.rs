#[derive(Debug)]
pub enum InternalPacketKind {
    CreateClient(CreateClient),
    DestoryClient(DestoryClient)
}
#[derive(Debug)]
pub struct CreateClient {
    pub client_id:u64
}

#[derive(Debug)]
pub struct DestoryClient {
    pub client_id:u64
}

macro_rules! packet_impls {
    ($($t:ident),*) => {
        $(
            impl From<$t> for InternalPacketKind {
                fn from(value: $t) -> Self {
                    InternalPacketKind::$t(value)
                }
            }
            impl From<InternalPacketKind> for $t {
                fn from(value: InternalPacketKind) -> Self {
                    match value {
                        InternalPacketKind::$t(kind) => kind,
                        _ => panic!("Invalid PacketKind")
                    }
                }
            }
        )*
    };
}
packet_impls!(CreateClient,DestoryClient);
