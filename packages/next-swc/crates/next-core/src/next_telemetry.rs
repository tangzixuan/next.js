use turbopack_binding::{turbo::tasks::Vc, turbopack::core::diagnostics::Diagnostic};

/// A struct represent telemetry event for the feature usage,
/// referred as `importing` a certain module. (i.e importing @next/image)
#[turbo_tasks::value(shared)]
pub struct ModuleFeatureTelemetry {
    pub event_name: String,
    pub feature_name: String,
    pub invocation_count: usize,
}

#[turbo_tasks::value_impl]
impl Diagnostic for ModuleFeatureTelemetry {
    #[turbo_tasks::function]
    fn category(&self) -> Vc<String> {
        Vc::cell("ModuleFeatureTelemetry_category_tbd".to_string())
    }

    #[turbo_tasks::function]
    fn key(&self) -> Vc<String> {
        Vc::cell(self.feature_name.to_string())
    }

    #[turbo_tasks::function]
    fn value(&self) -> Vc<String> {
        Vc::cell(self.invocation_count.to_string())
    }
}
