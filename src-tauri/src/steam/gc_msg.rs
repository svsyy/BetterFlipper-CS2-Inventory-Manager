

use std::fmt::Debug;
use std::io::Write;

use steam_vent::ConnectionTrait;
use steam_vent::message::{EncodableMessage, NetMessage};
use steam_vent::NetworkError;
use steam_vent::GameCoordinator;
use steam_vent_proto_common::protobuf::Message as ProtoMessage;
use steam_vent_proto_csgo::econ_gcmessages::EGCItemMsg;
use steam_vent_proto_csgo::gcsdk_gcmessages::{CMsgSOSingleObject, CMsgSOMultipleObjects};
use steam_vent_proto_csgo::gcsystemmsgs::ESOMsg;

#[derive(Debug)]
pub struct ProtoGcMsg<T: ProtoMessage + Debug + Default>(pub T);

impl<T: ProtoMessage + Debug + Default + Send + 'static> EncodableMessage for ProtoGcMsg<T> {
    fn write_body<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        self.0.write_to_writer(&mut writer)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
    fn encode_size(&self) -> usize {
        self.0.compute_size() as usize
    }
}

impl<T: ProtoMessage + Debug + Default + Send + 'static> NetMessage for ProtoGcMsg<T> {
    type KindEnum = EGCItemMsg;
    
    const KIND: Self::KindEnum = EGCItemMsg::k_EMsgGCBase;
    const IS_PROTOBUF: bool = true;
}

#[derive(Debug)]
pub struct RawGcMsg(pub Vec<u8>);

impl EncodableMessage for RawGcMsg {
    fn write_body<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        writer.write_all(&self.0)
    }
    fn encode_size(&self) -> usize { self.0.len() }
}

impl NetMessage for RawGcMsg {
    type KindEnum = EGCItemMsg;
    const KIND: Self::KindEnum = EGCItemMsg::k_EMsgGCBase;
    const IS_PROTOBUF: bool = false;
}

pub async fn send_proto<T>(gc: &GameCoordinator, kind: EGCItemMsg, body: T) -> Result<(), NetworkError>
where
    T: ProtoMessage + Debug + Default + Send + 'static,
{
    gc.send_with_kind(ProtoGcMsg(body), kind).await
}

pub async fn send_raw(gc: &GameCoordinator, kind: EGCItemMsg, body: Vec<u8>) -> Result<(), NetworkError> {
    gc.send_with_kind(RawGcMsg(body), kind).await
}

macro_rules! so_wrapper {
    ($name:ident, $inner:ty, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct $name(pub $inner);

        impl ::steam_vent_proto_common::protobuf::Message for $name {
            const NAME: &'static str = stringify!($name);
            fn is_initialized(&self) -> bool { self.0.is_initialized() }
            fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>)
                -> ::steam_vent_proto_common::protobuf::Result<()> {
                self.0.merge_from(is)
            }
            fn compute_size(&self) -> u64 { self.0.compute_size() }
            fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>)
                -> ::steam_vent_proto_common::protobuf::Result<()> {
                self.0.write_to_with_cached_sizes(os)
            }
            fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
                self.0.special_fields()
            }
            fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
                self.0.mut_special_fields()
            }
            fn new() -> Self { Self(<$inner>::new()) }
            fn default_instance() -> &'static Self {
                use ::std::sync::OnceLock;
                static D: OnceLock<$name> = OnceLock::new();
                D.get_or_init(|| $name(<$inner>::new()))
            }
        }

        impl ::steam_vent_proto_common::RpcMessage for $name {
            fn parse(reader: &mut dyn ::std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
                use ::steam_vent_proto_common::protobuf::Message;
                <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
            }
            fn write(&self, writer: &mut dyn ::std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
                use ::steam_vent_proto_common::protobuf::Message;
                self.write_to_writer(writer)
            }
            fn encode_size(&self) -> usize {
                use ::steam_vent_proto_common::protobuf::Message;
                self.compute_size() as usize
            }
        }

        impl ::steam_vent_proto_common::RpcMessageWithKind for $name {
            type KindEnum = ESOMsg;
            const KIND: Self::KindEnum = $kind;
        }
    };
}

so_wrapper!(SoSingleCreate, CMsgSOSingleObject, ESOMsg::k_ESOMsg_Create);
so_wrapper!(SoSingleUpdate, CMsgSOSingleObject, ESOMsg::k_ESOMsg_Update);
so_wrapper!(SoSingleDestroy, CMsgSOSingleObject, ESOMsg::k_ESOMsg_Destroy);
so_wrapper!(SoMultipleUpdate, CMsgSOMultipleObjects, ESOMsg::k_ESOMsg_UpdateMultiple);
