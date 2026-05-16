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
    pub sites: Vec<Site>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerritorySummary {
    pub territory_id: String,
    pub site_count: usize,
    pub demand: f64,
    pub revenue: f64,
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
        Self {
            id: id.into(),
            sites,
        }
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

pub fn sample_territories() -> Vec<Territory> {
    vec![
        Territory::new(
            "north",
            vec![
                Site::new("N-001", 12.0, 120_000.0, 47.62, -122.35),
                Site::new("N-002", 10.0, 90_000.0, 47.67, -122.31),
                Site::new("N-003", 9.0, 80_000.0, 47.58, -122.29),
            ],
        ),
        Territory::new(
            "south",
            vec![
                Site::new("S-001", 11.0, 105_000.0, 47.50, -122.27),
                Site::new("S-002", 10.0, 95_000.0, 47.46, -122.33),
                Site::new("S-003", 10.0, 92_000.0, 47.53, -122.38),
            ],
        ),
    ]
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
}
