//! Widget components.

use {
    crate::all::*,
    mythmallow_core_constants::ui::widget::*,
    mythmallow_core_dependencies::*,
};


/// Component for the menu background.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct MenuBackground;


/// Component for the widgets.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct Widget {
    /// Whether the widget is selected.
    pub is_selected: bool,
    /// Whether the widget is hovered.
    pub is_hovered: bool,
    /// Whether the widget is pressed.
    pub is_pressed: bool,
    /// Whether the widget is clicked.
    pub is_clicked: bool,
}

impl Widget {
    /// Performs an action if the button is just clicked.
    ///
    /// It clears the clicked status of the widget.
    /// So if it is called multiple times in a frame,
    /// only the action in the first call will be performed.
    pub fn on_click(&mut self, action: impl FnOnce()) {
        if self.is_clicked {
            self.is_clicked = false;
            action()
        }
    }
}


/// Component for the selected widget.
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct WidgetSelected {
    /// Instant at which the widget was selected.
    pub at: Instant,
}

impl WidgetSelected {
    /// Creates a new is widget selected at the current instant.
    pub fn now() -> WidgetSelected {
        WidgetSelected { at: Instant::now() }
    }
}


/// Component for the scaled font size of the texts.
#[derive(Clone, Component, Debug, Reflect)]
pub struct ScaledFontSize {
    /// Base font size before scaling.
    pub base: f32,
}


/// Component for the colors of the buttons.
#[derive(Clone, Component, Debug, Reflect)]
pub struct ButtonColors {
    /// Background color of the button when it's normal.
    pub normal_background_color: Color,

    /// Background color of the button when it's selected.
    pub selected_background_color: Color,

    /// Background color of the button when it's pressed.
    pub pressed_background_color: Color,
}

impl Default for ButtonColors {
    fn default() -> ButtonColors {
        ButtonColors {
            normal_background_color: NORMAL_BUTTON_COLOR,
            selected_background_color: SELECTED_BUTTON_COLOR,
            pressed_background_color: PRESSED_BUTTON_COLOR,
        }
    }
}


/// User interface builder extension.
pub trait UiExtension {
    /// Builds a menu title.
    fn menu_title(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity>;

    /// Builds a menu button.
    fn menu_button(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity>;

    /// Builds a change setting button.
    fn change_setting_button(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity>;

    /// Builds a setting name label.
    fn setting_name(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity>;

    /// Builds a setting value label.
    fn setting_value(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity>;
}

impl UiExtension for UiBuilder<'_, Entity> {
    fn menu_title(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity> {
        self.spawn((
            Name::new("Title"),
            TextBundle {
                style: Style {
                    width: MENU_TITLE_WIDTH,
                    height: MENU_TITLE_HEIGHT,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                text: Text::from_section(
                    text.get(localization),
                    TextStyle { color: MENU_TITLE_COLOR, font, ..default() },
                )
                .with_justify(JustifyText::Center),
                ..default()
            },
            text,
            ScaledFontSize { base: MENU_TITLE_FONT_SIZE },
        ))
    }

    fn menu_button(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity> {
        self.container(
            (
                Widget::default(),
                ButtonBundle {
                    style: Style {
                        width: MENU_BUTTON_WIDTH,
                        height: MENU_BUTTON_HEIGHT,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                ButtonColors::default(),
            ),
            |button| {
                button.spawn((
                    Name::new("Label"),
                    TextBundle {
                        text: Text::from_section(
                            text.get(localization),
                            TextStyle { color: NORMAL_BUTTON_LABEL_COLOR, font, ..default() },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    },
                    text,
                    ScaledFontSize { base: MENU_BUTTON_LABEL_FONT_SIZE },
                ));
            },
        )
    }

    fn change_setting_button(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity> {
        self.container(
            (
                Widget::default(),
                ButtonBundle {
                    style: Style {
                        width: CHANGE_SETTING_BUTTON_WIDTH,
                        height: CHANGE_SETTING_BUTTON_HEIGHT,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                ButtonColors::default(),
            ),
            |button| {
                button.style().margin(UiRect::horizontal(Val::Percent(2.50)));
                button.spawn((
                    Name::new("Label"),
                    TextBundle {
                        text: Text::from_section(
                            text.get(localization),
                            TextStyle { color: NORMAL_BUTTON_LABEL_COLOR, font, ..default() },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    },
                    text,
                    ScaledFontSize { base: CHANGE_SETTING_BUTTON_LABEL_FONT_SIZE },
                ));
            },
        )
    }

    fn setting_name(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity> {
        self.spawn((
            Name::new("Name"),
            TextBundle {
                text: Text::from_section(
                    text.get(localization),
                    TextStyle { color: SETTING_NAME_LABEL_COLOR, font, ..default() },
                )
                .with_justify(JustifyText::Center),
                ..default()
            },
            text,
            ScaledFontSize { base: SETTING_NAME_LABEL_FONT_SIZE },
        ))
    }

    fn setting_value(
        &mut self,
        text: LocalizedText,
        localization: &Localization,
        font: Handle<Font>,
    ) -> UiBuilder<Entity> {
        self.spawn((
            Name::new("Value"),
            TextBundle {
                text: Text::from_section(
                    text.get(localization),
                    TextStyle { color: SETTING_VALUE_LABEL_COLOR, font, ..default() },
                )
                .with_justify(JustifyText::Center),
                ..default()
            },
            text,
            ScaledFontSize { base: SETTING_VALUE_LABEL_FONT_SIZE },
        ))
    }
}
