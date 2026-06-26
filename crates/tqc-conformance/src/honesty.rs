//! The honesty meta-gate: a mechanical proof that the suite stays honest.
//!
//! It cross-checks the model against the feature files on disk and enforces status
//! discipline — mirroring F1's `scripts/honesty_audit.sh`, promoted to a typed invariant.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use tqc_model::{Model, Tier};

/// A summary of a passing audit.
#[derive(Debug, Clone, Copy)]
pub struct AuditReport {
    /// Total dictionary rows.
    pub rows_total: usize,
    /// Gating, green suite rows.
    pub suites: usize,
    /// Expected-RED, non-gating target rows.
    pub targets: usize,
    /// Absent (crux) rows.
    pub absent: usize,
    /// Feature files discovered on disk.
    pub features_on_disk: usize,
}

/// Run the full honesty audit against a workspace rooted at `root`.
///
/// # Errors
/// Returns a description of the first violation found.
pub fn audit(model: &Model, root: &Path) -> Result<AuditReport, String> {
    let mut referenced = BTreeSet::new();

    for row in &model.rows {
        match row.tier {
            Tier::Absent => {
                if !row.feature.is_empty() {
                    return Err(format!("absent row `{}` must not name a feature", row.id));
                }
            }
            Tier::Suite | Tier::Target => {
                let rel = normalize(&row.feature);
                let path = root.join(&rel);
                if !path.is_file() {
                    return Err(format!("row `{}`: feature file missing: {rel}", row.id));
                }
                let text = fs::read_to_string(&path).map_err(|e| format!("read {rel}: {e}"))?;
                let tag = format!("@row:{}", row.id);
                if !text.contains(&tag) {
                    return Err(format!("feature `{rel}` is missing the `{tag}` tag"));
                }
                let prefix = match row.tier {
                    Tier::Suite => "features/suites/",
                    Tier::Target => "features/targets/",
                    Tier::Absent => unreachable!(),
                };
                if !rel.starts_with(prefix) {
                    return Err(format!(
                        "row `{}`: tier requires prefix `{prefix}` (got `{rel}`)",
                        row.id
                    ));
                }
                referenced.insert(rel);
            }
        }

        // Status discipline: a non-gating level may never be a gating suite.
        let status = model
            .status(&row.status)
            .ok_or_else(|| format!("row `{}`: unknown status `{}`", row.id, row.status))?;
        if !status.gating && row.tier == Tier::Suite {
            return Err(format!(
                "row `{}`: non-gating status `{}` may not be a gating suite",
                row.id, row.status
            ));
        }
    }

    // Bidirectional coverage: dictionary <-> features on disk.
    let mut on_disk = BTreeSet::new();
    collect_features(&root.join("features"), root, &mut on_disk)?;
    let missing: Vec<_> = referenced.difference(&on_disk).cloned().collect();
    let orphan: Vec<_> = on_disk.difference(&referenced).cloned().collect();
    if !missing.is_empty() || !orphan.is_empty() {
        return Err(format!(
            "feature coverage mismatch: missing-on-disk={missing:?}, orphan-not-in-dictionary={orphan:?}"
        ));
    }

    // No feature may affirmatively assert an open/none claim.
    for rel in &on_disk {
        let text = fs::read_to_string(root.join(rel)).map_err(|e| format!("read {rel}: {e}"))?;
        for (i, line) in text.lines().enumerate() {
            if affirmative_forbidden(line) {
                return Err(format!(
                    "forbidden assertion of an open/none claim in {rel}:{}: {}",
                    i + 1,
                    line.trim()
                ));
            }
        }
    }

    // The inner product must be the Euclidean companion, with no crux-form smuggling.
    no_smuggling(root)?;

    Ok(AuditReport {
        rows_total: model.rows.len(),
        suites: model.rows_in_tier(Tier::Suite).count(),
        targets: model.rows_in_tier(Tier::Target).count(),
        absent: model.rows_in_tier(Tier::Absent).count(),
        features_on_disk: on_disk.len(),
    })
}

fn normalize(rel: &str) -> String {
    rel.replace('\\', "/")
}

fn collect_features(dir: &Path, root: &Path, out: &mut BTreeSet<String>) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_features(&path, root, out)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("feature") {
            let rel = path
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(rel);
        }
    }
    Ok(())
}

/// Whether a line affirmatively asserts one of the forbidden (open/none) claims.
fn affirmative_forbidden(line: &str) -> bool {
    let l = line.to_lowercase();
    let subject = [
        "riemann",
        "liposit",
        "li-posit",
        "universal",
        "advantage",
        "speedup",
    ]
    .iter()
    .any(|s| l.contains(s));
    if !subject {
        return false;
    }
    let hedged = [
        "open",
        "not asserted",
        "never asserted",
        "unasserted",
        "remains",
        "reported",
        "recorded",
        "measured",
        "unknown",
        "without",
        "distinct",
    ]
    .iter()
    .any(|h| l.contains(h));
    if hedged {
        return false;
    }
    [
        "holds",
        "is proven",
        "proven",
        "is established",
        "established",
        "is true",
        "guaranteed",
        "is dense",
        "densely",
    ]
    .iter()
    .any(|a| l.contains(a))
}

fn no_smuggling(root: &Path) -> Result<(), String> {
    let inner = root.join("crates/tqc-core/src/inner.rs");
    let text = fs::read_to_string(&inner).map_err(|e| format!("read inner.rs: {e}"))?;
    if !text.contains("euclidean_norm_sq") {
        return Err("the inner product must be the Euclidean companion (euclidean_norm_sq)".into());
    }
    for bad in [
        "PrimeForm",
        "liPositivity",
        "hodge_index",
        "signed_prime_form",
    ] {
        if text.contains(bad) {
            return Err(format!(
                "inner.rs references a crux-form symbol `{bad}` — smuggling"
            ));
        }
    }
    Ok(())
}
