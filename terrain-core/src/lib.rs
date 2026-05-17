#[derive(Debug, Clone, PartialEq)]
pub struct Site {
    pub id: String,
    pub demand: f64,
    pub revenue: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub product_demands: Vec<ProductDemand>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductDemand {
    pub product: String,
    pub demand: f64,
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
pub struct ProductDemandBalance {
    pub product: String,
    pub min_demand: f64,
    pub max_demand: f64,
    pub spread_ratio: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductBalanceAudit {
    pub balances: Vec<ProductDemandBalance>,
    pub overall_spread_score: f64,
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
pub struct TerritoryEdgeAuditDiagnostic {
    pub severity: String,
    pub code: String,
    pub territory_id: Option<String>,
    pub from_site_id: Option<String>,
    pub to_site_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerritoryEdgeAuditReport {
    pub territory_count: usize,
    pub site_count: usize,
    pub edge_count: usize,
    pub cut_edge_count: usize,
    pub disconnected_territory_count: usize,
    pub diagnostics: Vec<TerritoryEdgeAuditDiagnostic>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssigneeCapacity {
    pub assignee: String,
    pub team: String,
    pub responsibility: String,
    pub asset_group: String,
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
pub struct GraphPartitionReport {
    pub baseline: Vec<Territory>,
    pub graph_partition: Vec<Territory>,
    pub comparison: ScenarioComparison,
    pub movements: Vec<SiteMovement>,
    pub compactness_exceptions: Vec<CompactnessException>,
    pub graph_diagnostics: SiteGraphDiagnosticReport,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetisCoreHandoff {
    pub vertex_site_ids: Vec<String>,
    pub xadj: Vec<u32>,
    pub adjncy: Vec<u32>,
    pub vwgt: Vec<i32>,
    pub adjwgt: Vec<i32>,
    pub edge_weight_scale: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiteGraphNode {
    pub site_id: String,
    pub demand: f64,
    pub revenue: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiteGraphEdge {
    pub from_site_id: String,
    pub to_site_id: String,
    pub weight: f64,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiteGraphEdgeInput {
    pub from_site_id: String,
    pub to_site_id: String,
    pub weight: f64,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiteGraphDiagnostic {
    pub severity: String,
    pub code: String,
    pub site_ids: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiteGraph {
    pub nodes: Vec<SiteGraphNode>,
    pub edges: Vec<SiteGraphEdge>,
    pub diagnostics: Vec<SiteGraphDiagnostic>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiteGraphDiagnosticReport {
    pub node_count: usize,
    pub edge_count: usize,
    pub component_count: usize,
    pub diagnostics: Vec<SiteGraphDiagnostic>,
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
    MetisCore(String),
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
            Self::MetisCore(message) => write!(f, "METIS-CORE partition error: {message}"),
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
            product_demands: Vec::new(),
        }
    }

    pub fn with_product_demands(
        mut self,
        product_demands: impl IntoIterator<Item = ProductDemand>,
    ) -> Self {
        self.product_demands = product_demands.into_iter().collect();
        self
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

pub fn dashboard_schema_json() -> &'static str {
    r#"{"schema_id":"terrain.dashboard.v1","exports":[{"name":"territory","fields":["territory_id","territory_label","site_count","demand","revenue","capacity","overload","owner_count","assignee_count","assignees","centroid_latitude","centroid_longitude","max_radius_degrees"]},{"name":"site","fields":["site_id","territory_id","territory_label","demand","revenue","capacity","overload","owner_count","assignee_count","assignees","latitude","longitude","product_*"]},{"name":"product_balance","fields":["product","min_demand","max_demand","spread_ratio"]},{"name":"scenario_delta","fields":["territory_id","baseline_site_count","proposed_site_count","site_count_delta","baseline_demand","proposed_demand","demand_delta","baseline_revenue","proposed_revenue","revenue_delta"]},{"name":"movement","fields":["site_id","baseline_territory_id","proposed_territory_id","movement_kind","demand","revenue"]},{"name":"capacity_exception","fields":["territory_id","demand","capacity","overload","assignees"]}]}"#
}

pub fn integration_fixture_manifest_json() -> &'static str {
    r#"{"manifest_id":"terrain.integration-fixtures.v1","sources":[{"name":"crop-geography-cache","repo":"CROP","role":"cached public geography and boundaries","status":"candidate"},{"name":"pebble-context-packets","repo":"PEBBLE","role":"portable context and benchmark packets","status":"candidate"},{"name":"fletch-fetch-cache","repo":"FLETCH","role":"registered URL/cacheline-backed fixture retrieval","status":"candidate"}],"fixtures":[{"name":"sample-territories","path":"fixtures/sample-territories.csv","schema":"terrain.dashboard.v1","role":"assigned territory baseline"},{"name":"sample-sites","path":"fixtures/sample-sites.csv","schema":"terrain.dashboard.v1","role":"unassigned partition input"},{"name":"sample-capacity","path":"fixtures/sample-capacity.csv","schema":"terrain.dashboard.v1","role":"assignee capacity input"},{"name":"steady-state-scenario","path":"fixtures/scenarios/steady-state-territories.csv","schema":"terrain.dashboard.v1","role":"balanced multi-product territory scenario"},{"name":"risky-reassignment-scenario","path":"fixtures/scenarios/risky-reassignment-territories.csv","schema":"terrain.dashboard.v1","role":"movement, product imbalance, and capacity overload scenario"},{"name":"growth-sites-scenario","path":"fixtures/scenarios/growth-sites.csv","schema":"terrain.dashboard.v1","role":"multi-product count sweep input"}]}"#
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

pub fn audit_product_balance(
    territories: &[Territory],
    max_product_spread_ratio: f64,
) -> ProductBalanceAudit {
    let products = territories
        .iter()
        .flat_map(|territory| &territory.sites)
        .flat_map(|site| {
            site.product_demands
                .iter()
                .map(|demand| demand.product.clone())
        })
        .collect::<std::collections::BTreeSet<_>>();
    let balances = products
        .into_iter()
        .map(|product| {
            let totals = territories
                .iter()
                .map(|territory| {
                    territory
                        .sites
                        .iter()
                        .flat_map(|site| &site.product_demands)
                        .filter(|demand| demand.product == product)
                        .map(|demand| demand.demand)
                        .sum::<f64>()
                })
                .collect::<Vec<_>>();
            let min_demand = totals.iter().copied().fold(f64::INFINITY, f64::min);
            let max_demand = totals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            ProductDemandBalance {
                product,
                min_demand: if min_demand.is_finite() {
                    min_demand
                } else {
                    0.0
                },
                max_demand: if max_demand.is_finite() {
                    max_demand
                } else {
                    0.0
                },
                spread_ratio: spread_ratio(totals.into_iter()),
            }
        })
        .collect::<Vec<_>>();
    let overall_spread_score = if balances.is_empty() {
        0.0
    } else {
        balances
            .iter()
            .map(|balance| balance.spread_ratio)
            .sum::<f64>()
            / balances.len() as f64
    };
    let passes = balances
        .iter()
        .all(|balance| balance.spread_ratio <= max_product_spread_ratio);
    ProductBalanceAudit {
        balances,
        overall_spread_score,
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

pub fn territory_edge_audit(
    territories: &[Territory],
    edge_inputs: &[SiteGraphEdgeInput],
) -> TerritoryEdgeAuditReport {
    let site_index = territory_site_index(territories);
    let mut diagnostics = Vec::new();
    let mut cut_edge_count = 0usize;
    let mut internal_edges_by_territory =
        std::collections::BTreeMap::<String, Vec<(String, String)>>::new();

    for edge in edge_inputs {
        let from = site_index.get(&edge.from_site_id);
        let to = site_index.get(&edge.to_site_id);
        match (from, to) {
            (Some((from_territory, _)), Some((to_territory, _)))
                if from_territory == to_territory =>
            {
                let (left, right) = ordered_site_pair(&edge.from_site_id, &edge.to_site_id);
                internal_edges_by_territory
                    .entry(from_territory.clone())
                    .or_default()
                    .push((left, right));
            }
            (Some((from_territory, _)), Some((to_territory, _))) => {
                cut_edge_count += 1;
                diagnostics.push(TerritoryEdgeAuditDiagnostic {
                    severity: "warning".to_string(),
                    code: "cut-edge".to_string(),
                    territory_id: None,
                    from_site_id: Some(edge.from_site_id.clone()),
                    to_site_id: Some(edge.to_site_id.clone()),
                    message: format!(
                        "edge connects territory '{}' to '{}'",
                        from_territory, to_territory
                    ),
                });
            }
            _ => {
                diagnostics.push(TerritoryEdgeAuditDiagnostic {
                    severity: "error".to_string(),
                    code: "unknown-edge-site".to_string(),
                    territory_id: None,
                    from_site_id: Some(edge.from_site_id.clone()),
                    to_site_id: Some(edge.to_site_id.clone()),
                    message: format!(
                        "edge references site '{}' or '{}' outside the territory plan",
                        edge.from_site_id, edge.to_site_id
                    ),
                });
            }
        }
    }

    let mut disconnected_territory_count = 0usize;
    for territory in territories {
        if territory.sites.len() <= 1 {
            continue;
        }
        let components = territory_edge_components(
            &territory.sites,
            internal_edges_by_territory
                .get(&territory.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        );
        if components.len() > 1 {
            disconnected_territory_count += 1;
            diagnostics.push(TerritoryEdgeAuditDiagnostic {
                severity: "warning".to_string(),
                code: "disconnected-territory".to_string(),
                territory_id: Some(territory.id.clone()),
                from_site_id: None,
                to_site_id: None,
                message: format!(
                    "territory '{}' has {} edge-evidence component(s)",
                    territory.id,
                    components.len()
                ),
            });
        }
    }

    diagnostics.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.territory_id.cmp(&right.territory_id))
            .then_with(|| left.from_site_id.cmp(&right.from_site_id))
            .then_with(|| left.to_site_id.cmp(&right.to_site_id))
    });

    TerritoryEdgeAuditReport {
        territory_count: territories.len(),
        site_count: site_index.len(),
        edge_count: edge_inputs.len(),
        cut_edge_count,
        disconnected_territory_count,
        diagnostics,
    }
}

pub fn capacity_exceptions(
    territories: &[Territory],
    capacities: &[AssigneeCapacity],
) -> Vec<CapacityException> {
    let capacity_by_assignee = assignee_capacity_totals(capacities);

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
    let capacity_by_assignee = assignee_capacity_totals(capacities);
    territory
        .assignees
        .iter()
        .filter_map(|assignee| capacity_by_assignee.get(assignee))
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

pub fn render_site_graph_svg(
    sites: &[Site],
    edge_inputs: &[SiteGraphEdgeInput],
    options: &TerritoryVisualOptions,
) -> String {
    let graph = build_site_graph_with_edges(sites, edge_inputs);
    let site_by_id = sites
        .iter()
        .map(|site| (site.id.clone(), site))
        .collect::<std::collections::BTreeMap<_, _>>();
    let bounds_source = vec![Territory::new("edge-evidence", sites.to_vec())];
    let bounds = bounds(&bounds_source);
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" role="img" aria-labelledby="terrain-title terrain-desc">"#,
        options.width, options.height
    ));
    svg.push_str(&format!(
        "<title id=\"terrain-title\">{}</title>",
        escape_xml(&options.title)
    ));
    svg.push_str("<desc id=\"terrain-desc\">Data-bound site graph edge evidence with stable site IDs, evidence labels, and weights.</desc>");
    svg.push_str(r##"<rect width="100%" height="100%" fill="#f8fafc"/>"##);
    svg.push_str(r#"<g transform="translate(28 64)">"#);
    svg.push_str(r##"<text x="0" y="-28" font-family="Inter,Segoe UI,sans-serif" font-size="28" font-weight="700" fill="#0f172a">TERRAIN edge evidence</text>"##);
    svg.push_str(r##"<text x="0" y="-6" font-family="Inter,Segoe UI,sans-serif" font-size="13" fill="#475569">Lines carry data bindings for source site, target site, evidence, and weight.</text>"##);

    for edge in &graph.edges {
        let Some(from_site) = site_by_id.get(&edge.from_site_id) else {
            continue;
        };
        let Some(to_site) = site_by_id.get(&edge.to_site_id) else {
            continue;
        };
        let (x1, y1) = project(
            from_site.latitude,
            from_site.longitude,
            &bounds,
            options.width,
            options.height,
        );
        let (x2, y2) = project(
            to_site.latitude,
            to_site.longitude,
            &bounds,
            options.width,
            options.height,
        );
        svg.push_str(&format!(
            r##"<line class="site-edge" data-from-site-id="{}" data-to-site-id="{}" data-evidence="{}" data-weight="{:.6}" x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="#0f766e" stroke-width="3" stroke-opacity="0.58" stroke-linecap="round"/>"##,
            escape_attr(&edge.from_site_id),
            escape_attr(&edge.to_site_id),
            escape_attr(&edge.evidence),
            edge.weight,
        ));
    }

    for site in sites {
        let (x, y) = project(
            site.latitude,
            site.longitude,
            &bounds,
            options.width,
            options.height,
        );
        svg.push_str(&format!(
            r##"<circle class="site" data-site-id="{}" data-demand="{:.2}" data-revenue="{:.2}" cx="{x:.1}" cy="{y:.1}" r="9" fill="#2563eb" fill-opacity="0.88" stroke="#ffffff" stroke-width="2"/>"##,
            escape_attr(&site.id),
            site.demand,
            site.revenue,
        ));
        svg.push_str(&format!(
            r##"<text x="{:.1}" y="{:.1}" font-family="Inter,Segoe UI,sans-serif" font-size="11" fill="#0f172a">{}</text>"##,
            x + 12.0,
            y + 4.0,
            escape_xml(&site.id),
        ));
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

pub fn render_site_graph_geojson(sites: &[Site], edge_inputs: &[SiteGraphEdgeInput]) -> String {
    let graph = build_site_graph_with_edges(sites, edge_inputs);
    let site_by_id = sites
        .iter()
        .map(|site| (site.id.clone(), site))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut features = Vec::new();

    for edge in &graph.edges {
        let Some(from_site) = site_by_id.get(&edge.from_site_id) else {
            continue;
        };
        let Some(to_site) = site_by_id.get(&edge.to_site_id) else {
            continue;
        };
        features.push(format!(
            "{{\"type\":\"Feature\",\"geometry\":{{\"type\":\"LineString\",\"coordinates\":[[{:.6},{:.6}],[{:.6},{:.6}]]}},\"properties\":{{\"kind\":\"site_edge\",\"from_site_id\":\"{}\",\"to_site_id\":\"{}\",\"evidence\":\"{}\",\"weight\":{:.6}}}}}",
            from_site.longitude,
            from_site.latitude,
            to_site.longitude,
            to_site.latitude,
            escape_json(&edge.from_site_id),
            escape_json(&edge.to_site_id),
            escape_json(&edge.evidence),
            edge.weight,
        ));
    }

    for site in sites {
        features.push(format!(
            "{{\"type\":\"Feature\",\"geometry\":{{\"type\":\"Point\",\"coordinates\":[{:.6},{:.6}]}},\"properties\":{{\"kind\":\"site\",\"site_id\":\"{}\",\"demand\":{:.2},\"revenue\":{:.2}}}}}",
            site.longitude,
            site.latitude,
            escape_json(&site.id),
            site.demand,
            site.revenue,
        ));
    }

    format!(
        "{{\"type\":\"FeatureCollection\",\"name\":\"terrain-edge-evidence\",\"features\":[{}]}}",
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
        )
        .with_product_demands(extract_product_demands(&fields, &header_map, line_idx + 1)?);
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
        sites.push(
            Site::new(
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
            )
            .with_product_demands(extract_product_demands(
                &fields,
                &header_map,
                line_idx + 1,
            )?),
        );
    }
    Ok(sites)
}

pub fn parse_site_edges_csv(input: &str) -> Result<Vec<SiteGraphEdgeInput>, CsvIntakeError> {
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
    for header in ["from_site_id", "to_site_id", "weight", "evidence"] {
        if !header_map.contains_key(header) {
            return Err(CsvIntakeError {
                line: header_idx + 1,
                message: format!("missing required header '{header}'"),
            });
        }
    }

    let mut edges = Vec::new();
    for (line_idx, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields = parse_csv_line(line).map_err(|message| CsvIntakeError {
            line: line_idx + 1,
            message,
        })?;
        let from_site_id = csv_field(&fields, &header_map, "from_site_id")
            .trim()
            .to_string();
        let to_site_id = csv_field(&fields, &header_map, "to_site_id")
            .trim()
            .to_string();
        let evidence = csv_field(&fields, &header_map, "evidence")
            .trim()
            .to_string();
        if from_site_id.is_empty() {
            return Err(CsvIntakeError {
                line: line_idx + 1,
                message: "from_site_id cannot be empty".to_string(),
            });
        }
        if to_site_id.is_empty() {
            return Err(CsvIntakeError {
                line: line_idx + 1,
                message: "to_site_id cannot be empty".to_string(),
            });
        }
        if evidence.is_empty() {
            return Err(CsvIntakeError {
                line: line_idx + 1,
                message: "evidence cannot be empty".to_string(),
            });
        }
        let weight = parse_f64(
            csv_field(&fields, &header_map, "weight"),
            line_idx + 1,
            "weight",
        )?;
        if weight <= 0.0 {
            return Err(CsvIntakeError {
                line: line_idx + 1,
                message: "weight must be greater than zero".to_string(),
            });
        }
        edges.push(SiteGraphEdgeInput {
            from_site_id,
            to_site_id,
            weight,
            evidence,
        });
    }
    Ok(edges)
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
            responsibility: optional_csv_field(&fields, &header_map, "responsibility")
                .unwrap_or("territory")
                .trim()
                .to_string(),
            asset_group: optional_csv_field(&fields, &header_map, "asset_group")
                .unwrap_or("all")
                .trim()
                .to_string(),
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

pub fn partition_sites_by_graph(
    sites: &[Site],
    target_territory_count: usize,
) -> Result<Vec<Territory>, PartitionError> {
    if target_territory_count == 0 {
        return Err(PartitionError::ZeroTerritories);
    }
    if sites.is_empty() {
        return Err(PartitionError::EmptySiteSet);
    }

    let graph = build_site_graph(sites);
    let site_by_id = sites
        .iter()
        .map(|site| (site.id.clone(), site.clone()))
        .collect::<std::collections::BTreeMap<_, _>>();
    let territory_count = target_territory_count.min(site_by_id.len());
    let seeds = graph_partition_seeds(&graph, territory_count);
    let mut territories = (0..territory_count)
        .map(|idx| {
            let id = format!("territory-{}", idx + 1);
            Territory::new(&id, Vec::new()).with_label(format!("Graph territory {}", idx + 1))
        })
        .collect::<Vec<_>>();
    let mut assigned = std::collections::BTreeSet::new();
    let mut demand_totals = vec![0.0_f64; territory_count];
    let mut site_counts = vec![0usize; territory_count];

    for (idx, seed_id) in seeds.iter().enumerate() {
        if let Some(site) = site_by_id.get(seed_id) {
            demand_totals[idx] += site.demand;
            site_counts[idx] += 1;
            territories[idx].sites.push(site.clone());
            assigned.insert(seed_id.clone());
        }
    }

    for site_id in site_by_id
        .keys()
        .filter(|site_id| !assigned.contains(*site_id))
    {
        let target_idx = seeds
            .iter()
            .enumerate()
            .min_by(|(left_idx, left_seed), (right_idx, right_seed)| {
                graph_edge_weight(&graph, site_id, left_seed)
                    .total_cmp(&graph_edge_weight(&graph, site_id, right_seed))
                    .then_with(|| demand_totals[*left_idx].total_cmp(&demand_totals[*right_idx]))
                    .then_with(|| site_counts[*left_idx].cmp(&site_counts[*right_idx]))
                    .then_with(|| left_idx.cmp(right_idx))
            })
            .map(|(idx, _)| idx)
            .expect("territory_count is greater than zero");
        let site = site_by_id
            .get(site_id)
            .expect("site id came from site index")
            .clone();
        demand_totals[target_idx] += site.demand;
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

pub fn graph_partition_report(
    sites: &[Site],
    target_territory_count: usize,
    long_edge_threshold_degrees: f64,
    compactness_threshold_degrees: f64,
) -> Result<GraphPartitionReport, PartitionError> {
    let baseline = partition_sites(sites.iter().cloned(), target_territory_count)?;
    let graph_partition = partition_sites_by_graph(sites, target_territory_count)?;
    let comparison = compare_territory_plans(&baseline, &graph_partition, 0.50, 0.50);
    let movements = site_movements(&baseline, &graph_partition);
    let compactness_exceptions =
        compactness_exceptions(&graph_partition, compactness_threshold_degrees);
    let graph_diagnostics = site_graph_diagnostic_report(sites, long_edge_threshold_degrees);

    Ok(GraphPartitionReport {
        baseline,
        graph_partition,
        comparison,
        movements,
        compactness_exceptions,
        graph_diagnostics,
    })
}

pub fn partition_sites_with_metis_core(
    sites: &[Site],
    target_territory_count: usize,
    seed: u64,
) -> Result<Vec<Territory>, PartitionError> {
    if target_territory_count == 0 {
        return Err(PartitionError::ZeroTerritories);
    }
    if sites.is_empty() {
        return Err(PartitionError::EmptySiteSet);
    }

    metis_core_partition_from_handoff(
        sites,
        &metis_core_handoff(sites)?,
        target_territory_count,
        seed,
    )
}

pub fn partition_sites_with_metis_core_edges(
    sites: &[Site],
    edge_inputs: &[SiteGraphEdgeInput],
    target_territory_count: usize,
    seed: u64,
) -> Result<Vec<Territory>, PartitionError> {
    if target_territory_count == 0 {
        return Err(PartitionError::ZeroTerritories);
    }
    if sites.is_empty() {
        return Err(PartitionError::EmptySiteSet);
    }

    metis_core_partition_from_handoff(
        sites,
        &metis_core_handoff_with_edges(sites, edge_inputs)?,
        target_territory_count,
        seed,
    )
}

fn metis_core_partition_from_handoff(
    sites: &[Site],
    handoff: &MetisCoreHandoff,
    target_territory_count: usize,
    seed: u64,
) -> Result<Vec<Territory>, PartitionError> {
    let territory_count = target_territory_count.min(handoff.vertex_site_ids.len());
    let graph = metis_core::CsrGraph::from_csr(
        &handoff.xadj,
        &handoff.adjncy,
        &handoff.vwgt,
        &handoff.adjwgt,
    )
    .map_err(|error| PartitionError::MetisCore(error.to_string()))?;
    let params = metis_core::MetisParams::kway().with_seed(seed);
    let partitioner = metis_core::MetisPartitioner::from_params(params);
    let partition =
        metis_core::Partitioner::split(&partitioner, &graph, territory_count as u32, Some(seed))
            .map_err(|error| PartitionError::MetisCore(error.to_string()))?;
    partition
        .validate_for_graph(&graph)
        .map_err(|error| PartitionError::MetisCore(error.to_string()))?;

    let site_by_id = sites
        .iter()
        .map(|site| (site.id.clone(), site.clone()))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut territories = (0..territory_count)
        .map(|idx| {
            let id = format!("territory-{}", idx + 1);
            Territory::new(&id, Vec::new()).with_label(format!("METIS territory {}", idx + 1))
        })
        .collect::<Vec<_>>();

    for (vertex_idx, part) in partition.assignment().iter().enumerate() {
        let site_id = &handoff.vertex_site_ids[vertex_idx];
        if let Some(site) = site_by_id.get(site_id) {
            territories[*part as usize].sites.push(site.clone());
        }
    }

    for territory in &mut territories {
        territory
            .sites
            .sort_by(|left, right| left.id.cmp(&right.id));
    }

    Ok(territories)
}

pub fn metis_core_handoff(sites: &[Site]) -> Result<MetisCoreHandoff, PartitionError> {
    if sites.is_empty() {
        return Err(PartitionError::EmptySiteSet);
    }

    metis_core_handoff_from_graph(&build_site_graph(sites))
}

pub fn metis_core_handoff_with_edges(
    sites: &[Site],
    edge_inputs: &[SiteGraphEdgeInput],
) -> Result<MetisCoreHandoff, PartitionError> {
    if sites.is_empty() {
        return Err(PartitionError::EmptySiteSet);
    }

    metis_core_handoff_from_graph(&build_site_graph_with_edges(sites, edge_inputs))
}

fn metis_core_handoff_from_graph(graph: &SiteGraph) -> Result<MetisCoreHandoff, PartitionError> {
    let vertex_site_ids = graph
        .nodes
        .iter()
        .map(|node| node.site_id.clone())
        .collect::<Vec<_>>();
    let vertex_by_site_id = vertex_site_ids
        .iter()
        .enumerate()
        .map(|(idx, site_id)| (site_id.clone(), idx as u32))
        .collect::<std::collections::BTreeMap<_, _>>();
    let edge_weight_scale = 1_000_000.0;
    let mut adjacency = graph
        .nodes
        .iter()
        .map(|node| (node.site_id.clone(), Vec::<(String, i32)>::new()))
        .collect::<std::collections::BTreeMap<_, _>>();

    for edge in &graph.edges {
        let weight = scaled_metis_weight(edge.weight, edge_weight_scale);
        adjacency
            .entry(edge.from_site_id.clone())
            .or_default()
            .push((edge.to_site_id.clone(), weight));
        adjacency
            .entry(edge.to_site_id.clone())
            .or_default()
            .push((edge.from_site_id.clone(), weight));
    }

    let mut xadj = Vec::with_capacity(vertex_site_ids.len() + 1);
    let mut adjncy = Vec::new();
    let mut adjwgt = Vec::new();
    xadj.push(0);
    for site_id in &vertex_site_ids {
        if let Some(neighbors) = adjacency.get_mut(site_id) {
            neighbors.sort_by(|left, right| left.0.cmp(&right.0));
            for (neighbor_site_id, weight) in neighbors {
                if let Some(vertex_idx) = vertex_by_site_id.get(neighbor_site_id) {
                    adjncy.push(*vertex_idx);
                    adjwgt.push(*weight);
                }
            }
        }
        xadj.push(adjncy.len() as u32);
    }

    let vwgt = graph
        .nodes
        .iter()
        .map(|node| scaled_metis_weight(node.demand, 1.0))
        .collect::<Vec<_>>();

    Ok(MetisCoreHandoff {
        vertex_site_ids,
        xadj,
        adjncy,
        vwgt,
        adjwgt,
        edge_weight_scale,
    })
}

pub fn build_site_graph(sites: &[Site]) -> SiteGraph {
    let mut nodes = sites
        .iter()
        .map(|site| SiteGraphNode {
            site_id: site.id.clone(),
            demand: site.demand,
            revenue: site.revenue,
            latitude: site.latitude,
            longitude: site.longitude,
        })
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| left.site_id.cmp(&right.site_id));

    let mut diagnostics = site_graph_input_diagnostics(&nodes);
    let edges = coordinate_distance_edges(&nodes);
    diagnostics.extend(site_graph_connectivity_diagnostics(&nodes, &edges));

    SiteGraph {
        nodes,
        edges,
        diagnostics,
    }
}

pub fn build_site_graph_with_edges(
    sites: &[Site],
    edge_inputs: &[SiteGraphEdgeInput],
) -> SiteGraph {
    let mut nodes = sites
        .iter()
        .map(|site| SiteGraphNode {
            site_id: site.id.clone(),
            demand: site.demand,
            revenue: site.revenue,
            latitude: site.latitude,
            longitude: site.longitude,
        })
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| left.site_id.cmp(&right.site_id));

    let mut diagnostics = site_graph_input_diagnostics(&nodes);
    let (edges, edge_diagnostics) = explicit_evidence_edges(&nodes, edge_inputs);
    diagnostics.extend(edge_diagnostics);
    diagnostics.extend(site_graph_connectivity_diagnostics(&nodes, &edges));

    SiteGraph {
        nodes,
        edges,
        diagnostics,
    }
}

pub fn site_graph_diagnostic_report(
    sites: &[Site],
    long_edge_threshold_degrees: f64,
) -> SiteGraphDiagnosticReport {
    let graph = build_site_graph(sites);
    let component_count = site_graph_component_count(&graph.nodes, &graph.edges);
    let mut diagnostics = graph.diagnostics.clone();
    diagnostics.extend(site_graph_isolated_site_diagnostics(
        &graph.nodes,
        &graph.edges,
    ));
    diagnostics.extend(site_graph_long_edge_diagnostics(
        &graph.edges,
        long_edge_threshold_degrees,
    ));
    diagnostics.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.site_ids.cmp(&right.site_ids))
            .then_with(|| left.message.cmp(&right.message))
    });

    SiteGraphDiagnosticReport {
        node_count: graph.nodes.len(),
        edge_count: graph.edges.len(),
        component_count,
        diagnostics,
    }
}

pub fn site_graph_diagnostic_report_with_edges(
    sites: &[Site],
    edge_inputs: &[SiteGraphEdgeInput],
    long_edge_threshold_degrees: f64,
) -> SiteGraphDiagnosticReport {
    let graph = build_site_graph_with_edges(sites, edge_inputs);
    let component_count = site_graph_component_count(&graph.nodes, &graph.edges);
    let mut diagnostics = graph.diagnostics.clone();
    diagnostics.extend(site_graph_isolated_site_diagnostics(
        &graph.nodes,
        &graph.edges,
    ));
    diagnostics.extend(site_graph_long_edge_diagnostics(
        &graph.edges,
        long_edge_threshold_degrees,
    ));
    diagnostics.sort_by(|left, right| {
        left.severity
            .cmp(&right.severity)
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.site_ids.cmp(&right.site_ids))
            .then_with(|| left.message.cmp(&right.message))
    });

    SiteGraphDiagnosticReport {
        node_count: graph.nodes.len(),
        edge_count: graph.edges.len(),
        component_count,
        diagnostics,
    }
}

pub fn site_graph_component_count(nodes: &[SiteGraphNode], edges: &[SiteGraphEdge]) -> usize {
    if nodes.is_empty() {
        return 0;
    }

    connected_components(nodes, edges).len()
}

pub fn site_graph_connectivity_diagnostics(
    nodes: &[SiteGraphNode],
    edges: &[SiteGraphEdge],
) -> Vec<SiteGraphDiagnostic> {
    if nodes.is_empty() {
        return vec![SiteGraphDiagnostic {
            severity: "error".to_string(),
            code: "empty-site-graph".to_string(),
            site_ids: Vec::new(),
            message: "site graph requires at least one site".to_string(),
        }];
    }

    let components = connected_components(nodes, edges);
    if components.len() <= 1 {
        return Vec::new();
    }

    components
        .into_iter()
        .map(|site_ids| SiteGraphDiagnostic {
            severity: "warning".to_string(),
            code: "disconnected-component".to_string(),
            message: format!(
                "site graph component has {} site(s): {}",
                site_ids.len(),
                site_ids.join(";")
            ),
            site_ids,
        })
        .collect()
}

pub fn site_graph_isolated_site_diagnostics(
    nodes: &[SiteGraphNode],
    edges: &[SiteGraphEdge],
) -> Vec<SiteGraphDiagnostic> {
    let connected_site_ids = edges
        .iter()
        .flat_map(|edge| [edge.from_site_id.as_str(), edge.to_site_id.as_str()])
        .collect::<std::collections::BTreeSet<_>>();

    nodes
        .iter()
        .filter(|node| !connected_site_ids.contains(node.site_id.as_str()))
        .map(|node| SiteGraphDiagnostic {
            severity: "warning".to_string(),
            code: "isolated-site".to_string(),
            site_ids: vec![node.site_id.clone()],
            message: format!("site '{}' has no graph edges", node.site_id),
        })
        .collect()
}

pub fn site_graph_long_edge_diagnostics(
    edges: &[SiteGraphEdge],
    threshold_degrees: f64,
) -> Vec<SiteGraphDiagnostic> {
    edges
        .iter()
        .filter(|edge| edge.weight > threshold_degrees)
        .map(|edge| SiteGraphDiagnostic {
            severity: "warning".to_string(),
            code: "long-edge".to_string(),
            site_ids: vec![edge.from_site_id.clone(), edge.to_site_id.clone()],
            message: format!(
                "edge weight {:.6} exceeds threshold {:.6}",
                edge.weight, threshold_degrees
            ),
        })
        .collect()
}

fn site_graph_input_diagnostics(nodes: &[SiteGraphNode]) -> Vec<SiteGraphDiagnostic> {
    let mut diagnostics = Vec::new();
    if nodes.is_empty() {
        diagnostics.push(SiteGraphDiagnostic {
            severity: "error".to_string(),
            code: "empty-site-set".to_string(),
            site_ids: Vec::new(),
            message: "cannot build a site graph without sites".to_string(),
        });
        return diagnostics;
    }

    let mut ids = std::collections::BTreeMap::<String, Vec<String>>::new();
    let mut coordinates = std::collections::BTreeMap::<String, Vec<String>>::new();
    for node in nodes {
        ids.entry(node.site_id.clone())
            .or_default()
            .push(node.site_id.clone());
        coordinates
            .entry(format!("{:.6},{:.6}", node.latitude, node.longitude))
            .or_default()
            .push(node.site_id.clone());
    }

    for (site_id, duplicates) in ids {
        if duplicates.len() > 1 {
            diagnostics.push(SiteGraphDiagnostic {
                severity: "error".to_string(),
                code: "duplicate-site-id".to_string(),
                site_ids: duplicates,
                message: format!("site ID '{site_id}' appears more than once"),
            });
        }
    }

    for (coordinate, mut site_ids) in coordinates {
        if site_ids.len() > 1 {
            site_ids.sort();
            diagnostics.push(SiteGraphDiagnostic {
                severity: "warning".to_string(),
                code: "duplicate-coordinate".to_string(),
                site_ids,
                message: format!("multiple sites share coordinate {coordinate}"),
            });
        }
    }

    diagnostics
}

fn connected_components(nodes: &[SiteGraphNode], edges: &[SiteGraphEdge]) -> Vec<Vec<String>> {
    let node_ids = nodes
        .iter()
        .map(|node| node.site_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let mut adjacency = node_ids
        .iter()
        .map(|site_id| (site_id.clone(), std::collections::BTreeSet::new()))
        .collect::<std::collections::BTreeMap<_, _>>();

    for edge in edges {
        if node_ids.contains(&edge.from_site_id) && node_ids.contains(&edge.to_site_id) {
            adjacency
                .entry(edge.from_site_id.clone())
                .or_default()
                .insert(edge.to_site_id.clone());
            adjacency
                .entry(edge.to_site_id.clone())
                .or_default()
                .insert(edge.from_site_id.clone());
        }
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut components = Vec::new();
    for site_id in &node_ids {
        if seen.contains(site_id) {
            continue;
        }
        let mut stack = vec![site_id.clone()];
        let mut component = Vec::new();
        while let Some(current) = stack.pop() {
            if !seen.insert(current.clone()) {
                continue;
            }
            component.push(current.clone());
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors.iter().rev() {
                    if !seen.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }
        component.sort();
        components.push(component);
    }
    components
}

fn coordinate_distance_edges(nodes: &[SiteGraphNode]) -> Vec<SiteGraphEdge> {
    let mut edges = Vec::new();
    for left_idx in 0..nodes.len() {
        for right_idx in (left_idx + 1)..nodes.len() {
            let left = &nodes[left_idx];
            let right = &nodes[right_idx];
            let d_lat = left.latitude - right.latitude;
            let avg_latitude_radians = ((left.latitude + right.latitude) / 2.0).to_radians();
            let d_lon = (left.longitude - right.longitude) * avg_latitude_radians.cos();
            edges.push(SiteGraphEdge {
                from_site_id: left.site_id.clone(),
                to_site_id: right.site_id.clone(),
                weight: (d_lat * d_lat + d_lon * d_lon).sqrt(),
                evidence: "latitude-adjusted-coordinate-distance-degrees".to_string(),
            });
        }
    }
    edges
}

fn explicit_evidence_edges(
    nodes: &[SiteGraphNode],
    edge_inputs: &[SiteGraphEdgeInput],
) -> (Vec<SiteGraphEdge>, Vec<SiteGraphDiagnostic>) {
    let node_ids = nodes
        .iter()
        .map(|node| node.site_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let mut edges = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen_pairs = std::collections::BTreeMap::<(String, String), Vec<String>>::new();

    for input in edge_inputs {
        let mut site_ids = vec![input.from_site_id.clone(), input.to_site_id.clone()];
        site_ids.sort();
        if !node_ids.contains(&input.from_site_id) || !node_ids.contains(&input.to_site_id) {
            diagnostics.push(SiteGraphDiagnostic {
                severity: "error".to_string(),
                code: "unknown-edge-site".to_string(),
                site_ids,
                message: format!(
                    "edge references unknown site '{}' or '{}'",
                    input.from_site_id, input.to_site_id
                ),
            });
            continue;
        }
        if input.from_site_id == input.to_site_id {
            diagnostics.push(SiteGraphDiagnostic {
                severity: "error".to_string(),
                code: "self-edge".to_string(),
                site_ids,
                message: format!(
                    "edge cannot connect site '{}' to itself",
                    input.from_site_id
                ),
            });
            continue;
        }

        let (from_site_id, to_site_id) = if input.from_site_id <= input.to_site_id {
            (input.from_site_id.clone(), input.to_site_id.clone())
        } else {
            (input.to_site_id.clone(), input.from_site_id.clone())
        };
        seen_pairs
            .entry((from_site_id.clone(), to_site_id.clone()))
            .or_default()
            .push(input.evidence.clone());
        edges.push(SiteGraphEdge {
            from_site_id,
            to_site_id,
            weight: input.weight,
            evidence: input.evidence.clone(),
        });
    }

    for ((from_site_id, to_site_id), evidence_rows) in seen_pairs {
        if evidence_rows.len() > 1 {
            diagnostics.push(SiteGraphDiagnostic {
                severity: "warning".to_string(),
                code: "duplicate-edge".to_string(),
                site_ids: vec![from_site_id.clone(), to_site_id.clone()],
                message: format!(
                    "edge '{}' to '{}' appears {} times",
                    from_site_id,
                    to_site_id,
                    evidence_rows.len()
                ),
            });
        }
    }

    edges.sort_by(|left, right| {
        left.from_site_id
            .cmp(&right.from_site_id)
            .then_with(|| left.to_site_id.cmp(&right.to_site_id))
            .then_with(|| left.evidence.cmp(&right.evidence))
            .then_with(|| left.weight.total_cmp(&right.weight))
    });
    (edges, diagnostics)
}

fn graph_partition_seeds(graph: &SiteGraph, seed_count: usize) -> Vec<String> {
    let Some(first_seed) = graph.nodes.iter().max_by(|left, right| {
        left.demand
            .total_cmp(&right.demand)
            .then_with(|| right.site_id.cmp(&left.site_id))
    }) else {
        return Vec::new();
    };
    let mut seeds = vec![first_seed.site_id.clone()];

    while seeds.len() < seed_count {
        let Some(next_seed) = graph
            .nodes
            .iter()
            .filter(|node| !seeds.contains(&node.site_id))
            .max_by(|left, right| {
                nearest_seed_weight(graph, &left.site_id, &seeds)
                    .total_cmp(&nearest_seed_weight(graph, &right.site_id, &seeds))
                    .then_with(|| left.demand.total_cmp(&right.demand))
                    .then_with(|| right.site_id.cmp(&left.site_id))
            })
        else {
            break;
        };
        seeds.push(next_seed.site_id.clone());
    }

    seeds
}

fn nearest_seed_weight(graph: &SiteGraph, site_id: &str, seeds: &[String]) -> f64 {
    seeds
        .iter()
        .map(|seed_id| graph_edge_weight(graph, site_id, seed_id))
        .fold(f64::INFINITY, f64::min)
}

fn graph_edge_weight(graph: &SiteGraph, left_site_id: &str, right_site_id: &str) -> f64 {
    if left_site_id == right_site_id {
        return 0.0;
    }

    graph
        .edges
        .iter()
        .find(|edge| {
            (edge.from_site_id == left_site_id && edge.to_site_id == right_site_id)
                || (edge.from_site_id == right_site_id && edge.to_site_id == left_site_id)
        })
        .map(|edge| edge.weight)
        .unwrap_or(f64::INFINITY)
}

fn scaled_metis_weight(value: f64, scale: f64) -> i32 {
    (value * scale).round().max(1.0).min(i32::MAX as f64) as i32
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

pub fn sample_site_edges_csv() -> &'static str {
    "from_site_id,to_site_id,weight,evidence\n\
N-001,N-002,0.064031,field-adjacency\n\
N-001,N-003,0.072111,field-adjacency\n\
N-003,S-001,0.082462,manager-review\n\
S-001,S-002,0.072111,field-adjacency\n\
S-002,S-003,0.086023,field-adjacency\n"
}

pub fn sample_assignee_capacity_csv() -> &'static str {
    "assignee,team,responsibility,asset_group,capacity,home_latitude,home_longitude,skills\n\
Avery,north,territory,enterprise_accounts,9,47.61,-122.34,enterprise;onsite\n\
Avery,north,territory,field_visits,5,47.61,-122.34,onsite\n\
Morgan,north,renewals,subscription_book,12,47.66,-122.30,renewal\n\
Sam,north,onsite,field_visits,10,47.59,-122.28,onsite\n\
Jordan,south,territory,enterprise_accounts,16,47.49,-122.29,enterprise\n\
Riley,south,renewals,subscription_book,14,47.52,-122.36,renewal;onsite\n"
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

fn extract_product_demands(
    fields: &[String],
    header_map: &std::collections::BTreeMap<String, usize>,
    line: usize,
) -> Result<Vec<ProductDemand>, CsvIntakeError> {
    let mut product_demands = Vec::new();
    for (header, idx) in header_map {
        let Some(product) = header.strip_prefix("product_") else {
            continue;
        };
        let Some(value) = fields.get(*idx).map(String::as_str) else {
            continue;
        };
        if value.trim().is_empty() {
            continue;
        }
        product_demands.push(ProductDemand {
            product: product.to_string(),
            demand: parse_f64(value, line, header)?,
        });
    }
    Ok(product_demands)
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

fn assignee_capacity_totals(
    capacities: &[AssigneeCapacity],
) -> std::collections::BTreeMap<String, f64> {
    let mut totals = std::collections::BTreeMap::new();
    for capacity in capacities {
        *totals.entry(capacity.assignee.clone()).or_insert(0.0) += capacity.capacity;
    }
    totals
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

fn ordered_site_pair(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    }
}

fn territory_edge_components(sites: &[Site], edges: &[(String, String)]) -> Vec<Vec<String>> {
    let site_ids = sites
        .iter()
        .map(|site| site.id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let mut adjacency = site_ids
        .iter()
        .map(|site_id| (site_id.clone(), std::collections::BTreeSet::new()))
        .collect::<std::collections::BTreeMap<_, _>>();
    for (left, right) in edges {
        if site_ids.contains(left) && site_ids.contains(right) {
            adjacency
                .entry(left.clone())
                .or_default()
                .insert(right.clone());
            adjacency
                .entry(right.clone())
                .or_default()
                .insert(left.clone());
        }
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut components = Vec::new();
    for site_id in &site_ids {
        if seen.contains(site_id) {
            continue;
        }
        let mut stack = vec![site_id.clone()];
        let mut component = Vec::new();
        while let Some(current) = stack.pop() {
            if !seen.insert(current.clone()) {
                continue;
            }
            component.push(current.clone());
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors.iter().rev() {
                    if !seen.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }
        component.sort();
        components.push(component);
    }
    components
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
    fn exposes_dashboard_schema_contract() {
        let schema = dashboard_schema_json();

        assert!(schema.contains("\"schema_id\":\"terrain.dashboard.v1\""));
        assert!(schema.contains("\"territory_id\""));
        assert!(schema.contains("\"site_id\""));
        assert!(schema.contains("\"capacity_exception\""));
    }

    #[test]
    fn exposes_integration_fixture_manifest() {
        let manifest = integration_fixture_manifest_json();

        assert!(manifest.contains("\"manifest_id\":\"terrain.integration-fixtures.v1\""));
        assert!(manifest.contains("\"repo\":\"CROP\""));
        assert!(manifest.contains("\"repo\":\"PEBBLE\""));
        assert!(manifest.contains("\"repo\":\"FLETCH\""));
        assert!(manifest.contains("fixtures/sample-capacity.csv"));
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
    fn audits_multi_product_demand_balance() {
        let territories = parse_territories_csv(
            "territory_id,site_id,demand,revenue,latitude,longitude,product_alpha,product_beta\n\
north,N-001,10,100,1,1,9,1\n\
south,S-001,10,100,2,2,2,8\n",
        )
        .expect("product demand parses");

        let audit = audit_product_balance(&territories, 0.25);

        assert_eq!(territories[0].sites[0].product_demands.len(), 2);
        assert_eq!(audit.balances.len(), 2);
        assert!(!audit.passes);
        assert!(audit.overall_spread_score > 3.0);
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
    fn partitions_sites_with_graph_seed_baseline() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let territories = partition_sites_by_graph(&sites, 2).expect("graph partition works");
        let site_ids = territories
            .iter()
            .flat_map(|territory| &territory.sites)
            .map(|site| site.id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(territories.len(), 2);
        assert_eq!(territories[0].id, "territory-1");
        assert_eq!(territories[1].id, "territory-2");
        assert_eq!(site_ids.len(), 6);
        assert!(site_ids.contains(&"N-001"));
        assert!(site_ids.contains(&"S-003"));
    }

    #[test]
    fn compares_graph_partition_to_greedy_baseline() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let report = graph_partition_report(&sites, 2, 0.10, 0.06).expect("report builds");

        assert_eq!(report.baseline.len(), 2);
        assert_eq!(report.graph_partition.len(), 2);
        assert_eq!(report.movements.len(), 6);
        assert_eq!(report.graph_diagnostics.component_count, 1);
        assert!(!report.graph_diagnostics.diagnostics.is_empty());
    }

    #[test]
    fn emits_metis_core_csr_handoff() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let handoff = metis_core_handoff(&sites).expect("handoff builds");

        assert_eq!(
            handoff.vertex_site_ids,
            ["N-001", "N-002", "N-003", "S-001", "S-002", "S-003"]
        );
        assert_eq!(handoff.xadj.len(), handoff.vertex_site_ids.len() + 1);
        assert_eq!(handoff.xadj[0], 0);
        assert_eq!(
            handoff.xadj.last().copied(),
            Some(handoff.adjncy.len() as u32)
        );
        assert_eq!(handoff.adjncy.len(), handoff.adjwgt.len());
        assert!(handoff.vwgt.iter().all(|weight| *weight > 0));
        assert!(handoff.adjwgt.iter().all(|weight| *weight > 0));
    }

    #[test]
    fn partitions_sites_with_github_metis_core_dependency() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let territories =
            partition_sites_with_metis_core(&sites, 2, 7).expect("METIS partition works");
        let assigned_site_count = territories
            .iter()
            .map(|territory| territory.sites.len())
            .sum::<usize>();

        assert_eq!(territories.len(), 2);
        assert_eq!(assigned_site_count, sites.len());
        assert!(
            territories
                .iter()
                .all(|territory| !territory.sites.is_empty())
        );
    }

    #[test]
    fn parses_assignee_capacity_csv() {
        let capacities =
            parse_assignee_capacity_csv(sample_assignee_capacity_csv()).expect("capacity parses");

        assert_eq!(capacities.len(), 6);
        assert_eq!(capacities[0].assignee, "Avery");
        assert_eq!(capacities[0].team, "north");
        assert_eq!(capacities[0].responsibility, "territory");
        assert_eq!(capacities[0].asset_group, "enterprise_accounts");
        near(capacities[0].capacity, 9.0);
        assert_eq!(capacities[0].skills, ["enterprise", "onsite"]);
    }

    #[test]
    fn sums_multiple_capacity_lanes_per_assignee() {
        let capacities =
            parse_assignee_capacity_csv(sample_assignee_capacity_csv()).expect("capacity parses");
        let total = territory_capacity(
            &Territory::new("north", Vec::new()).with_assignees(["Avery"]),
            &capacities,
        );

        near(total, 14.0);
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

    #[test]
    fn builds_deterministic_site_graph_contract() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let graph = build_site_graph(&sites);

        assert_eq!(graph.nodes.len(), 6);
        assert_eq!(graph.edges.len(), 15);
        assert!(graph.diagnostics.is_empty());
        assert_eq!(graph.nodes[0].site_id, "N-001");
        assert_eq!(graph.edges[0].from_site_id, "N-001");
        assert_eq!(graph.edges[0].to_site_id, "N-002");
        assert_eq!(
            graph.edges[0].evidence,
            "latitude-adjusted-coordinate-distance-degrees"
        );
        assert!(graph.edges[0].weight > 0.0);
    }

    #[test]
    fn reports_site_graph_input_diagnostics() {
        let sites = vec![
            Site::new("A", 1.0, 10.0, 47.0, -122.0),
            Site::new("A", 2.0, 20.0, 47.0, -122.1),
            Site::new("B", 3.0, 30.0, 47.0, -122.0),
        ];
        let graph = build_site_graph(&sites);
        let codes = graph
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect::<Vec<_>>();

        assert!(codes.contains(&"duplicate-site-id"));
        assert!(codes.contains(&"duplicate-coordinate"));
    }

    #[test]
    fn reports_disconnected_site_graph_components() {
        let nodes = vec![
            SiteGraphNode {
                site_id: "A".to_string(),
                demand: 1.0,
                revenue: 10.0,
                latitude: 0.0,
                longitude: 0.0,
            },
            SiteGraphNode {
                site_id: "B".to_string(),
                demand: 1.0,
                revenue: 10.0,
                latitude: 1.0,
                longitude: 1.0,
            },
            SiteGraphNode {
                site_id: "C".to_string(),
                demand: 1.0,
                revenue: 10.0,
                latitude: 2.0,
                longitude: 2.0,
            },
        ];
        let diagnostics = site_graph_connectivity_diagnostics(
            &nodes,
            &[SiteGraphEdge {
                from_site_id: "A".to_string(),
                to_site_id: "B".to_string(),
                weight: 1.0,
                evidence: "fixture".to_string(),
            }],
        );

        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].code, "disconnected-component");
        assert_eq!(diagnostics[0].site_ids, ["A", "B"]);
        assert_eq!(diagnostics[1].site_ids, ["C"]);
    }

    #[test]
    fn reports_graph_diagnostics_with_stable_ordering() {
        let sites = vec![
            Site::new("B", 1.0, 10.0, 47.0, -122.0),
            Site::new("A", 1.0, 10.0, 47.2, -122.2),
        ];
        let report = site_graph_diagnostic_report(&sites, 0.01);

        assert_eq!(report.node_count, 2);
        assert_eq!(report.edge_count, 1);
        assert_eq!(report.component_count, 1);
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(report.diagnostics[0].code, "long-edge");
        assert_eq!(report.diagnostics[0].site_ids, ["A", "B"]);
    }

    #[test]
    fn reports_isolated_sites_for_empty_edge_sets() {
        let nodes = vec![SiteGraphNode {
            site_id: "solo".to_string(),
            demand: 1.0,
            revenue: 10.0,
            latitude: 47.0,
            longitude: -122.0,
        }];
        let diagnostics = site_graph_isolated_site_diagnostics(&nodes, &[]);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "isolated-site");
        assert_eq!(diagnostics[0].site_ids, ["solo"]);
    }

    #[test]
    fn parses_site_edge_evidence_csv() {
        let edges = parse_site_edges_csv(sample_site_edges_csv()).expect("edge sample parses");

        assert_eq!(edges.len(), 5);
        assert_eq!(edges[0].from_site_id, "N-001");
        assert_eq!(edges[0].to_site_id, "N-002");
        near(edges[0].weight, 0.064031);
        assert_eq!(edges[0].evidence, "field-adjacency");
    }

    #[test]
    fn builds_site_graph_from_explicit_edge_evidence() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let edges = parse_site_edges_csv(sample_site_edges_csv()).expect("edge sample parses");
        let graph = build_site_graph_with_edges(&sites, &edges);

        assert_eq!(graph.nodes.len(), 6);
        assert_eq!(graph.edges.len(), 5);
        assert!(graph.diagnostics.is_empty());
        assert_eq!(graph.edges[0].from_site_id, "N-001");
        assert_eq!(graph.edges[0].to_site_id, "N-002");
        assert_eq!(graph.edges[0].evidence, "field-adjacency");
    }

    #[test]
    fn diagnoses_invalid_edge_evidence() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let edges = vec![
            SiteGraphEdgeInput {
                from_site_id: "N-001".to_string(),
                to_site_id: "N-002".to_string(),
                weight: 0.1,
                evidence: "fixture".to_string(),
            },
            SiteGraphEdgeInput {
                from_site_id: "N-002".to_string(),
                to_site_id: "N-001".to_string(),
                weight: 0.1,
                evidence: "duplicate".to_string(),
            },
            SiteGraphEdgeInput {
                from_site_id: "N-003".to_string(),
                to_site_id: "missing".to_string(),
                weight: 0.1,
                evidence: "bad-ref".to_string(),
            },
        ];
        let report = site_graph_diagnostic_report_with_edges(&sites, &edges, 0.05);
        let codes = report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect::<Vec<_>>();

        assert_eq!(report.edge_count, 2);
        assert!(codes.contains(&"duplicate-edge"));
        assert!(codes.contains(&"unknown-edge-site"));
        assert!(codes.contains(&"disconnected-component"));
        assert!(codes.contains(&"isolated-site"));
        assert!(codes.contains(&"long-edge"));
    }

    #[test]
    fn emits_metis_core_handoff_from_edge_evidence() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let edges = parse_site_edges_csv(sample_site_edges_csv()).expect("edge sample parses");
        let handoff = metis_core_handoff_with_edges(&sites, &edges).expect("handoff builds");

        assert_eq!(handoff.vertex_site_ids.len(), 6);
        assert_eq!(handoff.xadj, [0, 2, 3, 5, 7, 9, 10]);
        assert_eq!(handoff.adjncy.len(), 10);
        assert_eq!(handoff.adjwgt.len(), 10);
        assert!(handoff.adjwgt.iter().all(|weight| *weight > 0));
    }

    #[test]
    fn partitions_sites_with_metis_core_edge_evidence() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let edges = parse_site_edges_csv(sample_site_edges_csv()).expect("edge sample parses");
        let territories =
            partition_sites_with_metis_core_edges(&sites, &edges, 2, 7).expect("partition works");
        let assigned_site_count = territories
            .iter()
            .map(|territory| territory.sites.len())
            .sum::<usize>();

        assert_eq!(territories.len(), 2);
        assert_eq!(assigned_site_count, sites.len());
    }

    #[test]
    fn renders_site_graph_svg_with_edge_bindings() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let edges = parse_site_edges_csv(sample_site_edges_csv()).expect("edge sample parses");
        let svg = render_site_graph_svg(&sites, &edges, &TerritoryVisualOptions::default());

        assert!(svg.contains("class=\"site-edge\""));
        assert!(svg.contains("data-from-site-id=\"N-001\""));
        assert!(svg.contains("data-to-site-id=\"N-002\""));
        assert!(svg.contains("data-evidence=\"field-adjacency\""));
        assert!(svg.contains("data-site-id=\"S-003\""));
    }

    #[test]
    fn renders_site_graph_geojson_with_edge_features() {
        let sites = parse_sites_csv(sample_sites_csv()).expect("site sample parses");
        let edges = parse_site_edges_csv(sample_site_edges_csv()).expect("edge sample parses");
        let geojson = render_site_graph_geojson(&sites, &edges);

        assert!(geojson.contains("\"name\":\"terrain-edge-evidence\""));
        assert!(geojson.contains("\"kind\":\"site_edge\""));
        assert!(geojson.contains("\"from_site_id\":\"N-001\""));
        assert!(geojson.contains("\"to_site_id\":\"N-002\""));
        assert!(geojson.contains("\"kind\":\"site\""));
    }

    #[test]
    fn audits_territory_edges_for_cut_edges_and_disconnected_components() {
        let territories =
            parse_territories_csv(sample_territories_csv()).expect("territories parse");
        let edges = parse_site_edges_csv(sample_site_edges_csv()).expect("edges parse");
        let report = territory_edge_audit(&territories, &edges);
        let codes = report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect::<Vec<_>>();

        assert_eq!(report.territory_count, 2);
        assert_eq!(report.site_count, 6);
        assert_eq!(report.edge_count, 5);
        assert_eq!(report.cut_edge_count, 1);
        assert_eq!(report.disconnected_territory_count, 0);
        assert!(codes.contains(&"cut-edge"));
    }

    #[test]
    fn audits_territory_edges_for_unknown_sites() {
        let territories =
            parse_territories_csv(sample_territories_csv()).expect("territories parse");
        let edges = vec![SiteGraphEdgeInput {
            from_site_id: "N-001".to_string(),
            to_site_id: "missing".to_string(),
            weight: 1.0,
            evidence: "fixture".to_string(),
        }];
        let report = territory_edge_audit(&territories, &edges);

        assert_eq!(report.diagnostics[0].severity, "error");
        assert_eq!(report.diagnostics[0].code, "unknown-edge-site");
    }

    #[test]
    fn audits_territory_edges_for_disconnected_territories() {
        let territories =
            parse_territories_csv(sample_territories_csv()).expect("territories parse");
        let edges = vec![SiteGraphEdgeInput {
            from_site_id: "N-001".to_string(),
            to_site_id: "N-002".to_string(),
            weight: 1.0,
            evidence: "fixture".to_string(),
        }];
        let report = territory_edge_audit(&territories, &edges);
        let disconnected = report
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "disconnected-territory")
            .expect("disconnected territory diagnostic exists");

        assert_eq!(report.disconnected_territory_count, 2);
        assert_eq!(disconnected.severity, "warning");
    }
}
