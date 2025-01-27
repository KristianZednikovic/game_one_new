use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(AppState::OptionsMenu), setup_options_menu)
        .add_systems(OnExit(AppState::OptionsMenu), despawn_options_menu)
        .add_systems(Update, menu_system.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
        .add_systems(
            Update,
            options_button_system.run_if(in_state(AppState::OptionsMenu)),
        )
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    OptionsMenu,
}

// Main Menu Components
#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
struct MainMenuText;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct OptionsButton;

#[derive(Component)]
struct ExitButton;

// Options Menu Components
#[derive(Component)]
struct OptionsMenuRoot;

#[derive(Component)]
struct SoundButton;

#[derive(Component)]
struct DifficultyButton;

#[derive(Component)]
struct BackButton;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Game One",
                        TextStyle {
                            font: default(),
                            font_size: 48.0,
                            color: Color::WHITE,
                        },
                    ),
                    style: Style {
                        margin: UiRect::bottom(Val::Px(150.0)),
                        ..default()
                    },
                    ..default()
                },
                MainMenuText,
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    StartButton,
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: default(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    OptionsButton,
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "Options",
                        TextStyle {
                            font: default(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    ExitButton,
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: default(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenuRoot>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

fn setup_options_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OptionsMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Options",
                    TextStyle {
                        font: default(),
                        font_size: 48.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(150.0)),
                    ..default()
                },
                ..default()
            },));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    SoundButton,
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "Sound On",
                        TextStyle {
                            font: default(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    DifficultyButton,
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "Difficulty Easy",
                        TextStyle {
                            font: default(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    BackButton,
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            font: default(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

fn despawn_options_menu(
    mut commands: Commands,
    options_menu_query: Query<Entity, With<OptionsMenuRoot>>,
) {
    if let Ok(options_menu_entity) = options_menu_query.get_single() {
        commands.entity(options_menu_entity).despawn_recursive();
    }
}

fn menu_system(mut app_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame);
        println!("Changing to in-game state");
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&OptionsButton>,
            Option<&ExitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, options_button, exit_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if options_button.is_some() {
                    app_state.set(AppState::OptionsMenu);
                } else if exit_button.is_some() {
                    app_exit_events.send(bevy::app::AppExit);
                }
            }
            Interaction::Hovered => {
                *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = Color::DARK_GRAY.into();
            }
        }
    }
}

fn options_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&BackButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, back_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if back_button.is_some() {
                    app_state.set(AppState::MainMenu);
                }
            }
            Interaction::Hovered => {
                *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = Color::DARK_GRAY.into();
            }
        }
    }
}
