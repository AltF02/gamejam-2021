use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct TextChanges;

pub fn init(commands: &mut Commands, asset_server: AssetServer) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn(CameraUiBundle::default());
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "".to_string(),
                font: font.clone(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::RED,
                    alignment: TextAlignment::default(),
                },
            },
            ..Default::default()
        })
        .with(TextChanges);
}

pub fn update_text_diagnostic(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.value = format!("{:.1} fps, {:.3} ms/frame", fps, frame_time * 1000.0).into();
    }
}
