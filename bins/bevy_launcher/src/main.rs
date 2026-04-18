//! A simple launcher for Bevy applications. 
//! This is intended to be a standalone binary that can be distributed for the purpose of creating/managing Bevy projects.
use bevy::{
    feathers::{
        constants::{fonts, icons},
        containers::{
            flex_spacer, group, group_body, group_header, pane, pane_body, pane_header,
            pane_header_divider, subpane, subpane_body, subpane_header,
        },
        controls::{
            button, checkbox, color_plane, color_slider, color_swatch, disclosure_toggle, menu,
            menu_button, menu_divider, menu_item, menu_popup, radio, slider, text_input,
            text_input_container, toggle_switch, tool_button, ButtonProps, ButtonVariant,
            CheckboxProps, ColorChannel, ColorPlane, ColorPlaneValue, ColorSlider,
            ColorSliderProps, ColorSwatch, ColorSwatchValue, MenuButtonProps, MenuItemProps,
            RadioProps, SliderBaseColor, SliderProps, TextInputProps,
        },
        cursor::{EntityCursor, OverrideCursor},
        dark_theme::create_dark_theme,
        display::{icon, label, label_dim},
        font_styles::InheritableFont,
        rounded_corners::RoundedCorners,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme, ThemeBorderColor},
        tokens, FeathersPlugins,
        
    }, 
    prelude::*, ui_widgets::Activate,};


/// The internal Bevy Launcher plugin.
pub struct LauncherInternalPlugin;

impl Plugin for LauncherInternalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Launcher".to_string(),
                resolution: bevy::window::WindowResolution::new(1000, 600),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::oklch(0.2046, 0.0, 0.0)))
        .add_plugins(FeathersPlugins).insert_resource(UiTheme(create_dark_theme())).add_systems(Startup, setup);
    }
}

fn main() {
    App::new().add_plugins(LauncherInternalPlugin).run();
}

fn setup(world: &mut World) -> Result {
    world.spawn_scene_list(bsn_list![Camera2d, launcher_root()])?;
    println!("Bevy Launcher setup complete.");
    Ok(())
}

fn launcher_root() -> impl Scene {
    bsn!{
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            
        }
        ThemeBackgroundColor(tokens::WINDOW_BG)
        :pane 
            Node {
                padding: UiRect::all(Val::Px(4.0)),
            }
            Children [
                :pane_header Children [
                    button(
                        ButtonProps {
                            caption: Box::new(bsn_list!(
                                (Text("Normal") ThemedText),
                            )),
                            ..default()
                        })
                        Node {
                            flex_grow: 1.0,
                        }
                        on(|_activate: On<Activate>| {
                            info!("Normal button clicked!");
                        }
                    )
                ],
                (
                    :pane_body 
                        Node {
                            height: percent(100),
                            width: percent(100),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            border: UiRect {
                                left: Val::Px(1.0),
                                top: Val::Px(0.0),
                                right: Val::Px(1.0),
                                bottom: Val::Px(1.0),
                            },
                        }
                        ThemeBorderColor(tokens::PANE_HEADER_BORDER)
                        Children [
                        
                        ]
                )
            ]
    }
}
