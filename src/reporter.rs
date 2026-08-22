use crate::pattern::ScanReport;

/// Format scan reports as human-readable text output.
///
/// Shows each file with findings, per-finding details (line, severity,
/// message, remediation, pattern ID), and a summary line with counts.
pub fn format_text(reports: &[ScanReport]) -> String {
    let mut output = String::new();

    for report in reports {
        if !report.has_findings() {
            continue;
        }

        output.push_str(&format!("\n{}\n", report.file));

        for m in &report.matches {
            output.push_str(&format!(
                "  :{} {}  {} — {}  ({})\n",
                m.line, m.severity, m.message, m.remediation, m.pattern_id
            ));
        }
    }

    let total_suppressed: usize = reports.iter().map(|r| r.suppressed_count()).sum();
    let total_critical: usize = reports.iter().map(|r| r.critical_count).sum();
    let total_high: usize = reports.iter().map(|r| r.high_count).sum();
    let total_medium: usize = reports.iter().map(|r| r.medium_count).sum();
    let total_low: usize = reports.iter().map(|r| r.low_count).sum();
    let total = total_critical + total_high + total_medium + total_low;

    if total == 0 {
        output.push_str("No injection patterns detected.\n");
    } else {
        output.push_str(&format!(
            "\n{} finding(s): {} critical, {} high, {} medium, {} low\n",
            total, total_critical, total_high, total_medium, total_low
        ));
    }

    // Suppression is invoked by the scanned document itself. If a file silences
    // findings, that must be visible — otherwise an untrusted document can
    // disarm the scanner and look identical to a clean one.
    if total_suppressed > 0 {
        let files_with_suppressions = reports.iter().filter(|r| r.suppressed_count() > 0).count();
        let (findings, them) = if total_suppressed == 1 {
            ("finding", "it")
        } else {
            ("findings", "them")
        };
        let files = if files_with_suppressions == 1 {
            "file"
        } else {
            "files"
        };
        output.push_str(&format!(
            "{total_suppressed} {findings} suppressed by directives in {files_with_suppressions} \
             scanned {files}. Re-run with --no-suppress to see {them}.\n"
        ));
    }

    output
}

/// Format scan reports as JSON.
///
/// Returns `Result<String, serde_json::Error>` (not `anyhow`) so
/// callers can handle serialization errors precisely.
pub fn format_json(reports: &[ScanReport]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(reports)
}
