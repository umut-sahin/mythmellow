//! Preludes of the game dependencies.

#[doc(inline)]
pub use {
    avian2d::{
        self,
        PhysicsPlugins as AvianPlugin,
        math::*,
        prelude::*,
    },
    bevy::{
        self,
        asset::{
            self as bevy_asset,
            AssetLoader,
            AsyncReadExt,
            LoadContext,
            LoadState,
            LoadedFolder,
            io::Reader,
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
                AsBindGroup,
                ShaderRef,
                encase,
            },
        },
        sprite::MaterialMesh2dBundle,
        state as bevy_state,
        utils::HashSet,
        window::{
            PrimaryWindow,
            WindowFocused,
            WindowResized,
        },
        winit::WinitWindows,
    },
    bevy_console::{
        self,
        AddConsoleCommand,
        ConsoleCommand,
        ConsoleConfiguration,
        ConsoleOpen as ConsoleState,
        ConsolePlugin as BevyConsolePlugin,
        ConsoleSet,
        reply,
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
        ResourceAsset,
        prelude::*,
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
        FluentArgs,
        FluentResource,
        bundle::FluentBundle,
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
    prettytable::{
        self,
        Table,
        row,
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
        SickleUiPlugin,
        prelude::*,
    },
    smallvec::{
        SmallVec,
        smallvec,
    },
    smol_str::{
        self,
        SmolStr,
        format_smolstr,
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
            NonZeroU8,
            NonZeroU16,
            NonZeroUsize,
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
            Arc,
            Mutex,
            atomic::{
                AtomicBool,
                Ordering as AtomicOrdering,
            },
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
    web_instant::{
        self,
        Instant,
    },
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

#[cfg(feature = "rerun")]
pub use revy::{
    self,
    RecordingStreamBuilder,
    RerunPlugin,
};
