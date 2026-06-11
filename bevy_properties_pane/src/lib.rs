//! An interactive, reflection-based inspector for Bevy ECS data in running applications.

use bevy::{prelude::*, reflect::*};
use bevy_editor_core::selection::{EditorSelection, common_conditions::primary_selection_changed};
use bevy_editor_styles::{EditorLocale, Theme};
use bevy_i_cant_believe_its_not_bsn::{Template, TemplateEntityCommandsExt, template};
use bevy_pane_layout::prelude::*;

/// Plugin for the editor properties pane.
pub struct PropertiesPanePlugin;

impl Plugin for PropertiesPanePlugin {
    fn build(&self, app: &mut App) {
        app.register_pane("Inspector", setup_pane).add_systems(
            Update,
            (update_properties_pane.run_if(
                primary_selection_changed.or(any_match_filter::<Added<PropertiesPaneBody>>),
            ),),
        );
    }
}

/// Root UI node of the properties pane.
#[derive(Component, Default, Clone)]
struct PropertiesPaneBody;

fn setup_pane(pane: In<PaneStructure>, mut commands: Commands) {
    commands.entity(pane.content).insert((
        PropertiesPaneBody,
        Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.0,
            overflow: Overflow::clip(),
            ..default()
        },
    ));
}

fn update_properties_pane(
    pane_bodies: Query<Entity, With<PropertiesPaneBody>>,
    selection: Res<EditorSelection>,
    theme: Res<Theme>,
    world: &World,
    mut commands: Commands,
) {
    for pane_body in &pane_bodies {
        commands
            .entity(pane_body)
            .build_children(properties_pane(&selection, &theme, world));
    }
}

fn properties_pane(selection: &EditorSelection, theme: &Theme, world: &World) -> Template {
    let empty_text = match EditorLocale::detect() {
        EditorLocale::ZhCn => "选择一个对象以查看属性",
        EditorLocale::EnUs => "Select an entity to inspect",
    };

    match selection.primary() {
        Some(selection) => template! {
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                }
            ) => [
                @{ summary_panel(selection, theme, world) };
                @{ component_list(selection, theme, world) };
            ];
        },
        None => template! {
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(24.0)),
                    height: Val::Percent(100.0),
                    ..default()
                }
            ) => [
                (
                    Text::new(empty_text),
                    TextFont {
                        font: theme.text.font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.514, 0.514, 0.522)),
                );
            ];
        },
    }
}

fn summary_panel(entity: Entity, theme: &Theme, world: &World) -> Template {
    let locale = EditorLocale::detect();
    let title = world
        .get::<Name>(entity)
        .map(|name| name.as_str().to_string())
        .unwrap_or_else(|| match locale {
            EditorLocale::ZhCn => "未命名对象".to_string(),
            EditorLocale::EnUs => "Unnamed Entity".to_string(),
        });
    let entity_label = match locale {
        EditorLocale::ZhCn => "对象",
        EditorLocale::EnUs => "Entity",
    };
    let component_label = match locale {
        EditorLocale::ZhCn => "组件数",
        EditorLocale::EnUs => "Components",
    };
    let component_count = world
        .inspect_entity(entity)
        .map(|iter| iter.count())
        .unwrap_or(0);

    template! {
        (
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(8.0)),
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.19)),
            BorderColor::all(Color::srgb(0.25, 0.25, 0.26)),
        ) => [
            (
                Text::new(title),
                TextFont {
                    font: theme.text.font.clone(),
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.90, 0.90, 0.92)),
            );
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                }
            ) => [
                (
                    Text::new(format!("{entity_label} {}", entity.index())),
                    TextFont {
                        font: theme.text.font.clone(),
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.70, 0.71, 0.73)),
                );
                (
                    Text::new(format!("{component_label}: {component_count}")),
                    TextFont {
                        font: theme.text.font.clone(),
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.70, 0.71, 0.73)),
                );
            ];
        ];
    }
}

fn component_list(entity: Entity, theme: &Theme, world: &World) -> Template {
    let type_registry = world.resource::<AppTypeRegistry>().read();
    world
        .inspect_entity(entity)
        .unwrap()
        .flat_map(|component_info| {
            let type_info = component_info
                .type_id()
                .and_then(|type_id| type_registry.get_type_info(type_id));
            let name = type_info.map_or_else(
                || "<unknown>".to_string(),
                |type_info| type_info.type_path_table().short_path().to_string(),
            );

            let reflect: Option<&dyn Reflect> = component_info.type_id().and_then(|type_id| {
                let registration = type_registry.get(type_id)?;
                let reflect_component = registration.data::<ReflectComponent>()?;
                let entity_ref = world.get_entity(entity);
                reflect_component.reflect(entity_ref.unwrap())
            });

            template! {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::bottom(Val::Px(6.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        padding: UiRect::all(Val::Px(0.0)),
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.165, 0.165, 0.180)),
                    BorderColor::all(Color::srgb(0.255, 0.255, 0.259)),
                ) => [
                    (
                        Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            height: Val::Px(26.0),
                            border_radius: BorderRadius::top(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.212, 0.216, 0.231)),
                    ) => [
                        (
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(5.0),
                                ..default()
                            }
                        ) => [
                            (
                                Text::new("▼"),
                                TextFont {
                                    font: theme.text.font.clone(),
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.769, 0.769, 0.769)),
                            );
                            (
                                Text::new(name.clone()),
                                TextFont {
                                    font: theme.text.font.clone(),
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.863, 0.863, 0.863)),
                            );
                        ];
                        (
                            Text::new("⋯"),
                            TextFont {
                                font: theme.text.font.clone(),
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.769, 0.769, 0.769)),
                        );
                    ];
                    @{ match reflect {
                        Some(reflect) => component(type_info, reflect, theme),
                        None => template! {
                            (
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    padding: UiRect::all(Val::Px(8.0)),
                                    ..default()
                                }
                            ) => [
                                (
                                    Text::new(match EditorLocale::detect() {
                                        EditorLocale::ZhCn => "<无反射信息>",
                                        EditorLocale::EnUs => "<reflection unavailable>",
                                    }),
                                    TextFont {
                                        font: theme.text.font.clone(),
                                        font_size: 11.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.514, 0.514, 0.522)),
                                );
                            ];
                        },
                    }};
                ];
            }
        })
        .collect()
}

fn component(type_info: Option<&TypeInfo>, reflect: &dyn Reflect, theme: &Theme) -> Template {
    match type_info {
        Some(TypeInfo::Struct(info)) => reflected_struct(info, reflect, theme),
        Some(TypeInfo::TupleStruct(info)) => reflected_tuple_struct(info, theme),
        Some(TypeInfo::Enum(info)) => reflected_enum(info, theme),
        _ => Template::default(),
    }
}

fn reflected_struct(struct_info: &StructInfo, reflect: &dyn Reflect, theme: &Theme) -> Template {
    let fields: Template = struct_info
        .iter()
        .enumerate()
        .flat_map(|(i, field)| {
            let field_reflect = reflect
                .reflect_ref()
                .as_struct()
                .ok()
                .and_then(|s| s.field_at(i));

            let value_string = field_reflect
                .map(|v| format!("{v:?}"))
                .unwrap_or_else(|| "<unavailable>".to_string());

            template! {
                (
                    Node {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::vertical(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        min_height: Val::Px(22.0),
                        border_radius: BorderRadius::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.212, 0.216, 0.231)),
                ) => [
                    (
                        Text::new(field.name()),
                        TextFont {
                            font: theme.text.font.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.855, 0.855, 0.855)),
                    );
                    (
                        Text::new(value_string.clone()),
                        TextFont {
                            font: theme.text.font.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.761, 0.761, 0.761)),
                    );
                ];
            }
        })
        .collect();

    template! {
        (
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(7.0)),
                row_gap: Val::Px(4.0),
                ..default()
            }
        ) => [
            @{ fields };
        ];
    }
}

fn reflected_tuple_struct(tuple_struct_info: &TupleStructInfo, theme: &Theme) -> Template {
    let todo_label = match EditorLocale::detect() {
        EditorLocale::ZhCn => "待实现",
        EditorLocale::EnUs => "TODO",
    };

    tuple_struct_info
        .iter()
        .flat_map(|_field| {
            template! {
                (
                    Text::new(todo_label),
                    TextFont {
                        font: theme.text.font.clone(),
                        font_size: 10.0,
                        ..default()
                    },
                );
            }
        })
        .collect()
}

fn reflected_enum(enum_info: &EnumInfo, theme: &Theme) -> Template {
    let variants: Template = enum_info
        .iter()
        .flat_map(|variant| {
            template! {
                (
                    Text::new(variant.name()),
                    TextFont {
                        font: theme.text.font.clone(),
                        font_size: 10.0,
                        ..default()
                    },
                );
            }
        })
        .collect();

    template! {
        (
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            }
        ) => [
            @{ variants };
        ];
    }
}
