use hecs::World;
use macroquad::prelude::*;

use crate::exit_modes::ExitMode;
use crate::systems::{
    animated_map_renderer, bullet, fire_control, life, map_renderer, path_follower, physics,
    player_control, rect_renderer, take_bullet_damage, enemy_fire
};

pub async fn game() -> ExitMode {
    let mut world = World::new();

    let player = world.spawn((
        player_control::PlayerControl::new(10.),
        fire_control::FireControl::new(30),
        map_renderer::MapRenderer::new(
            vec!["  &  ", " ### ", "#####"].into(),
            vec![(' ', Color::from_rgba(0, 0, 0, 0)), ('#', RED), ('&', BLUE)].into(),
        ),
        physics::Position::new(screen_width() / 2. - 15., screen_height() - 50.),
        physics::Size::new(30., 30.),
        physics::Velocity::new(0., 0.),
    ));

    let _ennemy = world.spawn((
        animated_map_renderer::AnimatedMapRenderer::new(
            vec![
                (vec![" ### ", "#@#@#", "#####", "&   &", " & & "].into(), 5),
                (vec![" ### ", "#@#@#", "#####", "&   &", "&   &"].into(), 5),
            ],
            vec![
                (' ', Color::from_rgba(0, 0, 0, 0)),
                ('#', RED),
                ('@', YELLOW),
                ('&', BLUE),
            ]
            .into(),
        ),
        path_follower::PathFollower::new(
            50.,
            vec![(100., 100.), (200., 200.), (200., 100.), (100., 200.)],
        ),
        life::Life::new(
            5,
            Box::new(move |_w, _e| {
                println!("Death !");
            }),
        ),
        take_bullet_damage::TakeBulletDamage::new(Box::new(move |_w, _e| {
            println!("Touched !");
        })),
        enemy_fire::EnemyFire::new(20..30),
        physics::Position::new(100., 100.),
        physics::Size::new(35., 35.),
        physics::Velocity::new(0., 0.),
    ));

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        player_control::player_system(&mut world, &player);
        fire_control::fire_control_system(&mut world, &player);

        bullet::bullet_system(&mut world);
        life::life_system(&mut world);
        take_bullet_damage::take_bullet_damage_system(&mut world);
        enemy_fire::enemy_fire_system(&mut world);

        physics::velocity_system(&mut world);
        path_follower::path_follower_system(&mut world);

        rect_renderer::rect_renderer_system(&mut world);
        map_renderer::map_renderer_system(&mut world);
        animated_map_renderer::animated_map_renderer_system(&mut world);

        next_frame().await;
    }

    ExitMode::Quit
}
