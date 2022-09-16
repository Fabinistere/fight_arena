pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1,0.1,0.1);

pub const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

pub mod locations {
    pub mod temple {
        use crate::TILE_SIZE;

        pub const BACKGROUND_Z: f32 = 0.;

        // Vec3::splat(TILE_SIZE*0.1)
        pub const TEMPLE_SCALE: (f32, f32, f32) = (-TILE_SIZE*0.1, TILE_SIZE*0.1, TILE_SIZE*0.1);
        pub const TEMPLE_Z: f32 = 2.0;
        pub const TEMPLE_POSITION: (f32, f32, f32) = (0., 0., TEMPLE_Z);
        
        pub const BANNERS_POSITION: (f32, f32, f32) = (0.23, 0.935, THRONE_Z_BACK);

        pub const THRONE_Z_BACK: f32 = 3.0;
        pub const THRONE_Z_FRONT: f32 = 7.0;
        pub const THRONE_POSITION: (f32, f32, f32) = (0.23, 0.74, THRONE_Z_BACK);

        pub const PILLAR_Z_BACK: f32 = 3.0;
        pub const PILLAR_Z_FRONT: f32 = 7.0;

        // make the pillar above and on coherence with the pillar hitbox
        pub const PILLAR_ADJUST: f32 = 0.01;

        pub const PILLAR_POSITION_1: (f32, f32, f32) = (0.485, 0.28, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_2: (f32, f32, f32) = (0.485, -0.12, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_3: (f32, f32, f32) = (0.485, -0.52, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_4: (f32, f32, f32) = (-0.025, 0.28, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_5: (f32, f32, f32) = (-0.025, -0.12, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_6: (f32, f32, f32) = (-0.025, -0.52, PILLAR_Z_BACK);
    }
}

pub mod npc {

    pub const NPC_SCALE: f32 = 2.;

    pub const NPC_Z_BACK: f32 = 2.;
    pub const NPC_Z_FRONT: f32 = 8.;

    pub mod movement {
        pub const REST_TIMER: u64 = 3;
        pub const NPC_SPEED_LEADER: f32 = 0.7;
        pub const NPC_SPEED: f32 = 0.5; // -> Speed::default()
    }
}

pub mod player {
    pub const PLAYER_SCALE: f32 = 2.;
    pub const PLAYER_Z: f32 = 6.;
    
    pub const PLAYER_HP: i32 = 50;
    pub const PLAYER_MANA: i32 = 100;
    pub const PLAYER_INITIATIVE: i32 = 40;
    pub const PLAYER_ATTACK: i32 = 10;
    pub const PLAYER_ATTACK_SPE: i32 = 30;
    pub const PLAYER_DEFENSE: i32 = 0;
    pub const PLAYER_DEFENSE_SPE: i32 = 10;
}

pub mod combat {
    pub mod team {
        pub const TEAM_MC: i32 = 0;
        pub const TEAM_OLF: i32 = 1;
        pub const TEAM_FABICURION: i32 = 2;
    }
}

