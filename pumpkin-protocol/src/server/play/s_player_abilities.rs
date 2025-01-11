use pumpkin_data::packet::serverbound::PLAY_PLAYER_ABILITIES;
use pumpkin_macros::server_packet;
use serde::Deserialize;

//The vanilla client sends this packet when the player starts/stops flying. Bitmask 0x02 is set when the player is flying.

#[derive(Deserialize)]
#[server_packet(PLAY_PLAYER_ABILITIES)]
pub struct SPlayerAbilities {
    pub flags: i8,
}
