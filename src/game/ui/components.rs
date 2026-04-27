use bevy::color::Color;
use bevy::prelude::*;
use univis_ui::prelude::*;

#[derive(Bundle)]
pub struct ScreenRootBundle {
    pub(crate) root: UScreenRoot,
    pub(crate) node: UNode,
    pub(crate) layout: ULayout,
}

impl Default for ScreenRootBundle {
    fn default() -> Self {
        Self {
            root: UScreenRoot,
            node: UNode {
                width: UVal::Percent(100.0),
                height: UVal::Percent(100.0),
                background_color: Color::srgb(0.1, 0.1, 0.15),
                ..default()
            },
            layout: ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ButtonBundle {
    node: UNode,
    layout: ULayout,
    interaction: UInteraction,
    interaction_colors: UInteractionColors,
}

impl ButtonBundle {
    pub fn new(normal: Color, hovered: Color, pressed: Color) -> Self {
        Self {
            node: UNode {
                width: UVal::Px(200.0),
                height: UVal::Px(50.0),
                background_color: normal,
                border_radius: UCornerRadius::all(8.0),
                ..default()
            },
            layout: ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
            interaction: UInteraction::default(),
            interaction_colors: UInteractionColors {
                normal,
                hovered,
                pressed,
            },
        }
    }
}
