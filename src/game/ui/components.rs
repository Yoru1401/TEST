use bevy::color::Color;
use bevy::prelude::*;
use univis_ui::prelude::*;

use crate::game::states::GameState;

#[derive(Component)]
pub struct MenuButton {
    pub variant: MenuButtonVariant,
    pub action: MenuButtonAction,
}

#[derive(Component, Clone)]
pub enum MenuButtonVariant {
    Start,
    Resume,
    Quit,
}

#[derive(Component, Clone)]
pub enum MenuButtonAction {
    Transition(GameState),
    Quit,
}

impl MenuButton {
    pub fn start() -> Self {
        Self {
            variant: MenuButtonVariant::Start,
            action: MenuButtonAction::Transition(crate::game::states::GameState::Playing),
        }
    }

    pub fn resume() -> Self {
        Self {
            variant: MenuButtonVariant::Resume,
            action: MenuButtonAction::Transition(crate::game::states::GameState::Playing),
        }
    }

    pub fn quit() -> Self {
        Self {
            variant: MenuButtonVariant::Quit,
            action: MenuButtonAction::Transition(crate::game::states::GameState::MainMenu),
        }
    }
}

pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl ButtonColors {
    pub fn blue() -> Self {
        Self {
            normal: Color::srgb(0.2, 0.6, 0.9),
            hovered: Color::srgb(0.3, 0.7, 1.0),
            pressed: Color::srgb(0.1, 0.5, 0.8),
        }
    }

    pub fn green() -> Self {
        Self {
            normal: Color::srgb(0.2, 0.8, 0.3),
            hovered: Color::srgb(0.3, 0.9, 0.4),
            pressed: Color::srgb(0.1, 0.6, 0.2),
        }
    }

    pub fn red() -> Self {
        Self {
            normal: Color::srgb(0.8, 0.2, 0.2),
            hovered: Color::srgb(0.9, 0.3, 0.3),
            pressed: Color::srgb(0.6, 0.1, 0.1),
        }
    }
}

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
