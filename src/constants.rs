use bevy::prelude::Color;

// Neon palette — valores HDR (> 1.0) produzem bloom glow
pub const COLOR_PLAYER: Color = Color::srgb(0.0, 4.0, 8.0);      // ciano brilhante
pub const COLOR_ENEMY: Color = Color::srgb(8.0, 0.5, 0.0);        // laranja neon
pub const COLOR_BULLET_PLAYER: Color = Color::srgb(0.0, 8.0, 2.0); // verde neon
pub const COLOR_BULLET_ENEMY: Color = Color::srgb(8.0, 0.0, 0.5);  // vermelho neon
pub const COLOR_BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.05); // azul escuro quase preto
pub const COLOR_STAR: Color = Color::srgb(1.5, 1.5, 2.0);          // branco azulado suave

// Tamanhos da nave (pixels)
pub const PLAYER_SHIP_SIZE: f32 = 14.0;

// Física do player
pub const PLAYER_SPEED: f32 = 280.0;
pub const PLAYER_ROTATION_SPEED: f32 = 3.5; // radianos por segundo

// Dimensões da janela (half-size para wrap)
pub const HALF_W: f32 = 640.0;
pub const HALF_H: f32 = 360.0;

// Z-layers para ordenação de profundidade
pub const Z_BACKGROUND: f32 = 0.0;
pub const Z_STAR: f32 = 0.5;
pub const Z_PICKUP: f32 = 1.0;
pub const Z_OBSTACLE: f32 = 2.0;
pub const Z_ENEMY: f32 = 3.0;
pub const Z_PLAYER: f32 = 4.0;
pub const Z_BULLET: f32 = 5.0;
pub const Z_VFX: f32 = 6.0;
pub const Z_UI: f32 = 10.0;
