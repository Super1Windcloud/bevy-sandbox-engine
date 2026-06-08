use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_pane_layout::{PaneLayoutPlugin, RootPaneLayoutNode};
use bevy_properties_pane::PropertiesPanePlugin;
use bevy_scene_tree::SceneTreePlugin;
use bevy_toolbar::{ActiveTool, EditorTool};
use bevy_transform_gizmos::{GizmoMode, TransformGizmoSettings};

/// The Bevy Editor UI Plugin.
pub struct EditorUIPlugin;

impl Plugin for EditorUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_setup.in_set(UISet))
            .add_systems(EguiPrimaryContextPass, render_editor_shell)
            .add_plugins((PaneLayoutPlugin, SceneTreePlugin, PropertiesPanePlugin))
            .init_resource::<EditorShellState>();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UISet;

/// The root node for the UI.
#[derive(Component)]
pub struct RootUINode;

#[derive(Resource, Default)]
struct EditorShellState {
    status: String,
}

fn ui_setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 10,
            ..default()
        },
    ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(56.0),
                bottom: Val::Px(26.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            RootUINode,
        ))
        .with_children(|parent| {
            parent.spawn(RootPaneLayoutNode);
        });
}

fn render_editor_shell(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut active_tool: ResMut<ActiveTool>,
    mut gizmo_settings: ResMut<TransformGizmoSettings>,
    mut shell_state: ResMut<EditorShellState>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::TopBottomPanel::top("editor_top_bar")
        .exact_height(56.0)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(20, 22, 26))
                .inner_margin(egui::Margin::symmetric(14, 10)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                ui.label(
                    egui::RichText::new("Bevy Sandbox Engine")
                        .strong()
                        .color(egui::Color32::from_rgb(230, 230, 232)),
                );
                ui.separator();
                menu_button(
                    ui,
                    "File",
                    &mut shell_state.status,
                    "File actions are not wired yet",
                );
                menu_button(
                    ui,
                    "Edit",
                    &mut shell_state.status,
                    "Edit actions are not wired yet",
                );
                menu_button(
                    ui,
                    "Build",
                    &mut shell_state.status,
                    "Build actions are not wired yet",
                );
                menu_button(
                    ui,
                    "Window",
                    &mut shell_state.status,
                    "Window actions are not wired yet",
                );
                menu_button(
                    ui,
                    "Help",
                    &mut shell_state.status,
                    "Help actions are not wired yet",
                );

                ui.separator();

                tool_button(
                    ui,
                    &mut active_tool,
                    &mut gizmo_settings,
                    EditorTool::Select,
                    "Select",
                );
                tool_button(
                    ui,
                    &mut active_tool,
                    &mut gizmo_settings,
                    EditorTool::Move,
                    "Move",
                );
                tool_button(
                    ui,
                    &mut active_tool,
                    &mut gizmo_settings,
                    EditorTool::Rotate,
                    "Rotate",
                );
                tool_button(
                    ui,
                    &mut active_tool,
                    &mut gizmo_settings,
                    EditorTool::Scale,
                    "Scale",
                );

                if ui.button("New Entity").clicked() {
                    spawn_new_entity(&mut commands);
                    shell_state.status = "Spawned a new entity".to_string();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let snap_label = if gizmo_settings.snap_enabled {
                        "Snap On"
                    } else {
                        "Snap Off"
                    };
                    if ui.button(snap_label).clicked() {
                        gizmo_settings.snap_enabled = !gizmo_settings.snap_enabled;
                        shell_state.status = if gizmo_settings.snap_enabled {
                            "Transform snap enabled".to_string()
                        } else {
                            "Transform snap disabled".to_string()
                        };
                    }

                    ui.label(
                        egui::RichText::new("W Move  E Rotate  R Scale")
                            .color(egui::Color32::from_rgb(145, 149, 157)),
                    );
                });
            });
        });

    egui::TopBottomPanel::bottom("editor_status_bar")
        .exact_height(26.0)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(16, 18, 20))
                .inner_margin(egui::Margin::symmetric(12, 6)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&shell_state.status)
                        .color(egui::Color32::from_rgb(188, 192, 198)),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("Tool: {}", tool_name(active_tool.0)))
                        .color(egui::Color32::from_rgb(148, 152, 158)),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(if gizmo_settings.snap_enabled {
                        "Backend chrome: egui"
                    } else {
                        "Backend chrome: egui"
                    })
                    .color(egui::Color32::from_rgb(148, 152, 158)),
                );
            });
        });

    Ok(())
}

fn menu_button(ui: &mut egui::Ui, label: &str, status: &mut String, message: &str) {
    if ui.button(label).clicked() {
        *status = message.to_string();
    }
}

fn tool_button(
    ui: &mut egui::Ui,
    active_tool: &mut ActiveTool,
    gizmo_settings: &mut TransformGizmoSettings,
    tool: EditorTool,
    label: &str,
) {
    let selected = active_tool.0 == tool;
    if ui.selectable_label(selected, label).clicked() {
        active_tool.0 = tool;
        match tool {
            EditorTool::Move => gizmo_settings.mode = GizmoMode::Translate,
            EditorTool::Rotate => gizmo_settings.mode = GizmoMode::Rotate,
            EditorTool::Scale => gizmo_settings.mode = GizmoMode::Scale,
            _ => {}
        }
    }
}

fn tool_name(tool: EditorTool) -> &'static str {
    match tool {
        EditorTool::Select => "Select",
        EditorTool::Move => "Move",
        EditorTool::Rotate => "Rotate",
        EditorTool::Scale => "Scale",
        EditorTool::NewEntity => "New Entity",
        EditorTool::Save => "Save",
        EditorTool::Load => "Load",
        EditorTool::Undo => "Undo",
        EditorTool::Redo => "Redo",
        EditorTool::Play => "Play",
        EditorTool::Pause => "Pause",
        EditorTool::Stop => "Stop",
    }
}

fn spawn_new_entity(commands: &mut Commands) {
    commands.spawn((
        Name::new("New Entity"),
        Transform::default(),
        Visibility::default(),
    ));
}
