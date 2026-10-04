//! Bounded browser adapter around TERRAIN's native planning and audit functions.
use serde::Serialize;
use std::collections::BTreeSet;
use terrain_core::{
    TerritoryVisualOptions, audit_territories, parse_sites_csv, partition_sites,
    partition_sites_with_metis_core, render_territory_geojson, render_territory_svg,
};

#[derive(Debug, Serialize)]
pub struct Summary {
    pub id: String,
    pub site_count: usize,
    pub demand: f64,
    pub revenue: f64,
    pub radius_degrees: f64,
}

#[derive(Debug, Serialize)]
pub struct Plan {
    pub site_count: usize,
    pub territory_count: usize,
    pub passes: bool,
    pub demand_ratio: Option<f64>,
    pub revenue_ratio: Option<f64>,
    pub radius_degrees: f64,
    pub summaries: Vec<Summary>,
    pub svg: String,
    pub geojson: String,
}

pub fn plan(
    csv: &str,
    count: usize,
    graph: bool,
    demand_limit: f64,
    revenue_limit: f64,
) -> Result<Plan, String> {
    if csv.len() > 500_000 || !(1..=32).contains(&count) {
        return Err("Use up to 500 KB of CSV and 1–32 territories".into());
    }
    if !demand_limit.is_finite()
        || !revenue_limit.is_finite()
        || !(1.0..=10.0).contains(&demand_limit)
        || !(1.0..=10.0).contains(&revenue_limit)
    {
        return Err("Balance ratios must be finite and between 1 and 10".into());
    }
    let sites = parse_sites_csv(csv).map_err(|error| error.to_string())?;
    if sites.is_empty() || sites.len() > 2000 || count > sites.len() {
        return Err("Use 1–2000 sites and no more territories than sites".into());
    }
    let mut ids = BTreeSet::new();
    for site in &sites {
        if !ids.insert(&site.id) {
            return Err(format!("Duplicate site ID: {}", site.id));
        }
        if !site.demand.is_finite()
            || !site.revenue.is_finite()
            || !(0.0..=1e12).contains(&site.demand)
            || !(0.0..=1e12).contains(&site.revenue)
            || !site.latitude.is_finite()
            || !site.longitude.is_finite()
            || !(-90.0..=90.0).contains(&site.latitude)
            || !(-180.0..=180.0).contains(&site.longitude)
        {
            return Err(format!(
                "{}: use nonnegative finite workload/revenue and valid coordinates",
                site.id
            ));
        }
    }
    let site_count = sites.len();
    let territories = if graph {
        partition_sites_with_metis_core(&sites, count, 42)
    } else {
        partition_sites(sites, count)
    }
    .map_err(|error| error.to_string())?;
    let audit = audit_territories(&territories, demand_limit, revenue_limit);
    let demand_ratio = maximum_minimum(audit.summaries.iter().map(|s| s.demand));
    let revenue_ratio = maximum_minimum(audit.summaries.iter().map(|s| s.revenue));
    Ok(Plan {
        site_count,
        territory_count: territories.len(),
        passes: demand_ratio.is_some_and(|ratio| ratio <= demand_limit)
            && revenue_ratio.is_some_and(|ratio| ratio <= revenue_limit),
        demand_ratio,
        revenue_ratio,
        radius_degrees: audit.max_radius_degrees,
        summaries: audit
            .summaries
            .iter()
            .map(|summary| Summary {
                id: summary.territory_id.clone(),
                site_count: summary.site_count,
                demand: summary.demand,
                revenue: summary.revenue,
                radius_degrees: summary.max_radius_degrees,
            })
            .collect(),
        svg: render_territory_svg(&territories, &TerritoryVisualOptions::default()),
        geojson: render_territory_geojson(&territories),
    })
}

// Equal zero totals are balanced; a zero minimum with positive totals is unbounded.
fn maximum_minimum(values: impl Iterator<Item = f64>) -> Option<f64> {
    let (min, max) = values.fold((f64::INFINITY, 0.0_f64), |(min, max), value| {
        (min.min(value), max.max(value))
    });
    if max == 0.0 {
        Some(1.0)
    } else if min == 0.0 {
        None
    } else {
        Some(max / min)
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn plan_json(
    csv: &str,
    count: usize,
    graph: bool,
    demand_limit: f64,
    revenue_limit: f64,
) -> Result<String, wasm_bindgen::JsValue> {
    let result = plan(csv, count, graph, demand_limit, revenue_limit)
        .and_then(|value| serde_json::to_string(&value).map_err(|error| error.to_string()));
    result.map_err(|error| wasm_bindgen::JsValue::from_str(&error))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "site_id,demand,revenue,latitude,longitude\na,10,100,47,-122\nb,20,200,48,-121\nc,10,100,47.1,-122.1\nd,20,200,48.1,-121.1\n";
    #[test]
    fn partitions_preserve_sites_and_totals_in_both_modes() {
        for graph in [false, true] {
            let result = plan(SAMPLE, 2, graph, 2.0, 2.0).unwrap();
            assert_eq!((result.site_count, result.territory_count), (4, 2));
            assert_eq!(
                result
                    .summaries
                    .iter()
                    .map(|value| value.site_count)
                    .sum::<usize>(),
                4
            );
            assert_eq!(
                result
                    .summaries
                    .iter()
                    .map(|value| value.demand)
                    .sum::<f64>(),
                60.0
            );
            assert!(result.svg.contains("<svg"));
            let geojson: serde_json::Value = serde_json::from_str(&result.geojson).unwrap();
            assert_eq!(geojson["features"].as_array().unwrap().len(), 6);
        }
    }
    #[test]
    fn balance_uses_maximum_minimum_and_handles_zero_totals() {
        assert_eq!(maximum_minimum([10.0, 20.0].into_iter()), Some(2.0));
        assert_eq!(maximum_minimum([0.0, 20.0].into_iter()), None);
        assert_eq!(maximum_minimum([0.0, 0.0].into_iter()), Some(1.0));
        let csv = "site_id,demand,revenue,latitude,longitude\na,10,100,47,-122\nb,20,200,48,-121\n";
        let strict = plan(csv, 2, false, 1.0, 1.0).unwrap();
        assert_eq!(strict.demand_ratio, Some(2.0));
        assert!(!strict.passes);
        assert!(plan(csv, 2, false, 2.0, 2.0).unwrap().passes);
        assert!(
            !plan(&csv.replace("a,10,100", "a,0,0"), 2, false, 3.0, 3.0)
                .unwrap()
                .passes
        );
    }
    #[test]
    fn malformed_and_unbounded_inputs_are_rejected() {
        assert!(plan(SAMPLE, 0, false, 2.0, 2.0).is_err());
        assert!(plan(SAMPLE, 5, false, 2.0, 2.0).is_err());
        assert!(plan(SAMPLE, 2, false, f64::NAN, 2.0).is_err());
        assert!(plan(&SAMPLE.replace("b,20", "a,20"), 2, false, 2.0, 2.0).is_err());
        assert!(plan(&SAMPLE.replace("47,-122", "91,-122"), 2, false, 2.0, 2.0).is_err());
        assert!(plan(&SAMPLE.replace("a,10", "a,NaN"), 2, false, 2.0, 2.0).is_err());
    }
}
