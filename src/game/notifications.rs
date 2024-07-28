use bevy::prelude::*;

use super::{assets::SfxKey, audio::sfx::PlaySfx, util::DestroyAfterSecs};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<Notification>();
    app.add_systems(Update, (listen_for_notifications, move_notification_text));
}

#[derive(Debug, Event)]
pub struct Notification(pub String);

#[derive(Component)]
pub struct NotificationText;

fn listen_for_notifications(mut event_reader: EventReader<Notification>, mut commands: Commands) {
    for notification in event_reader.read() {
        commands.trigger(PlaySfx::Key(SfxKey::Ping));
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    notification.0.clone(),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                        ..Default::default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    left: Val::Px(30.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            DestroyAfterSecs::new(6.0),
            NotificationText,
        ));
    }
}

fn move_notification_text(mut query: Query<&mut Style, With<NotificationText>>, time: Res<Time>) {
    for mut style in query.iter_mut() {
        if let Val::Px(px) = style.bottom {
            style.bottom = Val::Px(px + time.delta_seconds() * 50.0);
        }
    }
}
