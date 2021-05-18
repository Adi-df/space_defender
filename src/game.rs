use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

use hecs::World;
use macroquad::{prelude::*, rand::gen_range};

use crate::exit_modes::ExitMode;
use crate::systems::{
    animated_map_renderer, bullet, enemy_fire, fire_control, life, map_renderer, path_follower,
    physics, player_control, rect_renderer, take_bullet_damage,
};

pub async fn game() -> ExitMode {
    let mut world = World::new();

    let gameover = Arc::new(Mutex::new(false));

    let mut life_display = {
        let mut display = Vec::new();
        display.push(world.spawn((
            map_renderer::MapRenderer::new(
                vec!["  &  ", " ### ", "#####"].into(),
                vec![(' ', Color::from_rgba(0, 0, 0, 0)), ('#', RED), ('&', BLUE)].into(),
            ),
            physics::Position::new(10., 10.),
            physics::Size::new(20., 20.),
        )));
        display.push(world.spawn((
            map_renderer::MapRenderer::new(
                vec!["  &  ", " ### ", "#####"].into(),
                vec![(' ', Color::from_rgba(0, 0, 0, 0)), ('#', RED), ('&', BLUE)].into(),
            ),
            physics::Position::new(40., 10.),
            physics::Size::new(20., 20.),
        )));
        display.push(world.spawn((
            map_renderer::MapRenderer::new(
                vec!["  &  ", " ### ", "#####"].into(),
                vec![(' ', Color::from_rgba(0, 0, 0, 0)), ('#', RED), ('&', BLUE)].into(),
            ),
            physics::Position::new(70., 10.),
            physics::Size::new(20., 20.),
        )));

        display
    };

    let player = world.spawn((
        player_control::PlayerControl::new(10.),
        fire_control::FireControl::new(30),
        map_renderer::MapRenderer::new(
            vec!["  &  ", " ### ", "#####"].into(),
            vec![(' ', Color::from_rgba(0, 0, 0, 0)), ('#', RED), ('&', BLUE)].into(),
        ),
        take_bullet_damage::TakeBulletDamage::new(Box::new(move |w, _e| {
            w.despawn(life_display.pop().unwrap()).unwrap();
        })),
        life::Life::new(3, {
            let gameover_clone = gameover.clone();
            Box::new(move |_w, _e| {
                *gameover_clone.lock().unwrap() = true;
            })
        }),
        physics::Position::new(screen_width() / 2. - 15., screen_height() - 50.),
        physics::Size::new(30., 30.),
        physics::Velocity::new(0., 0.),
    ));

    let mut next_enemy: u16 = 30;
    let new_enemy = |path_lenght: Range<u8>| {
        let mut base_ennemy = (
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
            life::Life::new(
                1,
                Box::new(move |_w, _e| {
                    println!("Dead");
                }),
            ),
            take_bullet_damage::TakeBulletDamage::new(Box::new(move |_w, _e| {})),
            enemy_fire::EnemyFire::new(50..100),
            physics::Size::new(35., 35.),
            //Fixed
            physics::Velocity::new(0., 0.),
            // Dynamic
            physics::Position::new(0., 0.),
            path_follower::PathFollower::new(1., vec![]),
        );

        base_ennemy.6 = physics::Position::new(
            gen_range(0., screen_width() - base_ennemy.4 .0),
            gen_range(0., screen_height() / 3.),
        );
        base_ennemy.7 = path_follower::PathFollower::new(
            gen_range(15., 30.),
            (0..gen_range(path_lenght.start, path_lenght.end))
                .into_iter()
                .map(|_| {
                    (
                        gen_range(0., screen_width() - base_ennemy.4 .0),
                        gen_range(0., screen_height() / 3. - base_ennemy.4 .1),
                    )
                })
                .collect(),
        );

        base_ennemy
    };

    loop {
        if *gameover.lock().unwrap() {
            break;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(BLACK);

        next_enemy -= 1;
        if next_enemy == 0 {
            next_enemy = gen_range(20u8, 100) as u16;

            world.spawn(new_enemy(4..7));
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

    loop {
        clear_background(BLACK);

        // Text
        let text = String::from("Press [SPACE] to restart, [ESCAPE] to quit");
        let measure = measure_text(&text, None, 30, 1.);
        draw_text(
            &text,
            screen_width() / 2. - measure.width / 2.,
            screen_height() / 2. - measure.height / 2.,
            30.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Space) {
            break ExitMode::NewGame;
        } else if is_key_pressed(KeyCode::Escape) {
            break ExitMode::Quit;
        }

        next_frame().await
    }
}
