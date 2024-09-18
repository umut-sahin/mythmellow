//! Camera systems.

use crate::prelude::*;


/// Spawns the main camera.
pub fn spawn_main_camera(mut commands: Commands) {
    log::info!("spawning the main camera");
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        Camera2dBundle::default(),
        IsDefaultUiCamera,
    ));
}


/// Makes the main camera locked to the player.
pub fn player_lock(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>, Without<MainCamera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}


/// Sets the default ui camera based on the state of the editor.
#[cfg(feature = "development")]
pub fn set_default_ui_camera(
    mut commands: Commands,
    main_camera_query: Query<Entity, With<MainCamera>>,
    editor: Res<Editor>,
) {
    let main_camera_entity = match main_camera_query.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    if editor.active() {
        commands.entity(main_camera_entity).remove::<IsDefaultUiCamera>();
    } else {
        commands.entity(main_camera_entity).insert(IsDefaultUiCamera);
    }
}
