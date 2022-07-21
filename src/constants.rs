pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1,0.1,0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

pub mod locations {
    pub mod temple {
        pub const BACKGROUND_Z: f32 = 0.0;

        pub const TEMPLE_Z: f32 = 2.0;

        pub const THRONE_Z_BACK: f32 = 3.0;
        pub const THRONE_Z_FRONT: f32 = 6.0;
        pub const THRONE_POSITION: (f32, f32, f32) = (0.0, 0.0, THRONE_Z_BACK);
    }
}

