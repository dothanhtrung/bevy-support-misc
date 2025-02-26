use crate::ui::button::ButtonColorEffect;
use bevy::prelude::{
    default, AlignItems, BuildChildren, Button, ChildBuild, Click, Color, Commands, Component, Entity, JustifyContent,
    JustifyItems, Node, Pointer, Query, Text, TextColor, TextFont, Trigger, UiRect, Val,
};
use bevy::ui::{AlignContent, BackgroundColor, FlexDirection};
use bevy_text_edit::{TextEditable, TextEdited};
use std::cmp::{max, min};

#[derive(Component)]
struct NumberInput {
    max: i32,
    min: i32,
}

#[derive(Component)]
#[require(Button)]
struct NumberButton(Entity);

#[derive(Default)]
pub struct NumberInputSetting {
    pub min: i32,
    pub max: i32,
    pub text_bg: Color,
    pub btn_bg: Color,
    pub text_font: TextFont,
    pub text_color: Color,
    pub width: Val,
    pub height: Val,
}

pub fn spawn_number_input_text(commands: &mut Commands, number: i32, setting: NumberInputSetting) {
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            justify_content: JustifyContent::Center,
            width: setting.width,
            height: setting.height,
            ..default()
        })
        .with_children(|builder| {
            let id = builder
                .spawn((
                    Node {
                        width: Val::Percent(80.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::End,
                        align_content: AlignContent::Center,
                        margin: UiRect::right(Val::Px(5.)),
                        ..default()
                    },
                    TextEditable {
                        filter_in: vec!["[0-9.-]".to_string()],
                        ..default()
                    },
                    Text::new(number.to_string()),
                    TextColor::from(setting.text_color),
                    setting.text_font,
                    BackgroundColor::from(setting.text_bg),
                    NumberInput {
                        max: setting.max,
                        min: setting.min,
                    },
                ))
                .observe(change_value)
                .id();

            builder
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    height: Val::Percent(100.),
                    aspect_ratio: Some(1.0),
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn((
                            ButtonColorEffect::default(),
                            NumberButton(id),
                            BackgroundColor::from(setting.btn_bg),
                            Node {
                                height: Val::Percent(48.),
                                width: Val::Percent(100.),
                                margin: UiRect::bottom(Val::Percent(4.)),
                                align_items: AlignItems::Center,
                                align_content: AlignContent::Center,
                                ..default()
                            },
                        ))
                        .observe(increase);
                    builder
                        .spawn((
                            ButtonColorEffect::default(),
                            NumberButton(id),
                            BackgroundColor::from(setting.btn_bg),
                            Node {
                                height: Val::Percent(48.),
                                width: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                align_content: AlignContent::Center,
                                ..default()
                            },
                        ))
                        .observe(reduce);
                });
        });
}

fn change_value(trigger: Trigger<TextEdited>, mut query: Query<(&mut Text, &NumberInput)>) {
    let e = trigger.entity();
    let editted_text = trigger.text.clone();
    if let Ok((mut text, setting)) = query.get_mut(e) {
        if let Ok(num) = editted_text.parse::<i32>() {
            **text = max(min(setting.max, num), setting.min).to_string();
        }
    }
}

fn increase(
    trigger: Trigger<Pointer<Click>>,
    mut text_query: Query<(&mut Text, &NumberInput)>,
    button_query: Query<&NumberButton>,
) {
    if let Ok(NumberButton(e)) = button_query.get(trigger.entity()) {
        if let Ok((mut text, setting)) = text_query.get_mut(*e) {
            if let Ok(num) = text.parse::<i32>() {
                **text = min(setting.max, num + 1).to_string();
            }
        }
    }
}

fn reduce(
    trigger: Trigger<Pointer<Click>>,
    mut text_query: Query<(&mut Text, &NumberInput)>,
    button_query: Query<&NumberButton>,
) {
    if let Ok(NumberButton(e)) = button_query.get(trigger.entity()) {
        if let Ok((mut text, setting)) = text_query.get_mut(*e) {
            if let Ok(num) = text.parse::<i32>() {
                **text = max(setting.min, num - 1).to_string();
            }
        }
    }
}
