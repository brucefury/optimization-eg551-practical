//! Debug configuration for experiment output.

/// Configuration for debug output during experiments.
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Whether to print verbose progress messages.
    pub verbose: bool,
    /// Whether to show timing information.
    pub show_timings: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            verbose: true,
            show_timings: true,
        }
    }
}

impl DebugConfig {
    /// Create a new debug config with default settings (verbose with timings).
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a quiet config (no output).
    pub fn quiet() -> Self {
        Self {
            verbose: false,
            show_timings: false,
        }
    }

    /// Set verbose output.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Set timing output.
    pub fn with_timings(mut self, show_timings: bool) -> Self {
        self.show_timings = show_timings;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DebugConfig::default();
        assert!(config.verbose);
        assert!(config.show_timings);
    }

    #[test]
    fn test_quiet_config() {
        let config = DebugConfig::quiet();
        assert!(!config.verbose);
        assert!(!config.show_timings);
    }

    #[test]
    fn test_builder_methods() {
        let config = DebugConfig::new()
            .with_verbose(false)
            .with_timings(true);
        assert!(!config.verbose);
        assert!(config.show_timings);
    }
}
