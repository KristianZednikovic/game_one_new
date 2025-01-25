fn main() {
    println!("Hello, world!");
}
use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, menu_system)
        .add_systems(Update, button_system)
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Component)]
struct MainMenuText;

#[derive(Component)]
struct ExitButton;

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((NodeBundle {
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
        },))
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
                        margin: UiRect::bottom(Val::Px(50.0)),
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
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
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

fn menu_system(mut app_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame);
        println!("Changing to in-game state");
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    _button_query: Query<Entity, With<ExitButton>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                app_exit_events.send(bevy::app::AppExit);
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
