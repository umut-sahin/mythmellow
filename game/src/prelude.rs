#[doc(inline)]
pub use crate::{
    camera::components::*,
    combat::components::*,
    configuration::resources::*,
    core::{
        depths::*,
        resources::*,
        sets::*,
        states::*,
    },
    enemy::{
        components::*,
        interfaces::*,
        registry::*,
        resources::*,
    },
    input::actions::*,
    inventory::resources::*,
    items::{
        components::*,
        interfaces::*,
        registry::*,
    },
    leveling::{
        components::*,
        events::*,
        resources::*,
    },
    map::{
        components::*,
        resources::*,
    },
    market::resources::*,
    mode::{
        conditions::in_game_mode,
        interfaces::*,
        registry::*,
        resources::*,
    },
    movement::components::*,
    physics::layers::*,
    player::{
        components::*,
        conditions::*,
        interfaces::*,
        registry::*,
        resources::*,
    },
    plugin::MythmallowPlugin,
    property::components::*,
    status_effect::components::*,
    status_effect::systems::cooldown,
    ui::{
        diagnostics_overlay::components::*,
        enemy_selection_screen::components::*,
        game_mode_selection_screen::components::*,
        game_over_menu::components::*,
        hud::{
            components::*,
            materials::*,
        },
        main_menu::components::*,
        market::{
            components::*,
            resources::*,
        },
        pause_menu::components::*,
        player_selection_screen::components::*,
        widget::components::*,
    },
};

pub mod utils {
    pub use crate::{
        combat::utils as combat,
        map::utils as map,
    };
}

#[doc(inline)]
pub use {
    bevy::ui::Display as UiDisplay,
    bevy::{
        app::AppExit,
        diagnostic::{
            DiagnosticsStore,
            EntityCountDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
        },
        ecs::{
            self as bevy_ecs,
            system::SystemId,
            system::{
                EntityCommands,
                RunSystemOnce,
                SystemState,
            },
        },
        input::mouse::MouseMotion,
        log::{
            self,
            LogPlugin,
        },
        prelude::*,
        reflect as bevy_reflect,
        render::render_resource::{
            AsBindGroup,
            ShaderRef,
        },
        sprite::MaterialMesh2dBundle,
        transform::TransformSystem,
        utils::HashMap,
        utils::HashSet,
        window::{
            PrimaryWindow,
            WindowFocused,
        },
    },
    bevy_console::{
        reply,
        AddConsoleCommand,
        ConsoleCommand,
        ConsoleConfiguration,
        ConsoleOpen as ConsoleState,
        ConsolePlugin as BevyConsolePlugin,
        ConsoleSet,
    },
    bevy_persistent::prelude::*,
    bevy_prng::ChaCha8Rng,
    bevy_rand::prelude::*,
    bevy_xpbd_2d::{
        math::*,
        plugins::PhysicsPlugins as XpbdPlugin,
        prelude::*,
    },
    clap::{
        self,
        CommandFactory,
        Parser,
        Subcommand,
    },
    core::num::NonZeroU8,
    itertools::Itertools,
    leafwing_input_manager::{
        action_state::ActionData,
        buttonlike::ButtonState,
        prelude::*,
    },
    num_format::{
        Locale,
        ToFormattedString,
    },
    prettytable::{
        row,
        Table,
    },
    rand::prelude::*,
    serde::{
        Deserialize,
        Serialize,
    },
    smallvec::{
        smallvec,
        SmallVec,
    },
    smol_str::SmolStr,
    std::borrow::Cow,
    std::sync::atomic::AtomicBool,
    std::sync::atomic::Ordering as AtomicOrdering,
    std::{
        any::{
            Any,
            TypeId,
        },
        cmp::Ordering,
        fmt::{
            self,
            Debug,
            Display,
        },
        marker::PhantomData,
        num::{
            NonZeroU16,
            NonZeroUsize,
        },
        ops::{
            Deref,
            DerefMut,
            Index,
        },
        path::PathBuf,
        sync::{
            Arc,
            Mutex,
        },
        time::Duration,
    },
    strum::IntoEnumIterator,
    strum_macros::EnumIter,
    typed_builder::TypedBuilder,
};

#[cfg(feature = "native")]
#[doc(inline)]
pub use {
    bevy::window::{
        ExitCondition,
        WindowMode,
    },
    bevy_persistent_windows::prelude::*,
    std::time::Instant,
};

#[cfg(feature = "wasm")]
#[doc(inline)]
pub use {
    instant::Instant,
    std::path::Path,
    web_sys::{
        self,
        wasm_bindgen::JsCast,
        HtmlCanvasElement,
    },
};

#[cfg(feature = "bevy_editor_pls")]
#[doc(inline)]
pub use bevy_editor_pls::{
    controls::{
        Action as EditorAction,
        Binding as EditorBinding,
        BindingCondition as EditorBindingCondition,
        Button as EditorButton,
        EditorControls,
        UserInput as EditorUserInput,
    },
    prelude::*,
};
