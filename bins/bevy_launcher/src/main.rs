//! A simple launcher for Bevy applications. 
//! This is intended to be a standalone binary that can be distributed for the purpose of creating/managing Bevy projects.

use bevy::{
    feathers::{
        FeathersPlugins, constants::{fonts::{MONO, REGULAR}, icons, size::{MEDIUM_FONT, ROW_HEIGHT, SMALL_FONT}}, containers::{
            flex_spacer, group, group_body, group_header, pane, pane_body, pane_header,
            pane_header_divider, subpane, subpane_body, subpane_header,
        }, controls::{
            ButtonProps, ButtonVariant, CheckboxProps, ColorChannel, ColorPlane, ColorPlaneValue, ColorSlider, ColorSliderProps, ColorSwatch, ColorSwatchValue, MenuButtonProps, MenuItemProps, RadioProps, SliderBaseColor, SliderProps, TextInputProps, button, checkbox, color_plane, color_slider, color_swatch, disclosure_toggle, menu, menu_button, menu_divider, menu_item, menu_popup, radio, slider, text_input, text_input_container, toggle_switch, tool_button
        }, cursor::{EntityCursor, OverrideCursor}, dark_theme::create_dark_theme, display::{icon, label, label_dim}, font_styles::InheritableFont, palette::GRAY_1, rounded_corners::RoundedCorners, theme::{ThemeBackgroundColor, ThemeBorderColor, ThemeFontColor, ThemedText, UiTheme}, tokens
    }, prelude::*, scene::prelude::Scene, ui_widgets::Activate};
mod cargo_env;
use cargo_env::{CargoEnvPlugin, CargoEnv};


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
        .add_plugins((FeathersPlugins, CargoEnvPlugin)).insert_resource(UiTheme(create_dark_theme())).add_systems(Startup, setup);
    }
}

fn main() {
    App::new().add_plugins(LauncherInternalPlugin).run();
}

fn setup(world: &mut World) -> Result {
    world.spawn_scene_list(bsn_list![Camera2d, launcher_root()])?;
    let cargo_env = world.resource::<CargoEnv>();
    println!("Bevy Launcher setup complete. Version: {}", cargo_env.cargo_pkg_version);
    Ok(())
}

fn launcher_root() -> impl Scene {
    bsn!{
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(4.0)),
            row_gap: Val::Px(4.0),
            
        }
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children [
            (:pane 
            Node {
                height: percent(100),
                width: percent(100),
            }
            Children [
                :pane_header 
                Node {
                    padding: UiRect {
                        left: Val::Px(6.0),
                        top: Val::Px(0.0),
                        right: Val::Px(6.0),
                        bottom: Val::Px(0.0),
                    },
                    justify_content: JustifyContent::Start,
                    column_gap: Val::Px(0.0),
                }
                Children [
                   (:fake_pane_tab(ButtonProps {
                    caption: Box::new(bsn_list!(
                        (Text("Projects") ThemedText),
                    )),
                    ..default()
                   })),
                   (:inactive_tab(ButtonProps {
                       caption: Box::new(bsn_list!(
                           (Text("Templates") ThemedText),
                       )),
                       ..default()
                   })),
                   (:inactive_tab(ButtonProps {
                       caption: Box::new(bsn_list!(
                           (Text("Installs") ThemedText),
                       )),
                       ..default()
                   })),
                   (:inactive_tab(ButtonProps {
                       caption: Box::new(bsn_list!(
                           (Text("Assets") ThemedText),
                       )),
                       ..default()
                   })),
                   (:inactive_tab(ButtonProps {
                       caption: Box::new(bsn_list!(
                           (Text("Learn") ThemedText),
                       )),
                       ..default()
                   })),
                   (:inactive_tab(ButtonProps {
                       caption: Box::new(bsn_list!(
                           (Text("Community") ThemedText),
                       )),
                       ..default()
                   })),
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
            ]),
            // Bottom bar under the content pane, Intended to be a thin status/footer bar
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(24.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                }
                InheritableFont {
                    font: REGULAR,
                    font_size: SMALL_FONT,
                    weight: FontWeight::DEFAULT,
                }
                ThemeFontColor(tokens::TEXT_DIM)
                Children [
                    (
                        :external_link_bar
                    ),
                    // Version
                    (
                        :version
                    ),
                    //Settings
                    (
                        Node {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                        }
                        Text("Settings") 
                        ThemedText
                    ),
                ]
            )
        ]
    }
}


fn fake_pane_tab(props: ButtonProps) -> impl Scene {
    bsn!{
        Node { height: Val::Percent(100.0) }
        Children [
            // Spacer node before the button to create inverted border effect
            (Node { 
                width: Val::Px(5.0),
                border: UiRect::all(Val::Px(0.0)), 
                border_radius: BorderRadius::bottom_right(Val::Px(6.0))
            }
            OuterColor(GRAY_1)
            ),
            // The actual button in the tab
            (
                Node {
                    height: Val::Percent(100.0),
                    border: UiRect::top(Val::Px(2.0)),
                    border_radius: BorderRadius::top(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(0.)),
                }
                ThemeFontColor(tokens::BUTTON_TEXT)
                InheritableFont {
                    font: REGULAR,
                    font_size: SMALL_FONT,
                    weight: FontWeight::DEFAULT,
                }
                Children [
                    {props.caption}
                ]
                ThemeBackgroundColor(tokens::PANE_BODY_BG)
                ThemeBorderColor(tokens::SWITCH_BORDER_CHECKED)
            ),
            // Spacer node after the button to create inverted border effect
            (Node { 
                width: Val::Px(5.0),
                border: UiRect::all(Val::Px(0.0)), 
                border_radius: BorderRadius::bottom_left(Val::Px(6.0))
            }
            OuterColor(GRAY_1)
            ),
        ]
    }
}

fn inactive_tab(props: ButtonProps) -> impl Scene {
    bsn!{
        Node { 
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(8.0), Val::Px(0.)),
        }
        ThemeFontColor(tokens::TEXT_DIM)
        InheritableFont {
            font: REGULAR,
            font_size: SMALL_FONT,
            weight: FontWeight::DEFAULT,
        }
        Children [
            {props.caption}
        ]
    }
}

fn external_link_bar() -> impl Scene {
    bsn!{
        Node {
            
        }
        Children [
            :tool_button(ButtonProps{
                variant: ButtonVariant::Plain,
                ..default()
            }) Children [
                (Text("\u{00BD}") ThemedText)
            ],
            :tool_button(ButtonProps{
                variant: ButtonVariant::Plain,
                ..default()
            }) Children [
                (Text("\u{00BD}") ThemedText)
            ],
            :tool_button(ButtonProps{
                variant: ButtonVariant::Plain,
                ..default()
            }) Children [
                (Text("\u{00BD}") ThemedText)
            ],
        ]
    }
}

fn version() -> impl Scene {
    bsn!{
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Text({format!("V.{}", env!("CARGO_PKG_VERSION"))})
        ThemedText
    }
}