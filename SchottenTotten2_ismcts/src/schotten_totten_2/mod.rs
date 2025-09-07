pub mod com_st_proto {
    include!(concat!(env!("OUT_DIR"), "/com.st.proto.rs"));
}
pub mod schotten_totten_2_state;
pub mod card;
pub mod wall_tile;
pub mod player;
pub mod types;
pub mod r#move;