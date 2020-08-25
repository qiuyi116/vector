use super::InternalEvent;
use metrics::counter;

define_events_processed!(GrokParserEventProcessed, "transform", "grok_parser");

#[derive(Debug)]
pub(crate) struct GrokParserFailedMatch<'a> {
    pub value: &'a str,
}

impl InternalEvent for GrokParserFailedMatch<'_> {
    fn emit_logs(&self) {
        warn!(
            message = "Grok pattern failed to match.",
            field = &super::truncate_string_at(self.value, 60)[..],
            rate_limit_secs = 30
        );
    }

    fn emit_metrics(&self) {
        counter!("processing_error", 1,
            "component_kind" => "transform",
            "component_type" => "grok_parser",
            "error_type" => "failed_match",
        );
    }
}

#[derive(Debug)]
pub(crate) struct GrokParserMissingField<'a> {
    pub field: &'a str,
}

impl InternalEvent for GrokParserMissingField<'_> {
    fn emit_logs(&self) {
        debug!(message = "Field does not exist.", field = %self.field);
    }

    fn emit_metrics(&self) {
        counter!("processing_error", 1,
            "component_kind" => "transform",
            "component_type" => "grok_parser",
            "error_type" => "missing_field",
        );
    }
}

#[derive(Debug)]
pub(crate) struct GrokParserConversionFailed<'a> {
    pub name: &'a str,
    pub error: crate::types::Error,
}

impl<'a> InternalEvent for GrokParserConversionFailed<'a> {
    fn emit_logs(&self) {
        debug!(
            message = "Could not convert types.",
            name = %self.name,
            error = %self.error,
            rate_limit_secs = 30
        );
    }

    fn emit_metrics(&self) {
        counter!("processing_error", 1,
            "component_kind" => "transform",
            "component_type" => "grok_parser",
            "error_type" => "type_conversion_failed",
        );
    }
}