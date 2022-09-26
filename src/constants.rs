//! Constants
//! 
//! 1 == one pixel
//! 0.6 == characters' pixel
//! magical number = ratio

pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1,0.1,0.1);

pub const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 1.;

pub mod locations {
    pub mod temple {
        use crate::TILE_SIZE;


        pub const BACKGROUND_Z: f32 = 0.;

        // Vec3::splat(TILE_SIZE*0.1)
        pub const TEMPLE_SCALE: (f32, f32, f32) = (-1., 1., 1.);
        pub const TEMPLE_Z: f32 = 2.0;
        pub const TEMPLE_POSITION: (f32, f32, f32) = (0., 0., TEMPLE_Z);
        
        pub const BANNERS_POSITION: (f32, f32, f32) = (23.*TILE_SIZE, 935.*TILE_SIZE, THRONE_Z_BACK);

        pub const THRONE_Z_BACK: f32 = 3.;
        pub const THRONE_Z_FRONT: f32 = 7.;
        pub const THRONE_POSITION: (f32, f32, f32) = (23.*TILE_SIZE, 74.*TILE_SIZE, THRONE_Z_BACK);

        pub const PILLAR_Z_BACK: f32 = 3.;
        pub const PILLAR_Z_FRONT: f32 = 7.;

        // make the pillar above and on coherence with the pillar hitbox
        // used in the pillar z position
        pub const PILLAR_ADJUST: f32 = 3.;
        pub const PILLAR_HITBOX_Y_OFFSET: f32 = -12.5;

        pub const PILLAR_POSITION_1: (f32, f32, f32) = (48.5*TILE_SIZE, 28.*TILE_SIZE, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_2: (f32, f32, f32) = (48.5*TILE_SIZE, -12.*TILE_SIZE, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_3: (f32, f32, f32) = (48.5*TILE_SIZE, -52.*TILE_SIZE, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_4: (f32, f32, f32) = (-02.5*TILE_SIZE, 28.*TILE_SIZE, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_5: (f32, f32, f32) = (-02.5*TILE_SIZE, -12.*TILE_SIZE, PILLAR_Z_BACK);
        pub const PILLAR_POSITION_6: (f32, f32, f32) = (-02.5*TILE_SIZE, -52.*TILE_SIZE, PILLAR_Z_BACK);
    }
}

pub mod character {

    use super::TILE_SIZE;

    pub const CHAR_SCALE: f32 = 0.6 * TILE_SIZE;

    pub const CHAR_HITBOX_HEIGHT: f32 = 1.5 * CHAR_SCALE;
    pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;
    pub const CHAR_HITBOX_Y_OFFSET: f32 = -8.5 * CHAR_SCALE;

    pub mod npc {

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;
    
        pub const NPC_Z_BACK: f32 = 2.;
        pub const NPC_Z_FRONT: f32 = 8.;

        pub const ADMIRAL_STARTING_SS: usize = 0;

        pub const OLF_STARTING_SS: usize = 12;

        pub const HUGO_STARTING_SS: usize = 16;
    
        pub mod movement {
            use crate::TILE_SIZE;

            pub const REST_TIMER: u64 = 3;
            pub const NPC_SPEED_LEADER: f32 = 70. * TILE_SIZE;
            pub const NPC_SPEED: f32 = 50. * TILE_SIZE; // -> Speed::default()
        }
    }
    
    pub mod player {
    
        pub const PLAYER_SCALE: f32 = super::CHAR_SCALE;
        pub const PLAYER_Z: f32 = 6.;
        
        pub const PLAYER_HP: i32 = 50;
        pub const PLAYER_MANA: i32 = 100;
        pub const PLAYER_INITIATIVE: i32 = 40;
        pub const PLAYER_ATTACK: i32 = 10;
        pub const PLAYER_ATTACK_SPE: i32 = 30;
        pub const PLAYER_DEFENSE: i32 = 0;
        pub const PLAYER_DEFENSE_SPE: i32 = 10;
    }
}

pub mod combat {
    pub mod team {
        pub const TEAM_MC: i32 = 0;
        pub const TEAM_OLF: i32 = 1;
        pub const TEAM_FABICURION: i32 = 2;
    }
}

pub mod ui {
    pub mod dialogs {
        pub const DIALOG_BOX_ANIMATION_OFFSET: f32 = -1000.0;
        pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
        pub const DIALOG_BOX_ANIMATION_TIME_MS: u64 = 500;
        pub const SCROLL_SIZE: (f32, f32) = (490.0, 11700.0 / 45.0);
        pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
        pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;
    }
}
