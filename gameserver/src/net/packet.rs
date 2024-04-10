use anyhow::Result;
use paste::paste;
use proto::*;
use tracing::Instrument;

use super::{handlers::*, PlayerSession};

const HEAD_MAGIC: u16 = 0x4567;
const TAIL_MAGIC: u16 = 0x89AB;

#[derive(Debug)]
pub struct NetOperation {
    pub head: u32,
    pub param1: u32,
    pub param2: u32,
    pub data: u32,
    pub tail: u32,
}

#[derive(Debug)]
pub struct Packet {
    pub cmd_type: u16,
    pub head: Vec<u8>,
    pub body: Vec<u8>,
}

impl From<&[u8]> for Packet {
    fn from(value: &[u8]) -> Self {
        assert_eq!(
            HEAD_MAGIC,
            u16::from_be_bytes(value[..2].try_into().unwrap())
        );

        let cmd_type = u16::from_be_bytes(value[2..4].try_into().unwrap());
        let head_size = u16::from_be_bytes(value[4..6].try_into().unwrap()) as usize;
        let body_size = u32::from_be_bytes(value[6..10].try_into().unwrap()) as usize;

        assert_eq!(
            TAIL_MAGIC,
            u16::from_be_bytes(value[10 + head_size + body_size..].try_into().unwrap())
        );

        Self {
            cmd_type,
            head: value[10..10 + head_size].to_vec(),
            body: value[10 + head_size..10 + head_size + body_size].to_vec(),
        }
    }
}

impl From<Packet> for Vec<u8> {
    fn from(value: Packet) -> Self {
        let mut buf = Self::with_capacity(12 + value.head.len() + value.body.len());

        buf.extend(HEAD_MAGIC.to_be_bytes());
        buf.extend(value.cmd_type.to_be_bytes());
        buf.extend((value.head.len() as u16).to_be_bytes());
        buf.extend((value.body.len() as u32).to_be_bytes());
        buf.extend(value.head);
        buf.extend(value.body);
        buf.extend(TAIL_MAGIC.to_be_bytes());

        buf
    }
}

impl From<&[u8]> for NetOperation {
    fn from(value: &[u8]) -> Self {
        Self {
            head: u32::from_be_bytes(value[..4].try_into().unwrap()),
            param1: u32::from_be_bytes(value[4..8].try_into().unwrap()),
            param2: u32::from_be_bytes(value[8..12].try_into().unwrap()),
            data: u32::from_be_bytes(value[12..16].try_into().unwrap()),
            tail: u32::from_be_bytes(value[16..20].try_into().unwrap()),
        }
    }
}

impl From<NetOperation> for Vec<u8> {
    fn from(value: NetOperation) -> Self {
        let mut buf = Self::with_capacity(20);
        buf.extend(value.head.to_be_bytes());
        buf.extend(value.param1.to_be_bytes());
        buf.extend(value.param2.to_be_bytes());
        buf.extend(value.data.to_be_bytes());
        buf.extend(value.tail.to_be_bytes());

        buf
    }
}

macro_rules! trait_handler {
    ($($name:ident $cmd_type:expr;)*) => {
        pub trait CommandHandler {
            $(
                paste! {
                    async fn [<on_$name:snake>](session: &PlayerSession, body: &$name) -> Result<()> {
                        [<on_$name:snake>](session, body).await
                    }
                }
            )*

            async fn on_message(session: &PlayerSession, cmd_type: u16, payload: Vec<u8>) -> Result<()> {
                use ::prost::Message;
                match cmd_type {
                    $(
                        $cmd_type => {
                            let body = $name::decode(&mut &payload[..])?;
                            paste! {
                                Self::[<on_$name:snake>](session, &body)
                                    .instrument(tracing::info_span!(stringify!([<on_$name:snake>]), cmd_type = cmd_type))
                                    .await
                            }
                        }
                    )*
                    _ => {
                        tracing::warn!("Unknown command type: {cmd_type}");
                        Ok(())
                    },
                }
            }
        }
    };
}

trait_handler! {
    PingReq 22959;
    GetPlayerTokenReq 4241;
    PlayerLoginReq 22102;
    EnterSceneReadyReq 22284;
    SceneInitFinishReq 29047;
    EnterSceneDoneReq 8282;
    PostEnterSceneReq 8244;
    ChangeGameTimeReq 6910;
    SetUpAvatarTeamReq 8613;
    ChangeAvatarReq 3330;
    EvtAvatarSitDownReq 764;
    EvtAvatarStandUpNotify 8549;
}
