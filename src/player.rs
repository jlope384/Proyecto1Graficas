// player.rs

use raylib::prelude::*;
use std::f32::consts::PI;

use crate::maze;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32, // field of view
}

pub fn process_events(player: &mut Player, rl: &RaylibHandle, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;

    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }

    let mut next_pos = player.pos;

    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        next_pos.x -= MOVE_SPEED * player.a.cos();
        next_pos.y -= MOVE_SPEED * player.a.sin();
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        next_pos.x += MOVE_SPEED * player.a.cos();
        next_pos.y += MOVE_SPEED * player.a.sin();
    }
    let i = (next_pos.x as isize) / (block_size as isize);
    let j = (next_pos.y as isize) / (block_size as isize);

    if j >= 0 && j < maze.len() as isize && i >= 0 && i < maze[0].len() as isize {
        // Solo mover si la celda es un espacio vacío
        if maze[j as usize][i as usize] == ' ' {
            player.pos = next_pos;
        }
    }
}