//! Configuration resources.

use {
    crate::all::*,
    mythmallow_core_components::all::*,
    mythmallow_core_constants::configuration::*,
    mythmallow_core_dependencies::*,
};

/// Resource for the arguments of the application.
#[derive(Debug, Reflect, Resource)]
pub struct Arguments {
    /// Base directory for configuration files.
    pub configuration_directory: PathBuf,

    /// Base directory for data files.
    pub data_directory: PathBuf,

    /// Seed for random number generation.
    pub seed: Option<u64>,

    /// Whether to start directly in game.
    pub start_in_game: bool,

    /// Game mode to select when starting directly in game.
    pub start_in_game_mode: Option<String>,

    /// Player to select when starting directly in game.
    pub start_in_game_player: Option<String>,

    /// Enemies to select when starting directly in game.
    pub start_in_game_enemies: Option<String>,

    /// Level of the player to set when starting directly in game.
    pub start_in_game_level: Option<NonZeroU16>,

    /// Experience of the player to set when starting directly in game.
    pub start_in_game_experience: Option<f64>,
}

impl Arguments {
    /// Initializes the arguments from the environment.
    ///
    /// # Native
    ///
    /// Arguments are parsed from the command line.
    ///
    /// ```shell
    /// mythmallow --seed 42 --game
    /// ```
    ///
    /// # Web
    ///
    /// Arguments are parsed from the URL.
    ///
    /// ```txt
    /// https://mythmallow.io/?seed=42&game
    /// ```
    pub fn initialize(app: &mut App) {
        #[derive(Parser)]
        #[command(about, version)]
        #[clap(name = "mythmallow")]
        struct ArgumentParser {
            /// Set base directory for configuration files.
            #[arg(long)]
            pub configuration: Option<PathBuf>,

            /// Set base directory for data files.
            #[arg(long)]
            pub data: Option<PathBuf>,

            /// Sets the seed for random number generation.
            #[arg(long)]
            pub seed: Option<u64>,

            /// Starts the application directly in game.
            #[arg(long)]
            pub game: bool,

            /// Specifies the game mode to select when starting directly in game.
            #[arg(long)]
            pub mode: Option<String>,

            /// Specifies the player to select when starting directly in game.
            #[arg(long)]
            pub player: Option<String>,

            /// Specifies the enemies to select when starting directly in game.
            #[arg(long)]
            pub enemies: Option<String>,

            /// Specifies the level of the player to set when starting directly in game.
            #[arg(long)]
            pub level: Option<NonZeroU16>,

            /// Specifies the experience of the player to set when starting directly in game.
            #[arg(long)]
            pub experience: Option<f64>,
        }

        impl Default for ArgumentParser {
            fn default() -> ArgumentParser {
                ArgumentParser {
                    configuration: None,
                    data: None,
                    seed: None,
                    game: false,
                    mode: None,
                    player: None,
                    enemies: None,
                    level: None,
                    experience: None,
                }
            }
        }

        impl Display for ArgumentParser {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(configuration) = &self.configuration {
                    write!(f, " --configuration \"{}\"", configuration.display())?;
                }
                if let Some(data) = &self.data {
                    write!(f, " --data \"{}\"", data.display())?;
                }
                if let Some(seed) = &self.seed {
                    write!(f, " --seed {}", seed)?;
                }
                if self.game {
                    write!(f, " --game")?;
                }
                if let Some(mode) = &self.mode {
                    write!(f, " --mode \"{}\"", mode)?;
                }
                if let Some(player) = &self.player {
                    write!(f, " --player \"{}\"", player)?;
                }
                if let Some(enemies) = &self.enemies {
                    write!(f, " --enemies \"{}\"", enemies)?;
                }
                if let Some(level) = &self.level {
                    write!(f, " --level {}", level)?;
                }
                if let Some(experience) = &self.experience {
                    write!(f, " --experience {}", Experience(*experience))?;
                }
                Ok(())
            }
        }

        log::info!("version:\n\nv{}\n", env!("CARGO_PKG_VERSION"));

        let help = format!("{}", <ArgumentParser as CommandFactory>::command().render_help());
        log::info!("usage:\n\n{}\n", help.trim().trim_start_matches("Usage: "));

        let parser = {
            #[cfg(not(target_family = "wasm"))]
            {
                ArgumentParser::parse()
            }
            #[cfg(target_family = "wasm")]
            {
                let query = web_sys::window()
                    .and_then(|window| window.location().search().ok())
                    .unwrap_or("".to_owned());

                if query.is_empty() {
                    ArgumentParser::default()
                } else {
                    let processed_query = query.replace(['?', '&'], " --");

                    let mut parsed_arguments = vec![];
                    let mut parsed_argument = String::new();

                    let mut block = false;
                    for char in processed_query.trim_start().chars() {
                        match char {
                            '|' => {
                                block = !block;
                            },
                            ' ' | '=' if !block => {
                                parsed_arguments.push(std::mem::take(&mut parsed_argument));
                            },
                            _ => {
                                parsed_argument.push(char);
                            },
                        }
                    }
                    if !parsed_argument.is_empty() {
                        parsed_arguments.push(parsed_argument);
                    }

                    ArgumentParser::try_parse_from(
                        std::iter::once("mythmallow".to_owned()).chain(parsed_arguments),
                    )
                    .unwrap_or_else(|error| {
                        let full_error = format!("{}", error);
                        let short_error = full_error.split('\n').next().unwrap();

                        let error = short_error.trim_start_matches("error: ");
                        log::error!(
                            "unable to parse the arguments from the URL ({})",
                            error.replace("--", ""),
                        );

                        ArgumentParser::default()
                    })
                }
            }
        };

        let parser_display = format!("{}", parser);
        if !parser_display.is_empty() {
            log::info!("arguments:\n\n{}\n", parser_display.trim());
        }

        let configuration_directory = parser
            .configuration
            .as_ref()
            .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
            .unwrap_or_else(|| {
                #[cfg(not(target_family = "wasm"))]
                {
                    dirs::config_dir()
                        .map(|platform_config_dir| platform_config_dir.join("mythmallow"))
                        .unwrap_or_else(|| {
                            panic!("fatal: unable to determine the configuration directory",);
                        })
                }
                #[cfg(target_family = "wasm")]
                {
                    Path::new("local").join("configuration")
                }
            });

        let configuration_directory_display = format!("{}", configuration_directory.display());
        log::info!(
            "configuration directory:\n\n{}\n",
            configuration_directory_display
                .trim_start_matches("\\\\?\\")
                .trim_start_matches("\"\\\\?\\")
                .trim_end_matches('"'),
        );

        let data_directory = parser
            .data
            .as_ref()
            .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
            .unwrap_or_else(|| {
                #[cfg(not(target_family = "wasm"))]
                {
                    dirs::data_dir()
                        .map(|platform_data_dir| platform_data_dir.join("mythmallow"))
                        .unwrap_or_else(|| {
                            panic!("fatal: unable to determine the data directory");
                        })
                }
                #[cfg(target_family = "wasm")]
                {
                    Path::new("local").join("data")
                }
            });

        let data_directory_display = format!("{}", data_directory.display());
        log::info!(
            "data directory:\n\n{}\n",
            data_directory_display
                .trim_start_matches("\\\\?\\")
                .trim_start_matches("\"\\\\?\\")
                .trim_end_matches('"'),
        );

        let arguments = Arguments {
            data_directory,
            configuration_directory,
            seed: parser.seed,
            start_in_game: parser.game,
            start_in_game_mode: parser.mode,
            start_in_game_player: parser.player,
            start_in_game_enemies: parser.enemies,
            start_in_game_level: parser.level,
            start_in_game_experience: parser.experience,
        };
        app.insert_resource(arguments);
    }
}


/// Resource for the general settings of the application.
#[derive(Debug, Deserialize, Reflect, Resource, Serialize)]
#[serde(default)]
pub struct GeneralSettings {
    /// Desired locale.
    pub locale: String,

    /// Whether to pause the game when the application loses focus.
    pub pause_on_losing_focus: bool,
    /// Whether to show diagnostics overlay.
    pub show_diagnostics_overlay: bool,

    /// Whether to enable physics gizmos.
    #[cfg(feature = "development")]
    pub enable_physics_gizmos: bool,
}

impl GeneralSettings {
    /// Initializes the general settings from the configuration directory.
    pub fn initialize(app: &mut App) {
        let arguments = app.world().resource::<Arguments>();
        app.insert_resource(
            Persistent::<GeneralSettings>::builder()
                .name("general settings")
                .format(CONFIGURATION_STORAGE_FORMAT)
                .path({
                    #[cfg(not(target_family = "wasm"))]
                    {
                        arguments.configuration_directory.join("general-settings.toml")
                    }
                    #[cfg(target_family = "wasm")]
                    {
                        arguments.configuration_directory.join("general-settings")
                    }
                })
                .default(GeneralSettings::default())
                .revertible(true)
                .build()
                .unwrap_or_else(|_| {
                    panic!("fatal: unable to initialize persistent general settings")
                }),
        );
    }
}

impl GeneralSettings {
    /// Gets the desired locale.
    pub fn desired_locale(&self) -> Option<LanguageIdentifier> {
        self.locale.parse::<LanguageIdentifier>().ok()
    }
}

impl Default for GeneralSettings {
    fn default() -> GeneralSettings {
        GeneralSettings {
            locale: DefaultLocale::default().identifier().to_string(),

            pause_on_losing_focus: true,
            show_diagnostics_overlay: false,

            #[cfg(feature = "development")]
            enable_physics_gizmos: false,
        }
    }
}


/// Resource for the key bindings of the application.
#[derive(Debug, Deserialize, Reflect, Resource, Serialize)]
#[serde(default)]
pub struct KeyBindings {
    /// Keys to go up.
    pub up: SmallVec<[KeyCode; 2]>,
    /// Keys to go left.
    pub left: SmallVec<[KeyCode; 2]>,
    /// Keys to go down.
    pub down: SmallVec<[KeyCode; 2]>,
    /// Keys to go right.
    pub right: SmallVec<[KeyCode; 2]>,
    /// Keys to dash.
    pub dash: SmallVec<[KeyCode; 1]>,
    /// Keys to pause the game.
    pub pause: SmallVec<[KeyCode; 1]>,
    /// Keys to open the market, when the game mode allows it.
    pub market: SmallVec<[KeyCode; 1]>,
}

impl KeyBindings {
    /// Initializes the key bindings from the configuration directory.
    pub fn initialize(app: &mut App) {
        let arguments = app.world().resource::<Arguments>();
        app.insert_resource(
            Persistent::<KeyBindings>::builder()
                .name("key bindings")
                .format(CONFIGURATION_STORAGE_FORMAT)
                .path({
                    #[cfg(not(target_family = "wasm"))]
                    {
                        arguments.configuration_directory.join("key-bindings.toml")
                    }
                    #[cfg(target_family = "wasm")]
                    {
                        arguments.configuration_directory.join("key-bindings")
                    }
                })
                .default(KeyBindings::default())
                .revertible(true)
                .build()
                .unwrap_or_else(|_| panic!("fatal: unable to initialize persistent key bindings")),
        );
    }
}

impl Default for KeyBindings {
    fn default() -> KeyBindings {
        KeyBindings {
            up: smallvec![KeyCode::KeyW, KeyCode::ArrowUp],
            left: smallvec![KeyCode::KeyA, KeyCode::ArrowLeft],
            down: smallvec![KeyCode::KeyS, KeyCode::ArrowDown],
            right: smallvec![KeyCode::KeyD, KeyCode::ArrowRight],
            dash: smallvec![KeyCode::Space],
            pause: smallvec![KeyCode::Escape],
            market: smallvec![KeyCode::KeyB],
        }
    }
}
