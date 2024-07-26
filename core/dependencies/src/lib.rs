//! Dependencies of the game.

#[doc(inline)]
pub use {
    avian2d::{
        self,
        math::*,
        prelude::*,
        PhysicsPlugins as AvianPlugin,
    },
    bevy::{
        self,
        asset::{
            self as bevy_asset,
            io::Reader,
            AssetLoader,
            AsyncReadExt,
            LoadContext,
            LoadState,
            LoadedFolder,
        },
        diagnostic::{
            DiagnosticsStore,
            EntityCountDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
        },
        ecs::{
            self as bevy_ecs,
            component::{
                ComponentHooks,
                StorageType,
            },
            system::{
                EntityCommands,
                RunSystemOnce,
                SystemId,
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
        render::{
            self as bevy_render,
            render_resource::{
                encase,
                AsBindGroup,
                ShaderRef,
            },
        },
        sprite::MaterialMesh2dBundle,
        state as bevy_state,
        window::{
            PrimaryWindow,
            WindowResized,
        },
        winit::WinitWindows,
    },
    bevy_console::{
        self,
        reply,
        AddConsoleCommand,
        ConsoleCommand,
        ConsoleConfiguration,
        ConsoleOpen as ConsoleState,
        ConsolePlugin as BevyConsolePlugin,
        ConsoleSet,
    },
    bevy_easings::{
        self,
        Ease,
        EaseFunction,
        EasingChainComponent,
        EasingComponent,
        EasingState,
        EasingType,
        EasingsPlugin,
    },
    bevy_fluent::{
        self,
        prelude::*,
        ResourceAsset,
    },
    bevy_persistent::{
        self,
        prelude::*,
    },
    bevy_prng::{
        self,
        WyRand,
    },
    bevy_rand::{
        self,
        prelude::*,
    },
    clap::{
        self,
        CommandFactory,
        Parser,
        Subcommand,
    },
    egui::{
        self,
        Color32,
    },
    fluent::{
        self,
        bundle::FluentBundle,
        FluentArgs,
        FluentResource,
    },
    fluent_content::{
        self,
        Content,
        Request,
    },
    leafwing_input_manager::{
        self,
        prelude::*,
    },
    rand::{
        self,
        prelude::*,
    },
    serde::{
        self,
        Deserialize,
        Serialize,
    },
    sickle_ui::{
        self,
        prelude::*,
        SickleUiPlugin,
    },
    smallvec::{
        smallvec,
        SmallVec,
    },
    smol_str::{
        self,
        format_smolstr,
        SmolStr,
    },
    std::{
        any::Any,
        borrow::Cow,
        cmp::Ordering,
        fmt::{
            self,
            Debug,
            Display,
        },
        marker::PhantomData,
        num::{
            NonZeroU16,
            NonZeroU8,
        },
        ops::{
            Deref,
            DerefMut,
            Index,
        },
        path::{
            Path,
            PathBuf,
        },
        sync::{
            atomic::{
                AtomicBool,
                Ordering as AtomicOrdering,
            },
            Arc,
            Mutex,
        },
        time::Duration,
    },
    strum::{
        self,
        IntoEnumIterator,
    },
    strum_macros::{
        self,
        EnumIter,
    },
    sys_locale::{
        self,
    },
    thiserror::{
        self,
        Error,
    },
    toml::{
        self,
    },
    typed_builder::{
        self,
        TypedBuilder,
    },
    unic_langid::{
        self,
        LanguageIdentifier,
    },
};

#[cfg(not(target_family = "wasm"))]
#[doc(inline)]
pub use {
    bevy::window::WindowMode,
    bevy_persistent_windows::{
        self,
        prelude::*,
    },
    dirs::{
        self,
    },
    std::time::Instant,
};

#[cfg(target_family = "wasm")]
#[doc(inline)]
pub use {
    console_error_panic_hook::{
        self,
    },
    web_instant::Instant,
    web_sys::{
        self,
    },
};

#[cfg(feature = "bevy_editor_pls")]
#[doc(inline)]
pub use bevy_editor_pls::{
    self,
    controls::{
        Action as EditorAction,
        Binding as EditorBinding,
        BindingCondition as EditorBindingCondition,
        Button as EditorButton,
        EditorControls,
        UserInput as EditorUserInput,
    },
    editor::Editor,
    prelude::*,
};
