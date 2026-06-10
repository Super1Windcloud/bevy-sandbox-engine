use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_pane_layout::{PaneLayoutPlugin, RootPaneLayoutNode, prelude::*};
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
            .register_pane("Console", setup_console_pane)
            .register_pane("Asset Store", setup_asset_store_pane)
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
    play_state: PlayState,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum PlayState {
    #[default]
    Editing,
    Playing,
    Paused,
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
        .exact_height(74.0)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(43, 43, 43))
                .inner_margin(egui::Margin::symmetric(10, 6)),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 14.0;
                    ui.label(
                        egui::RichText::new("Sandmod Engine")
                            .strong()
                            .color(egui::Color32::from_rgb(238, 238, 239)),
                    );
                    menu_button(ui, "文件", &mut shell_state.status, "文件菜单暂未接线");
                    menu_button(ui, "编辑", &mut shell_state.status, "编辑菜单暂未接线");
                    menu_button(ui, "窗口", &mut shell_state.status, "窗口菜单暂未接线");
                    menu_button(ui, "游戏对象", &mut shell_state.status, "对象菜单暂未接线");
                    menu_button(ui, "组件", &mut shell_state.status, "组件菜单暂未接线");
                    menu_button(ui, "帮助", &mut shell_state.status, "帮助菜单暂未接线");
                });

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;

                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Select,
                        "选择",
                    );
                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Move,
                        "移动",
                    );
                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Rotate,
                        "旋转",
                    );
                    tool_button(
                        ui,
                        &mut active_tool,
                        &mut gizmo_settings,
                        EditorTool::Scale,
                        "缩放",
                    );

                    ui.separator();

                    play_button(ui, &mut shell_state, PlayState::Playing, "▶");
                    play_button(ui, &mut shell_state, PlayState::Paused, "⏸");
                    if ui.button("⏹").clicked() {
                        shell_state.play_state = PlayState::Editing;
                        shell_state.status = "已停止运行".to_string();
                    }

                    ui.separator();

                    if ui.button("新建对象").clicked() {
                        spawn_new_entity(&mut commands);
                        shell_state.status = "已创建新对象".to_string();
                    }

                    let snap_label = if gizmo_settings.snap_enabled {
                        "捕捉: 开"
                    } else {
                        "捕捉: 关"
                    };
                    if ui.button(snap_label).clicked() {
                        gizmo_settings.snap_enabled = !gizmo_settings.snap_enabled;
                        shell_state.status = if gizmo_settings.snap_enabled {
                            "已启用变换捕捉".to_string()
                        } else {
                            "已关闭变换捕捉".to_string()
                        };
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("坐标 5.00    缩放 1.0    旋转 0.01")
                                .color(egui::Color32::from_rgb(170, 170, 173)),
                        );
                    });
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
                    egui::RichText::new(play_state_name(shell_state.play_state))
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

fn play_button(
    ui: &mut egui::Ui,
    shell_state: &mut EditorShellState,
    target: PlayState,
    label: &str,
) {
    let selected = shell_state.play_state == target;
    let button = egui::Button::new(label).fill(if selected {
        egui::Color32::from_rgb(88, 132, 55)
    } else {
        egui::Color32::from_rgb(62, 62, 64)
    });

    if ui.add(button).clicked() {
        shell_state.play_state = target;
        shell_state.status = match target {
            PlayState::Editing => "编辑模式".to_string(),
            PlayState::Playing => "运行中".to_string(),
            PlayState::Paused => "已暂停".to_string(),
        };
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

fn play_state_name(state: PlayState) -> &'static str {
    match state {
        PlayState::Editing => "State: Editing",
        PlayState::Playing => "State: Playing",
        PlayState::Paused => "State: Paused",
    }
}

fn spawn_new_entity(commands: &mut Commands) {
    commands.spawn((
        Name::new("New Entity"),
        Transform::default(),
        Visibility::default(),
    ));
}

fn setup_console_pane(pane: In<PaneStructure>, mut commands: Commands) {
    commands.entity(pane.header).with_children(|parent| {
        parent.spawn((Text::new("控制台"), TextFont::from_font_size(12.0)));
    });

    commands.entity(pane.content).insert((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(6.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.12, 0.12, 0.13)),
    ));

    commands.entity(pane.content).with_children(|parent| {
        for line in [
            "[Info] Compatibility project loaded",
            "[Info] DefaultScene migrated into placeholder entities",
            "[Warn] Script bridge is running with stub engine APIs",
            "[Info] Press Play to step toward runtime parity",
        ] {
            parent.spawn((
                Text::new(line),
                TextFont::from_font_size(11.0),
                TextColor(Color::srgb(0.76, 0.77, 0.79)),
            ));
        }
    });
}

fn setup_asset_store_pane(pane: In<PaneStructure>, mut commands: Commands) {
    commands.entity(pane.header).with_children(|parent| {
        parent.spawn((Text::new("资源商店"), TextFont::from_font_size(12.0)));
    });

    commands.entity(pane.content).insert((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.16)),
    ));

    commands.entity(pane.content).with_children(|parent| {
        parent.spawn((
            Text::new("分类"),
            TextFont::from_font_size(12.0),
            TextColor(Color::srgb(0.86, 0.86, 0.88)),
        ));

        for category in ["推荐", "几何体", "生物", "自然", "人造物"] {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.19, 0.19, 0.20)),
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(category),
                        TextFont::from_font_size(11.0),
                        TextColor(Color::srgb(0.74, 0.75, 0.77)),
                    ));
                });
        }
    });
}
