use std::sync::{Arc, Mutex};

use hecs::{EntityBuilder, World};
use macroquad::{prelude::*, rand::gen_range};

use crate::exit_modes::ExitMode;
use crate::systems::{
    animated_map_renderer, bullet, enemy_fire, fire_control, life, map_renderer, path_follower,
    physics, player_control, rect_renderer, take_bullet_damage,
};

pub async fn game() -> ExitMode {
    let mut world = World::new();

    let gameover = Arc::new(Mutex::new(false));
    let scorecounter = Arc::new(Mutex::new(0));

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
        take_bullet_damage::TakeBulletDamage::new(
            String::from("Enemy Bullet"),
            Box::new(move |w, _e| {
                w.despawn(life_display.pop().unwrap()).unwrap();
            }),
        ),
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
    let new_enemy = || {
        let mut types: Vec<Box<dyn FnMut() -> EntityBuilder>> = vec![
            Box::new(|| {
                let mut enemy = EntityBuilder::new();

                let path: Vec<(f32, f32)> = vec![
                    (20., gen_range(0., screen_height() / 3.)),
                    (
                        screen_width() - 35. - 20.,
                        gen_range(0., screen_height() / 3.),
                    ),
                ]
                .into_iter()
                .cycle()
                .take(4)
                .collect();

                enemy
                    // Dynamic modified by Path
                    .add(physics::Position::new(path[0].0, path[0].1))
                    .add(physics::Velocity::new(0., 0.))
                    // Rendering
                    .add(physics::Size::new(35., 35.))
                    .add(animated_map_renderer::AnimatedMapRenderer::new(
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
                    ))
                    // Life & Fire
                    .add({
                        let scorecounter_clone = scorecounter.clone();
                        life::Life::new(
                            1,
                            Box::new(move |_w, _e| {
                                *scorecounter_clone.lock().unwrap() += 1;
                            }),
                        )
                    })
                    .add(take_bullet_damage::TakeBulletDamage::new(
                        String::from("Player Bullet"),
                        Box::new(move |_w, _e| {}),
                    ))
                    .add(enemy_fire::EnemyFire::new(20..100))
                    // Path
                    .add(path_follower::PathFollower::new(40., path));
                enemy
            }),
            Box::new(|| {
                let mut enemy = EntityBuilder::new();

                let radius = 100.;
                let center = (
                    gen_range(0. + radius, screen_width() - 35. - radius),
                    gen_range(0. + radius, screen_height() / 2. - 35. - radius),
                );

                let path = vec![
                    (center.0 - radius, center.1),
                    (center.0, center.1 + radius),
                    (center.0 + radius, center.1),
                    (center.0, center.1 - radius),
                ];

                enemy
                    // Dynamic modified by Path
                    .add(physics::Position::new(path[0].0, path[0].1))
                    .add(physics::Velocity::new(0., 0.))
                    // Rendering
                    .add(physics::Size::new(35., 35.))
                    .add(animated_map_renderer::AnimatedMapRenderer::new(
                        vec![
                            (vec![" ### ", "#@#@#", "#####", "&   &", " & & "].into(), 5),
                            (vec![" ### ", "#@#@#", "#####", "&   &", "&   &"].into(), 5),
                        ],
                        vec![
                            (' ', Color::from_rgba(0, 0, 0, 0)),
                            ('#', Color::from_rgba(169, 77, 238, 255)),
                            ('@', Color::from_rgba(37, 200, 53, 255)),
                            ('&', Color::from_rgba(232, 255, 121, 255)),
                        ]
                        .into(),
                    ))
                    // Life & Fire
                    .add({
                        let scorecounter_clone = scorecounter.clone();
                        life::Life::new(
                            1,
                            Box::new(move |_w, _e| {
                                *scorecounter_clone.lock().unwrap() += 1;
                            }),
                        )
                    })
                    .add(take_bullet_damage::TakeBulletDamage::new(
                        String::from("Player Bullet"),
                        Box::new(move |_w, _e| {}),
                    ))
                    .add(enemy_fire::EnemyFire::new(20..100))
                    // Path
                    .add(path_follower::PathFollower::new(40., path));

                enemy
            }),
        ];

        let selected = gen_range(0, types.len());
        types[selected]()
    };

    let score = loop {
        if *gameover.lock().unwrap() {
            break *scorecounter.lock().unwrap();
        }

        if is_key_pressed(KeyCode::Escape) {
            break *scorecounter.lock().unwrap();
        }

        clear_background(BLACK);

        next_enemy -= 1;
        if next_enemy == 0 {
            next_enemy = gen_range(20u8, 100) as u16;

            world.spawn(new_enemy().build());
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
    };

    loop {
        clear_background(BLACK);

        // Text Display

        let text = format!("Gameover! Your score was : {}", score);
        let measure = measure_text(&text, None, 30, 1.);
        draw_text(
            &text,
            screen_width() / 2. - measure.width / 2.,
            screen_height() / 2. - measure.height * 2.5,
            30.,
            WHITE,
        );

        let text = String::from("Press [SPACE] to restart, [ESCAPE] to quit");
        let measure = measure_text(&text, None, 30, 1.);
        draw_text(
            &text,
            screen_width() / 2. - measure.width / 2.,
            screen_height() / 2. - measure.height / 2.,
            30.,
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
