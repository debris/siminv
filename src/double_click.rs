use std::time::Duration;

use bevy::prelude::*;

#[derive(EntityEvent)]
pub(crate) struct DoubleClickEvent {
    entity: Entity,
}

#[derive(Component)]
pub(crate) struct DoubleClick {
    duration: Duration,
    timer: Option<Timer>
}

impl Default for DoubleClick {
    fn default() -> Self {
        DoubleClick {
            duration: Duration::from_secs_f64(0.2),
            timer: None,
        }
    }
}

pub(crate) fn on_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<&mut DoubleClick>,
) {
    if let Ok(mut double_click) = query.get_mut(click.entity) {
        match double_click.timer {
            None => {
                double_click.timer = Some(Timer::new(double_click.duration, TimerMode::Once));
            },
            Some(_) => {
                double_click.timer = None;
                // the click may be on different entities, but since it is supposed to
                // be fast, we don't care
                commands.trigger(DoubleClickEvent { entity: click.entity });
            }
        }
    }
}

pub(crate) fn update_time(
    time: ResMut<Time>,
    query: Query<&mut DoubleClick>
) {
    for mut click in query {
        if let Some(timer) = &mut click.timer {
            if timer.is_finished() {
                click.timer = None;
            } else {
                timer.tick(time.delta());
            }
        }
    }
}

