#![forbid(unsafe_code)]

//! Dioxus Kinetics — the single intended downstream facade for the workspace.
//!
//! Downstream apps should depend on this crate alone and pull everything in
//! through the prelude:
//!
//! ```ignore
//! use kinetics::prelude::*;
//! ```
//!
//! The prelude re-exports the stable, semantically named public surface of the
//! underlying crates (components, tokens, motion primitives, and feature-gated
//! runtimes) so callers never reach into the individual `ui-*` crates directly.
//!
//! See `docs/ai-cheatsheet.md` for ready-to-paste prelude examples and
//! `docs/component-naming.md` for the naming conventions behind these exports.

pub mod prelude {
    pub use motion_core::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    #[allow(deprecated)]
    pub use ui_dioxus::{
        apply_kanban_move, format_bytes, password_strength, usage_fraction, usage_tone,
        use_form_error, validate, visible_window, Accordion, AccordionSection, ActionBar,
        ActionControl, ActionMenu, AgentStep, AgentStepState, AgentTimeline, AiStatus,
        AiStatusState, Alert, AlertTone, ArcGauge, AreaChart, AreaTrend, AssistantPanel,
        AssistantSide, AttachedFile, Attachment, AudioLevels, AuthCard, Avatar, AvatarSize, Badge,
        BadgeTone, BarChart, BillingHistory, BlankState, Breadcrumb, BreadcrumbItem, Button,
        ButtonVariant, CaptureStage, ChartSeries, ChartTone, Checkbox, ChipInput, ChoiceMark,
        CitationChip, Clip, CodeInput, Combobox, ComboboxOption, CommandFinder, CommandGroup,
        CommandItem, CommandMenu, ComparisonChart, ContentPlane, ContextHint, ConversionFunnel,
        Cue, DataTable, DataTableColumn, DataTableRow, DatePicker, DensityGrid, Dialog,
        DialogAction, DialogActionTone, DonutGauge, DropZone, DropdownMenu, DropdownMenuItem,
        EmptyState, EntryForm, FieldRules, FileChip, FileInput, FilePicker, Form, FormErrors,
        FormSchema, FormValues, FrameClip, FrameLayer, FrameStage, FunnelChart, FunnelStage,
        GaugeChart, GlassLayer, GlassSurface, GuidedTour, Heading, Heatmap, HeatmapRow, IconButton,
        IconButtonSize, IconButtonTone, Invoice, InvoiceList, InvoiceStatus, KanbanBoard,
        KanbanColumn, KanbanMove, KineticBox, KineticText, LineChart, MetricCard, MetricReadout,
        MetricTone, MfaCodeInput, ModalLayer, MotionPath, NavigationRail, NoticeStack, OAuthButton,
        OAuthProvider, OptionGroup, Pagination, PasswordStrength, PasswordStrengthMeter, PlanCard,
        PlanCtaVariant, PlanPicker, PlanTile, Popover, PopoverSide, Presence, PresenceCue,
        PresenceGate, PricingPlan, PricingTable, Progress, ProgressDial, PromptInput,
        ProportionMap, QuotaBar, RadioGroup, RadioOption, ReorderList, Scene, SceneContext,
        SegmentItem, SegmentedControl, Select, SelectOption, Sequence, SequenceContext,
        SharedElement, SharedLayout, Sheet, SheetSide, Sidebar, SidebarItem, SidebarSection,
        SignInCard, Skeleton, Slider, SocialAuthButton, SortDirection, SortableItem, SortableList,
        SourceCard, SourceRail, Sparkline, Spinner, SplitMode, SplitText, Spotlight, Stack,
        StateSwitch, Stepper, StepperStep, StreamingText, StrengthMeter, Surface, Switch, TabItem,
        TabPanel, Tabs, TagInput, Text, TextEntry, TextField, TextFieldType, TextVariant,
        TimelineScope, Toast, ToastEntry, ToastTone, Toaster, Toolbar, Tooltip, Tour,
        TourPlacement, TourStep, Treemap, TreemapItem, TrendChart, TrendLine, UploadZone,
        UsageMeter, UsageTone, ViewSwitcher, VirtualizedDataTable, VoiceInput, VoiceInputState,
        Waveform, WindowedDataTable, WorkflowBoard,
    };
    pub use ui_glass::{
        resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRecipe, GlassRequest, GlassTone,
    };
    pub use ui_layout::{compute_flip, FlipDelta, Rect};
    pub use ui_styles::{base_css, library_css, COMPONENT_CSS};
    #[cfg(feature = "timeline")]
    pub use ui_timeline::{
        Axis, MotionCue, PathPoint, ResolvedMotionState, Timeline, TimelineClock,
    };
    pub use ui_tokens::{
        export_tokens_css, Color, Density, MotionPreference, MotionScale, RadiusScale,
        SemanticColors, SpacingScale, Theme, ThemeMode, TransparencyPreference,
    };

    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
    pub use ui_dom::{glass_style, CssStyleWriter};

    #[cfg(feature = "native")]
    pub use ui_native::{plan_native_glass, NativeCapabilities, NativeGlassPlan};

    #[cfg(feature = "timeline")]
    pub use ui_timeline::{TimelineCapability, TimelineRuntime};

    #[cfg(feature = "composition")]
    pub use ui_composition::{ClipFill, Composition};

    #[cfg(feature = "capture")]
    pub use ui_capture::CaptureStageDescriptor;

    #[cfg(feature = "liquid-glass")]
    pub use ui_dioxus::{GlassPower, LiquidSurface, LiquidSurfaceProps};

    #[cfg(feature = "runtime")]
    pub use ui_runtime::{
        use_animation_value, use_density, use_element_computed_style, use_element_rect,
        use_presence_animation, use_presence_state, use_reduced_motion,
        use_shared_element_registry, use_theme_mode, use_timeline_sample, CssKeyframesAdapter,
        ElementSnapshot, FrameAdapter, FrameAdapterHandle, FrameAdapterRegistry,
        MountedRectCallback, ReducedMotion, SceneClock, SceneDriver, SceneState,
        ScrollObserverConfig, SequenceAdapter, SharedElementRegistry, SharedTransition,
        ThemeProvider, WaapiAdapter,
    };

    #[cfg(feature = "icons")]
    pub use ui_icons::*;

    #[cfg(feature = "blocks")]
    pub use ui_blocks::{
        Caption, LowerThird, LowerThirdAccent, MetricCounter, SocialOverlay, SocialPlatform,
        WipeTransition,
    };

    #[cfg(feature = "learn")]
    pub use ui_learn::{
        course_progress, grade_answer, next_review, normalize_short_answer, AchievementUnlock,
        CertificateCard, CourseLesson, CourseModule, CourseOutline, CourseProgressCard, Flashcard,
        FlashcardDeck, FlipCard, Leaderboard, LeaderboardEntry, LessonState, QuestionCard,
        QuizAnswer, QuizChoice, QuizPrompt, QuizQuestion, QuizResults, QuizTimer, ResumeLearning,
        ReviewRating, ReviewState, StreakBadge, XpBar,
    };
    #[cfg(feature = "learn")]
    pub use ui_learn::{
        CertificateCard as CompletionCertificate, CourseOutline as LearningPath,
        FlashcardDeck as ReviewDeck,
    };
}

/// The introspectable list of stable public names exposed through
/// [`prelude`]. This list is pinned by `tests/prelude.rs` so that renames or
/// accidental removals of the facade's public surface are caught at test time.
pub fn public_api_names() -> Vec<&'static str> {
    let mut names = vec![
        "Button",
        "ActionControl",
        "IconButton",
        "IconButtonTone",
        "IconButtonSize",
        "TextField",
        "TextEntry",
        "TextFieldType",
        "Checkbox",
        "ChoiceMark",
        "Switch",
        "StateSwitch",
        "Tabs",
        "ViewSwitcher",
        "Dialog",
        "ModalLayer",
        "Toast",
        "NoticeStack",
        "CommandMenu",
        "CommandFinder",
        "Tooltip",
        "ContextHint",
        "Toolbar",
        "ActionBar",
        "Sidebar",
        "NavigationRail",
        "MetricCard",
        "MetricReadout",
        "EmptyState",
        "BlankState",
        "Surface",
        "ContentPlane",
        "GlassSurface",
        "GlassLayer",
        "Presence",
        "PresenceCue",
        "Transition",
        "Sequence",
        "Cue",
        "SequenceContext",
        "Axis",
        "ResolvedMotionState",
        "SharedLayout",
        "SharedElement",
        "Timeline",
        "TimelineScope",
        "KineticBox",
        "KineticText",
        "PresenceGate",
        "Composition",
        "FrameStage",
        "FrameClip",
        "FrameLayer",
        "CaptureStage",
        "Combobox",
        "RadioGroup",
        "DropdownMenu",
        "OptionGroup",
        "ActionMenu",
        "Scene",
        "Clip",
        "SceneContext",
        "MotionPath",
        "PathPoint",
        "SplitText",
        "SplitMode",
        "StreamingText",
        "AiStatus",
        "CitationChip",
        "SourceCard",
        "SourceRail",
        "PromptInput",
        "AssistantPanel",
        "AgentTimeline",
        "Heading",
        "Text",
        "Badge",
        "Avatar",
        "Spinner",
        "Sheet",
        "Toaster",
        "ToastEntry",
        "Sparkline",
        "TrendLine",
        "LineChart",
        "TrendChart",
        "BarChart",
        "ComparisonChart",
        "DonutGauge",
        "ProgressDial",
        "ChartSeries",
        "ChartTone",
        "SortableList",
        "ReorderList",
        "SortableItem",
        "KanbanBoard",
        "WorkflowBoard",
        "KanbanColumn",
        "KanbanMove",
        "apply_kanban_move",
        "Tour",
        "GuidedTour",
        "TourStep",
        "TourPlacement",
        "Spotlight",
        "Waveform",
        "AudioLevels",
        "VoiceInput",
        "VoiceInputState",
        "Form",
        "EntryForm",
        "FieldRules",
        "FormSchema",
        "FormValues",
        "FormErrors",
        "validate",
        "use_form_error",
        "FileInput",
        "FilePicker",
        "DropZone",
        "UploadZone",
        "Attachment",
        "FileChip",
        "AttachedFile",
        "format_bytes",
        "TagInput",
        "ChipInput",
        "AreaChart",
        "AreaTrend",
        "FunnelChart",
        "ConversionFunnel",
        "FunnelStage",
        "GaugeChart",
        "ArcGauge",
        "Heatmap",
        "DensityGrid",
        "HeatmapRow",
        "Treemap",
        "ProportionMap",
        "TreemapItem",
        "VirtualizedDataTable",
        "WindowedDataTable",
        "visible_window",
        "SignInCard",
        "AuthCard",
        "OAuthButton",
        "SocialAuthButton",
        "OAuthProvider",
        "PasswordStrengthMeter",
        "StrengthMeter",
        "PasswordStrength",
        "password_strength",
        "MfaCodeInput",
        "CodeInput",
        "PricingTable",
        "PlanPicker",
        "PlanCard",
        "PlanTile",
        "PricingPlan",
        "PlanCtaVariant",
        "UsageMeter",
        "QuotaBar",
        "UsageTone",
        "usage_fraction",
        "usage_tone",
        "InvoiceList",
        "BillingHistory",
        "Invoice",
        "InvoiceStatus",
        "export_tokens_css",
    ];

    #[cfg(feature = "icons")]
    names.extend_from_slice(&[
        "Close",
        "Check",
        "ChevronDown",
        "ChevronRight",
        "Plus",
        "Minus",
        "Trash",
        "Search",
        "Sparkle",
        "Stop",
        "Send",
        "Quote",
        "Globe",
        "Copy",
        "Link",
    ]);

    #[cfg(feature = "runtime")]
    names.extend_from_slice(&[
        "use_timeline_sample",
        "SharedTransition",
        "SharedElementRegistry",
        "ElementSnapshot",
        "use_shared_element_registry",
        "use_element_rect",
        "use_element_computed_style",
        "SceneState",
        "SceneClock",
        "SceneDriver",
        "ScrollObserverConfig",
        "FrameAdapter",
        "FrameAdapterRegistry",
        "SequenceAdapter",
        "WaapiAdapter",
        "CssKeyframesAdapter",
        "ThemeProvider",
        "use_theme_mode",
        "use_density",
    ]);

    #[cfg(feature = "blocks")]
    names.extend_from_slice(&[
        "LowerThird",
        "LowerThirdAccent",
        "Caption",
        "WipeTransition",
        "MetricCounter",
        "SocialOverlay",
        "SocialPlatform",
    ]);

    #[cfg(feature = "composition")]
    names.push("ClipFill");

    #[cfg(feature = "liquid-glass")]
    names.push("LiquidSurface");

    #[cfg(feature = "learn")]
    names.extend_from_slice(&[
        "CourseOutline",
        "LearningPath",
        "CourseLesson",
        "CourseModule",
        "LessonState",
        "course_progress",
        "CourseProgressCard",
        "ResumeLearning",
        "QuestionCard",
        "QuizQuestion",
        "QuizPrompt",
        "QuizAnswer",
        "QuizChoice",
        "QuizResults",
        "QuizTimer",
        "grade_answer",
        "normalize_short_answer",
        "FlipCard",
        "FlashcardDeck",
        "ReviewDeck",
        "Flashcard",
        "ReviewRating",
        "ReviewState",
        "next_review",
        "XpBar",
        "StreakBadge",
        "AchievementUnlock",
        "Leaderboard",
        "LeaderboardEntry",
        "CertificateCard",
        "CompletionCertificate",
    ]);

    names
}

#[cfg(feature = "timeline")]
pub mod timeline {
    pub use ui_timeline::{TimelineCapability, TimelineRuntime};
}

#[cfg(feature = "composition")]
pub mod composition {
    pub use ui_composition::Composition;
}

#[cfg(feature = "capture")]
pub mod capture {
    pub use ui_capture::CaptureStageDescriptor;
}
