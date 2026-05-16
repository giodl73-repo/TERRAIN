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

pub fn sample_territories_csv() -> &'static str {
    "territory_id,territory_label,assignees,site_id,demand,revenue,latitude,longitude\n\
north,North field team,Avery;Morgan;Sam,N-001,12,120000,47.62,-122.35\n\
north,North field team,Avery;Morgan;Sam,N-002,10,90000,47.67,-122.31\n\
north,North field team,Avery;Morgan;Sam,N-003,9,80000,47.58,-122.29\n\
south,South field team,Jordan;Riley,S-001,11,105000,47.50,-122.27\n\
south,South field team,Jordan;Riley,S-002,10,95000,47.46,-122.33\n\
south,South field team,Jordan;Riley,S-003,10,92000,47.53,-122.38\n"
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
}
