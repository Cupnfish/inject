use std::time::Duration;

use bevy::{
    math::{Rect, Size, Vec3, Vec4},
    prelude::{
        App, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Entity,
        Plugin, Query, Res, TextBundle, Transform, UiCameraBundle, With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, Interaction, JustifyContent, Style, UiColor, Val},
};
use bevy_tweening::{
    lens::TransformScaleLens, Animator, EaseFunction, EaseMethod, Lens, Tween, TweeningType,
};

pub struct StartPagePlugin;

impl Plugin for StartPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(button_system);
    }
}

const NORMAL_BUTTON: Color = Color::SEA_GREEN;
const HOVERED_BUTTON: Color = Color::DARK_GREEN;
const PRESSED_BUTTON: Color = Color::MIDNIGHT_BLUE;

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (button, interaction) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let tween = Tween::new(
                    EaseFunction::BackInOut,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.2),
                    TransformScaleLens {
                        start: Vec3::new(0.8, 0.8, 0.),
                        end: Vec3::new(0.5, 0.5, 0.),
                    },
                );
                let color = Tween::new(
                    EaseFunction::SineIn,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.2),
                    UiColorColorLens {
                        start: HOVERED_BUTTON,
                        end: PRESSED_BUTTON,
                    },
                );
                commands
                    .entity(button)
                    .insert(Animator::new(tween))
                    .insert(Animator::new(color));
            }
            Interaction::Hovered => {
                let tween = Tween::new(
                    EaseFunction::BackInOut,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.4),
                    TransformScaleLens {
                        start: Vec3::new(1., 1., 0.),
                        end: Vec3::new(0.8, 0.8, 0.),
                    },
                );
                let color = Tween::new(
                    EaseMethod::Linear,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.4),
                    UiColorColorLens {
                        start: NORMAL_BUTTON,
                        end: HOVERED_BUTTON,
                    },
                );
                commands
                    .entity(button)
                    .insert(Animator::new(tween))
                    .insert(Animator::new(color));
            }
            Interaction::None => {
                commands.entity(button).remove::<Animator<Transform>>();
                let tween = Tween::new(
                    EaseMethod::Linear,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.3),
                    TransformScaleLens {
                        start: Vec3::new(0.8, 0.8, 0.),
                        end: Vec3::new(1., 1., 0.),
                    },
                );
                let color = Tween::new(
                    EaseMethod::Linear,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.3),
                    UiColorColorLens {
                        start: HOVERED_BUTTON,
                        end: NORMAL_BUTTON,
                    },
                );
                commands
                    .entity(button)
                    .insert(Animator::new(tween))
                    .insert(Animator::new(color));
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiColorColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

impl Lens<UiColor> for UiColorColorLens {
    fn lerp(&mut self, target: &mut UiColor, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();

        let value = start.lerp(end, ratio);

        target.0 = value.into();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Start",
                    TextStyle {
                        font: asset_server.load("fonts/rock-salt-regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
