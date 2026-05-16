#[derive(Debug, Clone, PartialEq)]
pub struct Site {
    pub id: String,
    pub demand: f64,
    pub revenue: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Territory {
    pub id: String,
    pub label: String,
    pub assignees: Vec<String>,
    pub sites: Vec<Site>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerritorySummary {
    pub territory_id: String,
    pub site_count: usize,
    pub demand: f64,
    pub revenue: f64,
    pub assignee_count: usize,
    pub assignees: Vec<String>,
    pub centroid_latitude: f64,
    pub centroid_longitude: f64,
    pub max_radius_degrees: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BalanceAudit {
    pub summaries: Vec<TerritorySummary>,
    pub demand_spread_ratio: f64,
    pub revenue_spread_ratio: f64,
    pub max_radius_degrees: f64,
    pub passes: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerritoryScenarioDelta {
    pub territory_id: String,
    pub baseline_site_count: usize,
    pub proposed_site_count: usize,
    pub site_count_delta: isize,
    pub baseline_demand: f64,
    pub proposed_demand: f64,
    pub demand_delta: f64,
    pub baseline_revenue: f64,
    pub proposed_revenue: f64,
    pub revenue_delta: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioComparison {
    pub baseline: BalanceAudit,
    pub proposed: BalanceAudit,
    pub territory_deltas: Vec<TerritoryScenarioDelta>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompactnessException {
    pub territory_id: String,
    pub site_count: usize,
    pub max_radius_degrees: f64,
    pub threshold_degrees: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiteMovement {
    pub site_id: String,
    pub baseline_territory_id: Option<String>,
    pub proposed_territory_id: Option<String>,
    pub movement_kind: String,
    pub demand: f64,
    pub revenue: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssigneeCapacity {
    pub assignee: String,
    pub team: String,
    pub capacity: f64,
    pub home_latitude: f64,
    pub home_longitude: f64,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CapacityException {
    pub territory_id: String,
    pub demand: f64,
    pub capacity: f64,
    pub overload: f64,
    pub assignees: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartitionSweepResult {
    pub target_territory_count: usize,
    pub actual_territory_count: usize,
    pub audit: BalanceAudit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerritoryVisualOptions {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CsvIntakeError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CsvDiagnostic {
    pub severity: String,
    pub line: usize,
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartitionError {
    EmptySiteSet,
    ZeroTerritories,
}

impl std::fmt::Display for CsvIntakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CSV intake error on line {}: {}",
            self.line, self.message
        )
    }
}

impl std::error::Error for CsvIntakeError {}

impl std::fmt::Display for PartitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySiteSet => write!(f, "cannot partition an empty site set"),
            Self::ZeroTerritories => write!(f, "target territory count must be greater than zero"),
        }
    }
}

impl std::error::Error for PartitionError {}

impl Default for TerritoryVisualOptions {
    fn default() -> Self {
        Self {
            width: 960,
            height: 640,
            title: "TERRAIN territory split".to_string(),
        }
    }
}

impl Site {
    pub fn new(
        id: impl Into<String>,
        demand: f64,
        revenue: f64,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            id: id.into(),
            demand,
            revenue,
            latitude,
            longitude,
        }
    }
}

impl Territory {
    pub fn new(id: impl Into<String>, sites: Vec<Site>) -> Self {
        let id = id.into();
        Self {
            label: id.clone(),
            id,
            assignees: Vec::new(),
            sites,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_assignees(
        mut self,
        assignees: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.assignees = assignees.into_iter().map(Into::into).collect();
        self
    }
}

pub fn summarize_territory(territory: &Territory) -> TerritorySummary {
    let site_count = territory.sites.len();
    let demand = territory.sites.iter().map(|site| site.demand).sum();
    let revenue = territory.sites.iter().map(|site| site.revenue).sum();
    let centroid_latitude = mean(site_count, territory.sites.iter().map(|site| site.latitude));
    let centroid_longitude = mean(
        site_count,
        territory.sites.iter().map(|site| site.longitude),
    );
    let max_radius_degrees = territory
        .sites
        .iter()
        .map(|site| {
            let d_lat = site.latitude - centroid_latitude;
            let d_lon = site.longitude - centroid_longitude;
            (d_lat * d_lat + d_lon * d_lon).sqrt()
        })
        .fold(0.0, f64::max);

    TerritorySummary {
        territory_id: territory.id.clone(),
        site_count,
        demand,
        revenue,
        assignee_count: territory.assignees.len(),
        assignees: territory.assignees.clone(),
        centroid_latitude,
        centroid_longitude,
        max_radius_degrees,
    }
}

pub fn audit_territories(
    territories: &[Territory],
    max_demand_spread_ratio: f64,
    max_revenue_spread_ratio: f64,
) -> BalanceAudit {
    let summaries = territories
        .iter()
        .map(summarize_territory)
        .collect::<Vec<_>>();
    let demand_spread_ratio = spread_ratio(summaries.iter().map(|summary| summary.demand));
    let revenue_spread_ratio = spread_ratio(summaries.iter().map(|summary| summary.revenue));
    let max_radius_degrees = summaries
        .iter()
        .map(|summary| summary.max_radius_degrees)
        .fold(0.0, f64::max);
    let passes = demand_spread_ratio <= max_demand_spread_ratio
        && revenue_spread_ratio <= max_revenue_spread_ratio;

    BalanceAudit {
        summaries,
        demand_spread_ratio,
        revenue_spread_ratio,
        max_radius_degrees,
        passes,
    }
}

pub fn compare_territory_plans(
    baseline: &[Territory],
    proposed: &[Territory],
    max_demand_spread_ratio: f64,
    max_revenue_spread_ratio: f64,
) -> ScenarioComparison {
    let baseline_audit =
        audit_territories(baseline, max_demand_spread_ratio, max_revenue_spread_ratio);
    let proposed_audit =
        audit_territories(proposed, max_demand_spread_ratio, max_revenue_spread_ratio);
    let baseline_summaries = baseline_audit
        .summaries
        .iter()
        .map(|summary| (summary.territory_id.clone(), summary))
        .collect::<std::collections::BTreeMap<_, _>>();
    let proposed_summaries = proposed_audit
        .summaries
        .iter()
        .map(|summary| (summary.territory_id.clone(), summary))
        .collect::<std::collections::BTreeMap<_, _>>();
    let territory_ids = baseline_summaries
        .keys()
        .chain(proposed_summaries.keys())
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    let territory_deltas = territory_ids
        .into_iter()
        .map(|territory_id| {
            let baseline = baseline_summaries.get(&territory_id);
            let proposed = proposed_summaries.get(&territory_id);
            let baseline_site_count = baseline.map_or(0, |summary| summary.site_count);
            let proposed_site_count = proposed.map_or(0, |summary| summary.site_count);
            let baseline_demand = baseline.map_or(0.0, |summary| summary.demand);
            let proposed_demand = proposed.map_or(0.0, |summary| summary.demand);
            let baseline_revenue = baseline.map_or(0.0, |summary| summary.revenue);
            let proposed_revenue = proposed.map_or(0.0, |summary| summary.revenue);
            TerritoryScenarioDelta {
                territory_id,
                baseline_site_count,
                proposed_site_count,
                site_count_delta: proposed_site_count as isize - baseline_site_count as isize,
                baseline_demand,
                proposed_demand,
                demand_delta: proposed_demand - baseline_demand,
                baseline_revenue,
                proposed_revenue,
                revenue_delta: proposed_revenue - baseline_revenue,
            }
        })
        .collect();

    ScenarioComparison {
        baseline: baseline_audit,
        proposed: proposed_audit,
        territory_deltas,
    }
}

pub fn compactness_exceptions(
    territories: &[Territory],
    threshold_degrees: f64,
) -> Vec<CompactnessException> {
    territories
        .iter()
        .map(summarize_territory)
        .filter(|summary| summary.max_radius_degrees > threshold_degrees)
        .map(|summary| CompactnessException {
            territory_id: summary.territory_id,
            site_count: summary.site_count,
            max_radius_degrees: summary.max_radius_degrees,
            threshold_degrees,
        })
        .collect()
}

pub fn site_movements(baseline: &[Territory], proposed: &[Territory]) -> Vec<SiteMovement> {
    let baseline_sites = territory_site_index(baseline);
    let proposed_sites = territory_site_index(proposed);
    let site_ids = baseline_sites
        .keys()
        .chain(proposed_sites.keys())
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    site_ids
        .into_iter()
        .map(|site_id| {
            let baseline = baseline_sites.get(&site_id);
            let proposed = proposed_sites.get(&site_id);
            let baseline_territory_id = baseline.map(|(territory_id, _)| territory_id.to_string());
            let proposed_territory_id = proposed.map(|(territory_id, _)| territory_id.to_string());
            let movement_kind = match (&baseline_territory_id, &proposed_territory_id) {
                (Some(left), Some(right)) if left == right => "unchanged",
                (Some(_), Some(_)) => "moved",
                (Some(_), None) => "removed",
                (None, Some(_)) => "added",
                (None, None) => "unknown",
            }
            .to_string();
            let site = proposed
                .map(|(_, site)| *site)
                .or_else(|| baseline.map(|(_, site)| *site))
                .expect("site id came from one of the indexes");
            SiteMovement {
                site_id,
                baseline_territory_id,
                proposed_territory_id,
                movement_kind,
                demand: site.demand,
                revenue: site.revenue,
            }
        })
        .collect()
}

pub fn capacity_exceptions(
    territories: &[Territory],
    capacities: &[AssigneeCapacity],
) -> Vec<CapacityException> {
    let capacity_by_assignee = capacities
        .iter()
        .map(|capacity| (capacity.assignee.clone(), capacity.capacity))
        .collect::<std::collections::BTreeMap<_, _>>();

    territories
        .iter()
        .map(|territory| {
            let summary = summarize_territory(territory);
            let capacity = territory
                .assignees
                .iter()
                .filter_map(|assignee| capacity_by_assignee.get(assignee))
                .sum::<f64>();
            CapacityException {
                territory_id: territory.id.clone(),
                demand: summary.demand,
                capacity,
                overload: (summary.demand - capacity).max(0.0),
                assignees: territory.assignees.clone(),
            }
        })
        .filter(|exception| exception.overload > 0.0)
        .collect()
}

pub fn territory_capacity(territory: &Territory, capacities: &[AssigneeCapacity]) -> f64 {
    let capacity_by_assignee = capacities
        .iter()
        .map(|capacity| (capacity.assignee.as_str(), capacity.capacity))
        .collect::<std::collections::BTreeMap<_, _>>();
    territory
        .assignees
        .iter()
        .filter_map(|assignee| capacity_by_assignee.get(assignee.as_str()))
        .sum()
}

pub fn render_territory_svg_with_capacity(
    territories: &[Territory],
    capacities: &[AssigneeCapacity],
    options: &TerritoryVisualOptions,
) -> String {
    let mut svg = render_territory_svg(territories, options);
    for territory in territories {
        let summary = summarize_territory(territory);
        let capacity = territory_capacity(territory, capacities);
        let overload = (summary.demand - capacity).max(0.0);
        let marker = format!(
            r#"<g class="territory" data-territory-id="{}""#,
            escape_attr(&territory.id)
        );
        let replacement = format!(
            r#"{marker} data-capacity="{capacity:.2}" data-overload="{overload:.2}" data-owner-count="{}""#,
            territory.assignees.len()
        );
        svg = svg.replace(&marker, &replacement);
    }
    svg
}

pub fn render_territory_svg(territories: &[Territory], options: &TerritoryVisualOptions) -> String {
    let bounds = bounds(territories);
    let palette = [
        "#2563eb", "#16a34a", "#f97316", "#7c3aed", "#dc2626", "#0891b2",
    ];
    let summaries = territories
        .iter()
        .map(summarize_territory)
        .collect::<Vec<_>>();
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" role="img" aria-labelledby="terrain-title terrain-desc">"#,
        options.width, options.height
    ));
    svg.push_str(&format!(
        "<title id=\"terrain-title\">{}</title>",
        escape_xml(&options.title)
    ));
    svg.push_str("<desc id=\"terrain-desc\">Data-bound territory split with site, demand, revenue, assignee, and centroid metadata for dashboard embedding.</desc>");
    svg.push_str(r##"<rect width="100%" height="100%" fill="#f8fafc"/>"##);
    svg.push_str(r#"<g transform="translate(28 64)">"#);
    svg.push_str(r##"<text x="0" y="-28" font-family="Inter,Segoe UI,sans-serif" font-size="28" font-weight="700" fill="#0f172a">TERRAIN split preview</text>"##);
    svg.push_str(r##"<text x="0" y="-6" font-family="Inter,Segoe UI,sans-serif" font-size="13" fill="#475569">Every territory and site carries data-* bindings for dashboard joins.</text>"##);

    for (idx, territory) in territories.iter().enumerate() {
        let color = palette[idx % palette.len()];
        let summary = &summaries[idx];
        svg.push_str(&format!(
            r#"<g class="territory" data-territory-id="{}" data-site-count="{}" data-demand="{:.2}" data-revenue="{:.2}" data-assignee-count="{}" data-assignees="{}">"#,
            escape_attr(&territory.id),
            summary.site_count,
            summary.demand,
            summary.revenue,
            summary.assignee_count,
            escape_attr(&summary.assignees.join(";"))
        ));
        for site in &territory.sites {
            let (x, y) = project(
                site.latitude,
                site.longitude,
                &bounds,
                options.width,
                options.height,
            );
            svg.push_str(&format!(
                r##"<circle class="site" data-site-id="{}" data-territory-id="{}" data-demand="{:.2}" data-revenue="{:.2}" cx="{:.1}" cy="{:.1}" r="9" fill="{}" fill-opacity="0.82" stroke="#ffffff" stroke-width="2"/>"##,
                escape_attr(&site.id),
                escape_attr(&territory.id),
                site.demand,
                site.revenue,
                x,
                y,
                color
            ));
        }
        let (cx, cy) = project(
            summary.centroid_latitude,
            summary.centroid_longitude,
            &bounds,
            options.width,
            options.height,
        );
        let label_x = (cx + 28.0).min(options.width as f64 - 216.0);
        let card_x = (cx + 14.0).min(options.width as f64 - 230.0);
        let card_y = (cy - 32.0).max(0.0);
        svg.push_str(&format!(
            r##"<rect x="{card_x:.1}" y="{card_y:.1}" width="188" height="64" rx="12" fill="#ffffff" stroke="{color}" stroke-width="2"/>"##
        ));
        svg.push_str(&format!(
            r##"<text x="{label_x:.1}" y="{:.1}" font-family="Inter,Segoe UI,sans-serif" font-size="14" font-weight="700" fill="#0f172a">{}</text>"##,
            (cy - 8.0).max(24.0),
            escape_xml(&territory.label)
        ));
        svg.push_str(&format!(
            r##"<text x="{label_x:.1}" y="{:.1}" font-family="Inter,Segoe UI,sans-serif" font-size="12" fill="#334155">{} sites - {} people - ${:.0}k</text>"##,
            (cy + 12.0).max(44.0),
            summary.site_count,
            summary.assignee_count,
            summary.revenue / 1000.0
        ));
        svg.push_str(&format!(
            r##"<text x="{label_x:.1}" y="{:.1}" font-family="Inter,Segoe UI,sans-serif" font-size="11" fill="#64748b">{}</text>"##,
            (cy + 30.0).max(62.0),
            escape_xml(&summary.assignees.join(", "))
        ));
        svg.push_str("</g>");
    }
    svg.push_str("</g></svg>");
    svg
}

pub fn render_territory_geojson(territories: &[Territory]) -> String {
    let summaries = territories
        .iter()
        .map(summarize_territory)
        .collect::<Vec<_>>();
    let mut features = Vec::new();

    for (territory, summary) in territories.iter().zip(summaries.iter()) {
        features.push(format!(
            "{{\"type\":\"Feature\",\"geometry\":{{\"type\":\"Point\",\"coordinates\":[{:.6},{:.6}]}},\"properties\":{{\"kind\":\"territory\",\"territory_id\":\"{}\",\"territory_label\":\"{}\",\"site_count\":{},\"demand\":{:.2},\"revenue\":{:.2},\"assignee_count\":{},\"assignees\":{},\"centroid_latitude\":{:.6},\"centroid_longitude\":{:.6},\"max_radius_degrees\":{:.6}}}}}",
            summary.centroid_longitude,
            summary.centroid_latitude,
            escape_json(&territory.id),
            escape_json(&territory.label),
            summary.site_count,
            summary.demand,
            summary.revenue,
            summary.assignee_count,
            json_string_array(&summary.assignees),
            summary.centroid_latitude,
            summary.centroid_longitude,
            summary.max_radius_degrees,
        ));

        for site in &territory.sites {
            features.push(format!(
                "{{\"type\":\"Feature\",\"geometry\":{{\"type\":\"Point\",\"coordinates\":[{:.6},{:.6}]}},\"properties\":{{\"kind\":\"site\",\"territory_id\":\"{}\",\"territory_label\":\"{}\",\"site_id\":\"{}\",\"demand\":{:.2},\"revenue\":{:.2},\"assignee_count\":{},\"assignees\":{}}}}}",
                site.longitude,
                site.latitude,
                escape_json(&territory.id),
                escape_json(&territory.label),
                escape_json(&site.id),
                site.demand,
                site.revenue,
                summary.assignee_count,
                json_string_array(&summary.assignees),
            ));
        }
    }

    format!(
        "{{\"type\":\"FeatureCollection\",\"name\":\"terrain-territory-split\",\"features\":[{}]}}",
        features.join(",")
    )
}

pub fn render_territory_geojson_with_capacity(
    territories: &[Territory],
    capacities: &[AssigneeCapacity],
) -> String {
    let summaries = territories
        .iter()
        .map(summarize_territory)
        .collect::<Vec<_>>();
    let mut features = Vec::new();

    for (territory, summary) in territories.iter().zip(summaries.iter()) {
        let capacity = territory_capacity(territory, capacities);
        let overload = (summary.demand - capacity).max(0.0);
        features.push(format!(
            "{{\"type\":\"Feature\",\"geometry\":{{\"type\":\"Point\",\"coordinates\":[{:.6},{:.6}]}},\"properties\":{{\"kind\":\"territory\",\"territory_id\":\"{}\",\"territory_label\":\"{}\",\"site_count\":{},\"demand\":{:.2},\"revenue\":{:.2},\"capacity\":{:.2},\"overload\":{:.2},\"owner_count\":{},\"assignee_count\":{},\"assignees\":{},\"centroid_latitude\":{:.6},\"centroid_longitude\":{:.6},\"max_radius_degrees\":{:.6}}}}}",
            summary.centroid_longitude,
            summary.centroid_latitude,
            escape_json(&territory.id),
            escape_json(&territory.label),
            summary.site_count,
            summary.demand,
            summary.revenue,
            capacity,
            overload,
            territory.assignees.len(),
            summary.assignee_count,
            json_string_array(&summary.assignees),
            summary.centroid_latitude,
            summary.centroid_longitude,
            summary.max_radius_degrees,
        ));

        for site in &territory.sites {
            features.push(format!(
                "{{\"type\":\"Feature\",\"geometry\":{{\"type\":\"Point\",\"coordinates\":[{:.6},{:.6}]}},\"properties\":{{\"kind\":\"site\",\"territory_id\":\"{}\",\"territory_label\":\"{}\",\"site_id\":\"{}\",\"demand\":{:.2},\"revenue\":{:.2},\"capacity\":{:.2},\"overload\":{:.2},\"owner_count\":{},\"assignee_count\":{},\"assignees\":{}}}}}",
                site.longitude,
                site.latitude,
                escape_json(&territory.id),
                escape_json(&territory.label),
                escape_json(&site.id),
                site.demand,
                site.revenue,
                capacity,
                overload,
                territory.assignees.len(),
                summary.assignee_count,
                json_string_array(&summary.assignees),
            ));
        }
    }

    format!(
        "{{\"type\":\"FeatureCollection\",\"name\":\"terrain-ownership-split\",\"features\":[{}]}}",
        features.join(",")
    )
}

pub fn parse_territories_csv(input: &str) -> Result<Vec<Territory>, CsvIntakeError> {
    let mut lines = input.lines().enumerate();
    let Some((header_idx, header_line)) = lines.find(|(_, line)| !line.trim().is_empty()) else {
        return Err(CsvIntakeError {
            line: 1,
            message: "missing header row".to_string(),
        });
    };
    let headers = parse_csv_line(header_line).map_err(|message| CsvIntakeError {
        line: header_idx + 1,
        message,
    })?;
    let header_map = headers
        .iter()
        .enumerate()
        .map(|(idx, header)| (header.trim().to_ascii_lowercase(), idx))
        .collect::<std::collections::BTreeMap<_, _>>();
    let required = [
        "site_id",
        "territory_id",
        "demand",
        "revenue",
        "latitude",
        "longitude",
    ];
    for header in required {
        if !header_map.contains_key(header) {
            return Err(CsvIntakeError {
                line: header_idx + 1,
                message: format!("missing required header '{header}'"),
            });
        }
    }

    let mut order = Vec::<String>::new();
    let mut territories = std::collections::BTreeMap::<String, Territory>::new();
    for (line_idx, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields = parse_csv_line(line).map_err(|message| CsvIntakeError {
            line: line_idx + 1,
            message,
        })?;
        let territory_id = csv_field(&fields, &header_map, "territory_id")
            .trim()
            .to_string();
        if territory_id.is_empty() {
            return Err(CsvIntakeError {
                line: line_idx + 1,
                message: "territory_id cannot be empty".to_string(),
            });
        }
        let site_id = csv_field(&fields, &header_map, "site_id")
            .trim()
            .to_string();
        if site_id.is_empty() {
            return Err(CsvIntakeError {
                line: line_idx + 1,
                message: "site_id cannot be empty".to_string(),
            });
        }
        let site = Site::new(
            site_id,
            parse_f64(
                csv_field(&fields, &header_map, "demand"),
                line_idx + 1,
                "demand",
            )?,
            parse_f64(
                csv_field(&fields, &header_map, "revenue"),
                line_idx + 1,
                "revenue",
            )?,
            parse_f64(
                csv_field(&fields, &header_map, "latitude"),
                line_idx + 1,
                "latitude",
            )?,
            parse_f64(
                csv_field(&fields, &header_map, "longitude"),
                line_idx + 1,
                "longitude",
            )?,
        );
        if !territories.contains_key(&territory_id) {
            order.push(territory_id.clone());
            let label = optional_csv_field(&fields, &header_map, "territory_label")
                .filter(|label| !label.trim().is_empty())
                .unwrap_or(&territory_id)
                .trim()
                .to_string();
            territories.insert(
                territory_id.clone(),
                Territory::new(&territory_id, Vec::new()).with_label(label),
            );
        }
        let territory = territories
            .get_mut(&territory_id)
            .expect("territory was inserted above");
        territory.sites.push(site);
        if let Some(assignees) = optional_csv_field(&fields, &header_map, "assignees") {
            for assignee in split_assignees(assignees) {
                if !territory.assignees.contains(&assignee) {
                    territory.assignees.push(assignee);
                }
            }
        }
    }

    Ok(order
        .into_iter()
        .filter_map(|id| territories.remove(&id))
        .collect())
}

pub fn diagnose_territories_csv(input: &str) -> Vec<CsvDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut lines = input.lines().enumerate();
    let Some((header_idx, header_line)) = lines.find(|(_, line)| !line.trim().is_empty()) else {
        diagnostics.push(CsvDiagnostic {
            severity: "error".to_string(),
            line: 1,
            field: "".to_string(),
            message: "missing header row".to_string(),
        });
        return diagnostics;
    };
    let headers = match parse_csv_line(header_line) {
        Ok(headers) => headers,
        Err(message) => {
            diagnostics.push(CsvDiagnostic {
                severity: "error".to_string(),
                line: header_idx + 1,
                field: "".to_string(),
                message,
            });
            return diagnostics;
        }
    };
    let header_map = headers
        .iter()
        .enumerate()
        .map(|(idx, header)| (header.trim().to_ascii_lowercase(), idx))
        .collect::<std::collections::BTreeMap<_, _>>();
    for header in [
        "site_id",
        "territory_id",
        "demand",
        "revenue",
        "latitude",
        "longitude",
    ] {
        if !header_map.contains_key(header) {
            diagnostics.push(CsvDiagnostic {
                severity: "error".to_string(),
                line: header_idx + 1,
                field: header.to_string(),
                message: format!("missing required header '{header}'"),
            });
        }
    }

    let mut site_lines = std::collections::BTreeMap::<String, usize>::new();
    for (line_idx, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let line_number = line_idx + 1;
        let fields = match parse_csv_line(line) {
            Ok(fields) => fields,
            Err(message) => {
                diagnostics.push(CsvDiagnostic {
                    severity: "error".to_string(),
                    line: line_number,
                    field: "".to_string(),
                    message,
                });
                continue;
            }
        };

        for header in [
            "site_id",
            "territory_id",
            "demand",
            "revenue",
            "latitude",
            "longitude",
        ] {
            if header_map.contains_key(header) && csv_field(&fields, &header_map, header).is_empty()
            {
                diagnostics.push(CsvDiagnostic {
                    severity: "error".to_string(),
                    line: line_number,
                    field: header.to_string(),
                    message: format!("field '{header}' cannot be empty"),
                });
            }
        }

        let site_id = csv_field(&fields, &header_map, "site_id").trim();
        if !site_id.is_empty() {
            if let Some(first_line) = site_lines.insert(site_id.to_string(), line_number) {
                diagnostics.push(CsvDiagnostic {
                    severity: "error".to_string(),
                    line: line_number,
                    field: "site_id".to_string(),
                    message: format!(
                        "duplicate site_id '{site_id}' first seen on line {first_line}"
                    ),
                });
            }
        }

        for header in ["demand", "revenue", "latitude", "longitude"] {
            let value = csv_field(&fields, &header_map, header).trim();
            if !value.is_empty() && value.parse::<f64>().is_err() {
                diagnostics.push(CsvDiagnostic {
                    severity: "error".to_string(),
                    line: line_number,
                    field: header.to_string(),
                    message: format!("field '{header}' must be a number"),
                });
            }
        }

        diagnose_coordinate(
            &mut diagnostics,
            &fields,
            &header_map,
            line_number,
            "latitude",
            -90.0,
            90.0,
        );
        diagnose_coordinate(
            &mut diagnostics,
            &fields,
            &header_map,
            line_number,
            "longitude",
            -180.0,
            180.0,
        );
    }
    diagnostics
}

pub fn parse_sites_csv(input: &str) -> Result<Vec<Site>, CsvIntakeError> {
    let mut lines = input.lines().enumerate();
    let Some((header_idx, header_line)) = lines.find(|(_, line)| !line.trim().is_empty()) else {
        return Err(CsvIntakeError {
            line: 1,
            message: "missing header row".to_string(),
        });
    };
    let headers = parse_csv_line(header_line).map_err(|message| CsvIntakeError {
        line: header_idx + 1,
        message,
    })?;
    let header_map = headers
        .iter()
        .enumerate()
        .map(|(idx, header)| (header.trim().to_ascii_lowercase(), idx))
        .collect::<std::collections::BTreeMap<_, _>>();
    let required = ["site_id", "demand", "revenue", "latitude", "longitude"];
    for header in required {
        if !header_map.contains_key(header) {
            return Err(CsvIntakeError {
                line: header_idx + 1,
                message: format!("missing required header '{header}'"),
            });
        }
    }

    let mut sites = Vec::new();
    for (line_idx, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields = parse_csv_line(line).map_err(|message| CsvIntakeError {
            line: line_idx + 1,
            message,
        })?;
        let site_id = csv_field(&fields, &header_map, "site_id")
            .trim()
            .to_string();
        if site_id.is_empty() {
            return Err(CsvIntakeError {
                line: line_idx + 1,
                message: "site_id cannot be empty".to_string(),
            });
        }
        sites.push(Site::new(
            site_id,
            parse_f64(
                csv_field(&fields, &header_map, "demand"),
                line_idx + 1,
                "demand",
            )?,
            parse_f64(
                csv_field(&fields, &header_map, "revenue"),
                line_idx + 1,
                "revenue",
            )?,
            parse_f64(
                csv_field(&fields, &header_map, "latitude"),
                line_idx + 1,
                "latitude",
            )?,
            parse_f64(
                csv_field(&fields, &header_map, "longitude"),
                line_idx + 1,
                "longitude",
            )?,
        ));
    }
    Ok(sites)
}

pub fn parse_assignee_capacity_csv(input: &str) -> Result<Vec<AssigneeCapacity>, CsvIntakeError> {
    let mut lines = input.lines().enumerate();
    let Some((header_idx, header_line)) = lines.find(|(_, line)| !line.trim().is_empty()) else {
        return Err(CsvIntakeError {
            line: 1,
            message: "missing header row".to_string(),
        });
    };
    let headers = parse_csv_line(header_line).map_err(|message| CsvIntakeError {
        line: header_idx + 1,
        message,
    })?;
    let header_map = headers
        .iter()
        .enumerate()
        .map(|(idx, header)| (header.trim().to_ascii_lowercase(), idx))
        .collect::<std::collections::BTreeMap<_, _>>();
    for header in ["assignee", "capacity", "home_latitude", "home_longitude"] {
        if !header_map.contains_key(header) {
            return Err(CsvIntakeError {
                line: header_idx + 1,
                message: format!("missing required header '{header}'"),
            });
        }
    }

    let mut capacities = Vec::new();
    for (line_idx, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let line_number = line_idx + 1;
        let fields = parse_csv_line(line).map_err(|message| CsvIntakeError {
            line: line_number,
            message,
        })?;
        let assignee = csv_field(&fields, &header_map, "assignee")
            .trim()
            .to_string();
        if assignee.is_empty() {
            return Err(CsvIntakeError {
                line: line_number,
                message: "assignee cannot be empty".to_string(),
            });
        }
        capacities.push(AssigneeCapacity {
            assignee,
            team: csv_field(&fields, &header_map, "team").trim().to_string(),
            capacity: parse_f64(
                csv_field(&fields, &header_map, "capacity"),
                line_number,
                "capacity",
            )?,
            home_latitude: parse_f64(
                csv_field(&fields, &header_map, "home_latitude"),
                line_number,
                "home_latitude",
            )?,
            home_longitude: parse_f64(
                csv_field(&fields, &header_map, "home_longitude"),
                line_number,
                "home_longitude",
            )?,
            skills: optional_csv_field(&fields, &header_map, "skills")
                .map(split_assignees)
                .unwrap_or_default(),
        });
    }
    Ok(capacities)
}

pub fn partition_sites(
    sites: impl IntoIterator<Item = Site>,
    target_territory_count: usize,
) -> Result<Vec<Territory>, PartitionError> {
    if target_territory_count == 0 {
        return Err(PartitionError::ZeroTerritories);
    }
    let mut sites = sites.into_iter().collect::<Vec<_>>();
    if sites.is_empty() {
        return Err(PartitionError::EmptySiteSet);
    }
    sites.sort_by(|left, right| {
        right
            .demand
            .total_cmp(&left.demand)
            .then_with(|| left.id.cmp(&right.id))
    });

    let territory_count = target_territory_count.min(sites.len());
    let mut territories = (0..territory_count)
        .map(|idx| {
            let id = format!("territory-{}", idx + 1);
            Territory::new(&id, Vec::new()).with_label(format!("Territory {}", idx + 1))
        })
        .collect::<Vec<_>>();
    let mut demand_totals = vec![0.0_f64; territory_count];
    let mut revenue_totals = vec![0.0_f64; territory_count];
    let mut site_counts = vec![0usize; territory_count];

    for site in sites {
        let target_idx = (0..territory_count)
            .min_by(|left, right| {
                demand_totals[*left]
                    .total_cmp(&demand_totals[*right])
                    .then_with(|| site_counts[*left].cmp(&site_counts[*right]))
                    .then_with(|| revenue_totals[*left].total_cmp(&revenue_totals[*right]))
                    .then_with(|| left.cmp(right))
            })
            .expect("territory_count is greater than zero");
        demand_totals[target_idx] += site.demand;
        revenue_totals[target_idx] += site.revenue;
        site_counts[target_idx] += 1;
        territories[target_idx].sites.push(site);
    }

    for territory in &mut territories {
        territory
            .sites
            .sort_by(|left, right| left.id.cmp(&right.id));
    }
    Ok(territories)
}

pub fn partition_count_sweep(
    sites: &[Site],
    min_territories: usize,
    max_territories: usize,
    max_demand_spread_ratio: f64,
    max_revenue_spread_ratio: f64,
) -> Result<Vec<PartitionSweepResult>, PartitionError> {
    if min_territories == 0 || max_territories == 0 {
        return Err(PartitionError::ZeroTerritories);
    }
    if sites.is_empty() {
        return Err(PartitionError::EmptySiteSet);
    }
    let (start, end) = if min_territories <= max_territories {
        (min_territories, max_territories)
    } else {
        (max_territories, min_territories)
    };
    let mut results = Vec::new();
    for target_territory_count in start..=end {
        let territories = partition_sites(sites.iter().cloned(), target_territory_count)?;
        let audit = audit_territories(
            &territories,
            max_demand_spread_ratio,
            max_revenue_spread_ratio,
        );
        results.push(PartitionSweepResult {
            target_territory_count,
            actual_territory_count: territories.len(),
            audit,
        });
    }
    Ok(results)
}

pub fn sample_territories_csv() -> &'static str {
    "territory_id,territory_label,assignees,site_id,demand,revenue,latitude,longitude\n\
north,North field team,Avery;Morgan;Sam,N-001,12,120000,47.62,-122.35\n\
north,North field team,Avery;Morgan;Sam,N-002,10,90000,47.67,-122.31\n\
north,North field team,Avery;Morgan;Sam,N-003,9,80000,47.58,-122.29\n\
south,South field team,Jordan;Riley,S-001,11,105000,47.50,-122.27\n\
south,South field team,Jordan;Riley,S-002,10,95000,47.46,-122.33\n\
south,South field team,Jordan;Riley,S-003,10,92000,47.53,-122.38\n"
}

pub fn sample_proposed_territories_csv() -> &'static str {
    "territory_id,territory_label,assignees,site_id,demand,revenue,latitude,longitude\n\
north,North field team,Avery;Morgan;Sam,N-001,12,120000,47.62,-122.35\n\
north,North field team,Avery;Morgan;Sam,N-002,10,90000,47.67,-122.31\n\
south,South field team,Jordan;Riley,N-003,9,80000,47.58,-122.29\n\
south,South field team,Jordan;Riley,S-001,11,105000,47.50,-122.27\n\
south,South field team,Jordan;Riley,S-002,10,95000,47.46,-122.33\n\
south,South field team,Jordan;Riley,S-003,10,92000,47.53,-122.38\n"
}

pub fn sample_sites_csv() -> &'static str {
    "site_id,demand,revenue,latitude,longitude\n\
N-001,12,120000,47.62,-122.35\n\
N-002,10,90000,47.67,-122.31\n\
N-003,9,80000,47.58,-122.29\n\
S-001,11,105000,47.50,-122.27\n\
S-002,10,95000,47.46,-122.33\n\
S-003,10,92000,47.53,-122.38\n"
}

pub fn sample_assignee_capacity_csv() -> &'static str {
    "assignee,team,capacity,home_latitude,home_longitude,skills\n\
Avery,north,14,47.61,-122.34,enterprise;onsite\n\
Morgan,north,12,47.66,-122.30,renewal\n\
Sam,north,10,47.59,-122.28,onsite\n\
Jordan,south,16,47.49,-122.29,enterprise\n\
Riley,south,14,47.52,-122.36,renewal;onsite\n"
}

pub fn sample_territories() -> Vec<Territory> {
    vec![
        Territory::new(
            "north",
            vec![
                Site::new("N-001", 12.0, 120_000.0, 47.62, -122.35),
                Site::new("N-002", 10.0, 90_000.0, 47.67, -122.31),
                Site::new("N-003", 9.0, 80_000.0, 47.58, -122.29),
            ],
        )
        .with_label("North field team")
        .with_assignees(["Avery", "Morgan", "Sam"]),
        Territory::new(
            "south",
            vec![
                Site::new("S-001", 11.0, 105_000.0, 47.50, -122.27),
                Site::new("S-002", 10.0, 95_000.0, 47.46, -122.33),
                Site::new("S-003", 10.0, 92_000.0, 47.53, -122.38),
            ],
        )
        .with_label("South field team")
        .with_assignees(["Jordan", "Riley"]),
    ]
}

fn parse_f64(value: &str, line: usize, field: &str) -> Result<f64, CsvIntakeError> {
    value.trim().parse::<f64>().map_err(|_| CsvIntakeError {
        line,
        message: format!("field '{field}' must be a number"),
    })
}

fn diagnose_coordinate(
    diagnostics: &mut Vec<CsvDiagnostic>,
    fields: &[String],
    header_map: &std::collections::BTreeMap<String, usize>,
    line: usize,
    field: &str,
    min: f64,
    max: f64,
) {
    let value = csv_field(fields, header_map, field).trim();
    let Ok(value) = value.parse::<f64>() else {
        return;
    };
    if !(min..=max).contains(&value) {
        diagnostics.push(CsvDiagnostic {
            severity: "warning".to_string(),
            line,
            field: field.to_string(),
            message: format!("field '{field}' is outside expected range {min}..{max}"),
        });
    }
}

fn territory_site_index(
    territories: &[Territory],
) -> std::collections::BTreeMap<String, (String, &Site)> {
    let mut index = std::collections::BTreeMap::new();
    for territory in territories {
        for site in &territory.sites {
            index.insert(site.id.clone(), (territory.id.clone(), site));
        }
    }
    index
}

fn csv_field<'a>(
    fields: &'a [String],
    header_map: &std::collections::BTreeMap<String, usize>,
    header: &str,
) -> &'a str {
    optional_csv_field(fields, header_map, header).unwrap_or("")
}

fn optional_csv_field<'a>(
    fields: &'a [String],
    header_map: &std::collections::BTreeMap<String, usize>,
    header: &str,
) -> Option<&'a str> {
    header_map
        .get(header)
        .and_then(|idx| fields.get(*idx))
        .map(String::as_str)
}

fn split_assignees(value: &str) -> Vec<String> {
    value
        .split([';', '|'])
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn parse_csv_line(line: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(field.trim().to_string());
                field.clear();
            }
            _ => field.push(ch),
        }
    }
    if quoted {
        return Err("unterminated quoted field".to_string());
    }
    fields.push(field.trim().to_string());
    Ok(fields)
}

fn mean(count: usize, values: impl Iterator<Item = f64>) -> f64 {
    if count == 0 {
        return 0.0;
    }
    values.sum::<f64>() / count as f64
}

fn spread_ratio(values: impl Iterator<Item = f64>) -> f64 {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for value in values {
        min = min.min(value);
        max = max.max(value);
    }
    if !min.is_finite() || min == 0.0 {
        return 0.0;
    }
    (max - min) / min
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
}

fn bounds(territories: &[Territory]) -> Bounds {
    let mut bounds = Bounds {
        min_lat: f64::INFINITY,
        max_lat: f64::NEG_INFINITY,
        min_lon: f64::INFINITY,
        max_lon: f64::NEG_INFINITY,
    };
    for site in territories.iter().flat_map(|territory| &territory.sites) {
        bounds.min_lat = bounds.min_lat.min(site.latitude);
        bounds.max_lat = bounds.max_lat.max(site.latitude);
        bounds.min_lon = bounds.min_lon.min(site.longitude);
        bounds.max_lon = bounds.max_lon.max(site.longitude);
    }
    if !bounds.min_lat.is_finite() {
        return Bounds {
            min_lat: 0.0,
            max_lat: 1.0,
            min_lon: 0.0,
            max_lon: 1.0,
        };
    }
    bounds
}

fn project(lat: f64, lon: f64, bounds: &Bounds, width: u32, height: u32) -> (f64, f64) {
    let pad = 56.0;
    let span_lat = (bounds.max_lat - bounds.min_lat).max(0.000_001);
    let span_lon = (bounds.max_lon - bounds.min_lon).max(0.000_001);
    let x = pad + ((lon - bounds.min_lon) / span_lon) * (width as f64 - pad * 2.0);
    let y = pad + ((bounds.max_lat - lat) / span_lat) * (height as f64 - pad * 2.0 - 64.0);
    (x, y)
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(value: &str) -> String {
    escape_xml(value).replace('"', "&quot;")
}

fn escape_json(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => escaped.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => escaped.push(ch),
        }
    }
    escaped
}

fn json_string_array(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", escape_json(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(left: f64, right: f64) {
        assert!((left - right).abs() < 0.000_001, "{left} != {right}");
    }

    #[test]
    fn summarizes_territory_totals_and_centroid() {
        let territory = Territory::new(
            "west",
            vec![
                Site::new("a", 3.0, 30.0, 1.0, 2.0),
                Site::new("b", 5.0, 70.0, 3.0, 6.0),
            ],
        );

        let summary = summarize_territory(&territory);

        assert_eq!(summary.territory_id, "west");
        assert_eq!(summary.site_count, 2);
        near(summary.demand, 8.0);
        near(summary.revenue, 100.0);
        assert_eq!(summary.assignee_count, 0);
        near(summary.centroid_latitude, 2.0);
        near(summary.centroid_longitude, 4.0);
    }

    #[test]
    fn audits_balance_spread() {
        let audit = audit_territories(&sample_territories(), 0.05, 0.05);

        assert_eq!(audit.summaries.len(), 2);
        near(audit.demand_spread_ratio, 0.0);
        assert!(audit.revenue_spread_ratio < 0.01);
        assert!(audit.passes);
    }

    #[test]
    fn renders_data_bound_svg_for_dashboards() {
        let svg = render_territory_svg(&sample_territories(), &TerritoryVisualOptions::default());

        assert!(svg.contains("data-territory-id=\"north\""));
        assert!(svg.contains("data-site-count=\"3\""));
        assert!(svg.contains("data-assignee-count=\"3\""));
        assert!(svg.contains("data-site-id=\"N-001\""));
    }

    #[test]
    fn renders_data_bound_geojson_for_dashboards() {
        let geojson = render_territory_geojson(&sample_territories());

        assert!(geojson.contains("\"type\":\"FeatureCollection\""));
        assert!(geojson.contains("\"kind\":\"territory\""));
        assert!(geojson.contains("\"territory_id\":\"north\""));
        assert!(geojson.contains("\"site_id\":\"N-001\""));
        assert!(geojson.contains("\"assignees\":[\"Avery\",\"Morgan\",\"Sam\"]"));
    }

    #[test]
    fn compares_baseline_and_proposed_plans() {
        let baseline = parse_territories_csv(sample_territories_csv()).expect("baseline parses");
        let proposed =
            parse_territories_csv(sample_proposed_territories_csv()).expect("proposed parses");
        let comparison = compare_territory_plans(&baseline, &proposed, 0.05, 0.05);

        assert!(comparison.baseline.passes);
        assert!(!comparison.proposed.passes);
        assert_eq!(comparison.territory_deltas.len(), 2);
        assert_eq!(comparison.territory_deltas[0].territory_id, "north");
        assert_eq!(comparison.territory_deltas[0].site_count_delta, -1);
        near(comparison.territory_deltas[1].demand_delta, 9.0);
    }

    #[test]
    fn reports_compactness_exceptions() {
        let exceptions = compactness_exceptions(&sample_territories(), 0.06);

        assert_eq!(exceptions.len(), 1);
        assert_eq!(exceptions[0].territory_id, "south");
        assert_eq!(exceptions[0].site_count, 3);
        assert!(exceptions[0].max_radius_degrees > exceptions[0].threshold_degrees);
    }

    #[test]
    fn reports_site_movements_between_plans() {
        let baseline = parse_territories_csv(sample_territories_csv()).expect("baseline parses");
        let proposed =
            parse_territories_csv(sample_proposed_territories_csv()).expect("proposed parses");
        let movements = site_movements(&baseline, &proposed);

        assert_eq!(movements.len(), 6);
        let moved = movements
            .iter()
            .find(|movement| movement.site_id == "N-003")
            .expect("N-003 movement exists");
        assert_eq!(moved.baseline_territory_id.as_deref(), Some("north"));
        assert_eq!(moved.proposed_territory_id.as_deref(), Some("south"));
        assert_eq!(moved.movement_kind, "moved");
        near(moved.demand, 9.0);
    }

    #[test]
    fn sweeps_partition_counts_deterministically() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let sweep = partition_count_sweep(&sites, 2, 3, 0.50, 0.50).expect("sweep works");

        assert_eq!(sweep.len(), 2);
        assert_eq!(sweep[0].target_territory_count, 2);
        assert_eq!(sweep[0].actual_territory_count, 2);
        assert!(sweep[0].audit.passes);
        assert_eq!(sweep[1].target_territory_count, 3);
        assert_eq!(sweep[1].actual_territory_count, 3);
    }

    #[test]
    fn parses_assignee_capacity_csv() {
        let capacities =
            parse_assignee_capacity_csv(sample_assignee_capacity_csv()).expect("capacity parses");

        assert_eq!(capacities.len(), 5);
        assert_eq!(capacities[0].assignee, "Avery");
        assert_eq!(capacities[0].team, "north");
        near(capacities[0].capacity, 14.0);
        assert_eq!(capacities[0].skills, ["enterprise", "onsite"]);
    }

    #[test]
    fn flags_capacity_overloads() {
        let territories =
            parse_territories_csv(sample_territories_csv()).expect("territories parse");
        let capacities =
            parse_assignee_capacity_csv(sample_assignee_capacity_csv()).expect("capacity parses");
        let exceptions = capacity_exceptions(&territories, &capacities);

        assert_eq!(exceptions.len(), 1);
        assert_eq!(exceptions[0].territory_id, "south");
        near(exceptions[0].demand, 31.0);
        near(exceptions[0].capacity, 30.0);
        near(exceptions[0].overload, 1.0);
    }

    #[test]
    fn renders_ownership_visual_bindings() {
        let territories =
            parse_territories_csv(sample_territories_csv()).expect("territories parse");
        let capacities =
            parse_assignee_capacity_csv(sample_assignee_capacity_csv()).expect("capacity parses");

        let svg = render_territory_svg_with_capacity(
            &territories,
            &capacities,
            &TerritoryVisualOptions::default(),
        );
        let geojson = render_territory_geojson_with_capacity(&territories, &capacities);

        assert!(svg.contains("data-capacity=\"30.00\""));
        assert!(svg.contains("data-overload=\"1.00\""));
        assert!(geojson.contains("\"capacity\":30.00"));
        assert!(geojson.contains("\"overload\":1.00"));
        assert!(geojson.contains("\"owner_count\":2"));
    }

    #[test]
    fn diagnoses_messy_territory_csv_with_row_context() {
        let diagnostics = diagnose_territories_csv(
            "territory_id,site_id,demand,revenue,latitude,longitude\n\
north,N-001,12,120000,47.62,-122.35\n\
north,N-001,nope,90000,147.67,-122.31\n\
south,,10,95000,47.46,-222.33\n",
        );

        assert_eq!(diagnostics.len(), 5);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.line == 3
                && diagnostic.field == "site_id"
                && diagnostic.message.contains("duplicate")
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.line == 3
                && diagnostic.field == "demand"
                && diagnostic.message.contains("number")
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.line == 4
                && diagnostic.field == "longitude"
                && diagnostic.severity == "warning"
        }));
    }

    #[test]
    fn parses_csv_into_territories_with_assignees() {
        let territories = parse_territories_csv(sample_territories_csv()).expect("sample parses");

        assert_eq!(territories.len(), 2);
        assert_eq!(territories[0].id, "north");
        assert_eq!(territories[0].label, "North field team");
        assert_eq!(territories[0].sites.len(), 3);
        assert_eq!(territories[0].assignees, ["Avery", "Morgan", "Sam"]);
        assert_eq!(territories[1].assignees, ["Jordan", "Riley"]);
    }

    #[test]
    fn csv_reports_line_numbers_for_bad_numbers() {
        let error = parse_territories_csv(
            "territory_id,site_id,demand,revenue,latitude,longitude\nnorth,N-001,nope,1,2,3\n",
        )
        .expect_err("bad demand should fail");

        assert_eq!(error.line, 2);
        assert!(error.message.contains("demand"));
    }

    #[test]
    fn parses_unassigned_site_csv() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");

        assert_eq!(sites.len(), 6);
        assert_eq!(sites[0].id, "N-001");
        near(sites.iter().map(|site| site.demand).sum::<f64>(), 62.0);
    }

    #[test]
    fn partitions_sites_by_demand_deterministically() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let territories = partition_sites(sites, 2).expect("sites partition");
        let audit = audit_territories(&territories, 0.05, 0.05);

        assert_eq!(territories.len(), 2);
        assert_eq!(territories[0].id, "territory-1");
        assert_eq!(territories[1].id, "territory-2");
        near(audit.summaries[0].demand, 31.0);
        near(audit.summaries[1].demand, 31.0);
        assert!(audit.passes);
    }
}
