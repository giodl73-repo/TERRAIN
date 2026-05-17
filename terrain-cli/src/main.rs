use terrain_core::{
    TerritoryVisualOptions, audit_product_balance, audit_territories, capacity_exceptions,
    compactness_exceptions, compare_territory_plans, dashboard_schema_json,
    diagnose_territories_csv, graph_partition_report, integration_fixture_manifest_json,
    parse_assignee_capacity_csv, parse_sites_csv, parse_territories_csv, partition_count_sweep,
    partition_sites, render_territory_geojson, render_territory_geojson_with_capacity,
    render_territory_svg, render_territory_svg_with_capacity, sample_assignee_capacity_csv,
    sample_proposed_territories_csv, sample_sites_csv, sample_territories, sample_territories_csv,
    site_graph_diagnostic_report, site_movements, summarize_territory,
};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args
        .first()
        .is_some_and(|arg| arg == "--help" || arg == "-h")
    {
        print_help();
        return;
    }

    match args.first().map(String::as_str).unwrap_or("sample-audit") {
        "sample-audit" => print_sample_audit(),
        "schema" => print_schema(),
        "integration-fixtures" => print_integration_fixtures(),
        "integration-packet" => run_integration_packet_command(args.get(1)),
        "sample-svg" => print_sample_svg(),
        "sample-geojson" => print_sample_geojson(),
        "sample-csv" => print_sample_csv(),
        "sample-proposed-csv" => print_sample_proposed_csv(),
        "sample-sites-csv" => print_sample_sites_csv(),
        "sample-capacity-csv" => print_sample_capacity_csv(),
        "audit-csv" => run_csv_command(args.get(1), print_audit_for_csv),
        "product-balance-csv" => run_csv_command(args.get(1), print_product_balance_for_csv),
        "capacity-csv" => run_csv_command(args.get(1), print_capacity_for_csv),
        "capacity-audit-csv" => run_capacity_audit_command(args.get(1), args.get(2)),
        "diagnose-csv" => run_csv_command(args.get(1), print_diagnostics_for_csv),
        "graph-diagnostics-csv" => run_graph_diagnostics_command(args.get(1), args.get(2)),
        "graph-partition-csv" => run_graph_partition_command(args.get(1), args.get(2)),
        "compare-csv" => run_compare_command(args.get(1), args.get(2)),
        "movement-csv" => run_movement_command(args.get(1), args.get(2)),
        "compactness-csv" => run_compactness_command(args.get(1), args.get(2)),
        "packet-csv" => run_packet_command(args.get(1), args.get(2), args.get(3)),
        "field-review-csv" => run_field_review_command(args.get(1), args.get(2)),
        "fairness-packet-csv" => run_fairness_packet_command(args.get(1), args.get(2), args.get(3)),
        "svg-csv" => run_csv_command(args.get(1), print_svg_for_csv),
        "geojson-csv" => run_csv_command(args.get(1), print_geojson_for_csv),
        "ownership-svg-csv" => run_ownership_svg_command(args.get(1), args.get(2)),
        "ownership-geojson-csv" => run_ownership_geojson_command(args.get(1), args.get(2)),
        "partition-csv" => run_partition_command(args.get(1), args.get(2), print_partition_audit),
        "sweep-csv" => run_sweep_command(args.get(1), args.get(2), args.get(3)),
        "partition-svg-csv" => run_partition_command(args.get(1), args.get(2), print_partition_svg),
        "partition-geojson-csv" => {
            run_partition_command(args.get(1), args.get(2), print_partition_geojson)
        }
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!("terrain - balanced territory planning");
    println!();
    println!("Commands:");
    println!("  schema         Emit the TERRAIN dashboard schema contract");
    println!("  integration-fixtures Emit reusable fixture and cache-source manifest");
    println!("  integration-packet OUT_DIR Write schema and fixture manifests");
    println!("  sample-audit   Run the built-in territory balance audit fixture");
    println!("  sample-svg     Emit a data-bound SVG territory split fixture");
    println!("  sample-geojson Emit a data-bound GeoJSON territory split fixture");
    println!("  sample-csv     Emit the built-in CSV intake fixture");
    println!("  sample-proposed-csv Emit a proposed-plan CSV fixture for comparison");
    println!("  sample-sites-csv Emit the built-in unassigned site CSV fixture");
    println!("  sample-capacity-csv Emit the built-in assignee capacity fixture");
    println!("  audit-csv PATH Audit a territory CSV file");
    println!("  product-balance-csv PATH Audit per-product demand balance");
    println!("  capacity-csv PATH Summarize assignee capacity CSV");
    println!("  capacity-audit-csv TERRITORIES CAPACITY Report capacity overloads");
    println!("  diagnose-csv PATH Report territory CSV intake diagnostics");
    println!("  graph-diagnostics-csv PATH [LONG_EDGE_THRESHOLD] Report site graph diagnostics");
    println!("  graph-partition-csv PATH COUNT Compare greedy and graph-backed partitions");
    println!("  compare-csv BASELINE PROPOSED Compare two territory CSV plans");
    println!("  movement-csv BASELINE PROPOSED List stable site movement between plans");
    println!("  compactness-csv PATH THRESHOLD Report max-radius compactness exceptions");
    println!("  packet-csv BASELINE PROPOSED OUT_DIR Write a scenario review packet");
    println!("  field-review-csv BASELINE PROPOSED Emit a plain-language field review");
    println!("  fairness-packet-csv TERRITORIES CAPACITY OUT_DIR Write ownership packet");
    println!("  svg-csv PATH   Emit a data-bound SVG split from a CSV file");
    println!("  geojson-csv PATH Emit a data-bound GeoJSON split from a CSV file");
    println!("  ownership-svg-csv TERRITORIES CAPACITY Emit SVG with capacity bindings");
    println!("  ownership-geojson-csv TERRITORIES CAPACITY Emit GeoJSON with capacity bindings");
    println!("  partition-csv PATH COUNT Audit a deterministic partition from site rows");
    println!("  sweep-csv PATH MIN MAX Compare deterministic partition counts");
    println!("  partition-svg-csv PATH COUNT Emit SVG for a deterministic partition");
    println!("  partition-geojson-csv PATH COUNT Emit GeoJSON for a deterministic partition");
}

fn print_sample_audit() {
    print_audit(&sample_territories());
}

fn print_schema() {
    println!("{}", dashboard_schema_json());
}

fn print_integration_fixtures() {
    println!("{}", integration_fixture_manifest_json());
}

fn run_integration_packet_command(output_dir: Option<&String>) {
    let Some(output_dir) = output_dir else {
        eprintln!("missing output directory");
        print_help();
        std::process::exit(2);
    };
    let output_dir = std::path::Path::new(output_dir);
    std::fs::create_dir_all(output_dir).unwrap_or_else(|error| {
        eprintln!("failed to create {}: {error}", output_dir.display());
        std::process::exit(1);
    });
    write_packet_file(output_dir, "dashboard-schema.json", dashboard_schema_json());
    write_packet_file(
        output_dir,
        "integration-fixtures.json",
        integration_fixture_manifest_json(),
    );
    write_packet_file(
        output_dir,
        "integration-summary.txt",
        "TERRAIN integration packet\nschema=terrain.dashboard.v1\nfixtures=terrain.integration-fixtures.v1\npolicy_boundary=TERRAIN keeps territory policy local; RLINE/METIS/CROP/PEBBLE/FLETCH are candidate integration surfaces.\n",
    );
    println!(
        "wrote integration packet to {} with 3 files",
        output_dir.display()
    );
}

fn print_audit_for_csv(csv: &str) {
    let territories = parse_territories_csv(csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    print_audit(&territories);
}

fn print_audit(territories: &[terrain_core::Territory]) {
    let audit = audit_territories(territories, 0.05, 0.05);
    println!("TERRAIN balance audit");
    println!(
        "status={} demand_spread={:.3} revenue_spread={:.3} max_radius_deg={:.3}",
        if audit.passes { "pass" } else { "review" },
        audit.demand_spread_ratio,
        audit.revenue_spread_ratio,
        audit.max_radius_degrees,
    );
    println!("territory,sites,demand,revenue,centroid_lat,centroid_lon,assignees");
    for summary in audit.summaries {
        println!(
            "{},{},{:.1},{:.0},{:.4},{:.4},{}",
            summary.territory_id,
            summary.site_count,
            summary.demand,
            summary.revenue,
            summary.centroid_latitude,
            summary.centroid_longitude,
            summary.assignees.join(";"),
        );
    }
}

fn print_product_balance_for_csv(csv: &str) {
    let territories = parse_territories_csv(csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let audit = audit_product_balance(&territories, 0.25);
    println!(
        "status={} overall_product_spread={:.3}",
        if audit.passes { "pass" } else { "review" },
        audit.overall_spread_score,
    );
    println!("product,min_demand,max_demand,spread_ratio");
    for balance in audit.balances {
        println!(
            "{},{:.1},{:.1},{:.3}",
            balance.product, balance.min_demand, balance.max_demand, balance.spread_ratio,
        );
    }
}

fn print_diagnostics_for_csv(csv: &str) {
    let diagnostics = diagnose_territories_csv(csv);
    println!(
        "status={} diagnostic_count={}",
        if diagnostics.is_empty() {
            "pass"
        } else {
            "review"
        },
        diagnostics.len()
    );
    println!("severity,line,field,message");
    for diagnostic in diagnostics {
        println!(
            "{},{},{},{}",
            diagnostic.severity,
            diagnostic.line,
            diagnostic.field,
            diagnostic.message.replace(',', ";"),
        );
    }
}

fn run_graph_diagnostics_command(path: Option<&String>, threshold: Option<&String>) {
    let threshold = threshold
        .map(|value| {
            value.parse::<f64>().unwrap_or_else(|_| {
                eprintln!("long edge threshold must be a number");
                std::process::exit(2);
            })
        })
        .unwrap_or(0.10);
    let csv = read_csv_file(path);
    let sites = parse_sites_csv(&csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let report = site_graph_diagnostic_report(&sites, threshold);
    println!(
        "status={} node_count={} edge_count={} component_count={} diagnostic_count={}",
        if report.diagnostics.iter().any(|d| d.severity == "error") {
            "error"
        } else if report.diagnostics.is_empty() {
            "pass"
        } else {
            "review"
        },
        report.node_count,
        report.edge_count,
        report.component_count,
        report.diagnostics.len()
    );
    println!("severity,code,site_ids,message");
    for diagnostic in report.diagnostics {
        println!(
            "{},{},{},{}",
            diagnostic.severity,
            diagnostic.code,
            diagnostic.site_ids.join(";"),
            diagnostic.message.replace(',', ";"),
        );
    }
}

fn run_graph_partition_command(path: Option<&String>, target_count: Option<&String>) {
    let target_count = parse_count_arg(target_count, "missing target territory count");
    let csv = read_csv_file(path);
    let sites = parse_sites_csv(&csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let report = graph_partition_report(&sites, target_count, 0.10, 0.06).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let moved_count = report
        .movements
        .iter()
        .filter(|movement| movement.movement_kind != "unchanged")
        .count();
    println!("TERRAIN graph partition comparison");
    println!(
        "baseline_status={} graph_status={} movement_count={} compactness_exception_count={} component_count={} graph_diagnostic_count={}",
        if report.comparison.baseline.passes {
            "pass"
        } else {
            "review"
        },
        if report.comparison.proposed.passes {
            "pass"
        } else {
            "review"
        },
        moved_count,
        report.compactness_exceptions.len(),
        report.graph_diagnostics.component_count,
        report.graph_diagnostics.diagnostics.len(),
    );
    println!(
        "territory,site_count_delta,demand_delta,revenue_delta,baseline_demand,graph_demand,graph_max_radius_degrees"
    );
    for delta in &report.comparison.territory_deltas {
        let graph_radius = report
            .graph_partition
            .iter()
            .find(|territory| territory.id == delta.territory_id)
            .map(summarize_territory)
            .map_or(0.0, |summary| summary.max_radius_degrees);
        println!(
            "{},{},{:.1},{:.0},{:.1},{:.1},{:.6}",
            delta.territory_id,
            delta.site_count_delta,
            delta.demand_delta,
            delta.revenue_delta,
            delta.baseline_demand,
            delta.proposed_demand,
            graph_radius,
        );
    }
    println!("movement_site_id,baseline_territory,graph_territory,movement_kind");
    for movement in report.movements {
        println!(
            "{},{},{},{}",
            movement.site_id,
            movement.baseline_territory_id.unwrap_or_default(),
            movement.proposed_territory_id.unwrap_or_default(),
            movement.movement_kind,
        );
    }
    println!("graph_diagnostic_severity,code,site_ids,message");
    for diagnostic in report.graph_diagnostics.diagnostics {
        println!(
            "{},{},{},{}",
            diagnostic.severity,
            diagnostic.code,
            diagnostic.site_ids.join(";"),
            diagnostic.message.replace(',', ";"),
        );
    }
}

fn print_sample_svg() {
    let svg = render_territory_svg(&sample_territories(), &TerritoryVisualOptions::default());
    println!("{svg}");
}

fn print_sample_geojson() {
    let geojson = render_territory_geojson(&sample_territories());
    println!("{geojson}");
}

fn print_sample_csv() {
    print!("{}", sample_territories_csv());
}

fn print_sample_proposed_csv() {
    print!("{}", sample_proposed_territories_csv());
}

fn print_sample_sites_csv() {
    print!("{}", sample_sites_csv());
}

fn print_sample_capacity_csv() {
    print!("{}", sample_assignee_capacity_csv());
}

fn print_capacity_for_csv(csv: &str) {
    let capacities = parse_assignee_capacity_csv(csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    println!(
        "assignee,team,responsibility,asset_group,capacity,home_latitude,home_longitude,skills"
    );
    for capacity in capacities {
        println!(
            "{},{},{},{},{:.1},{:.4},{:.4},{}",
            capacity.assignee,
            capacity.team,
            capacity.responsibility,
            capacity.asset_group,
            capacity.capacity,
            capacity.home_latitude,
            capacity.home_longitude,
            capacity.skills.join(";"),
        );
    }
}

fn run_capacity_audit_command(territory_path: Option<&String>, capacity_path: Option<&String>) {
    let Some(capacity_path) = capacity_path else {
        eprintln!("missing capacity CSV path");
        print_help();
        std::process::exit(2);
    };
    let territory_csv = read_csv_file(territory_path);
    let capacity_csv = read_csv_file(Some(capacity_path));
    let territories = parse_territories_csv(&territory_csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let capacities = parse_assignee_capacity_csv(&capacity_csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let exceptions = capacity_exceptions(&territories, &capacities);
    println!(
        "status={} exception_count={}",
        if exceptions.is_empty() {
            "pass"
        } else {
            "review"
        },
        exceptions.len(),
    );
    println!("territory,demand,capacity,overload,assignees");
    for exception in exceptions {
        println!(
            "{},{:.1},{:.1},{:.1},{}",
            exception.territory_id,
            exception.demand,
            exception.capacity,
            exception.overload,
            exception.assignees.join(";"),
        );
    }
}

fn print_svg_for_csv(csv: &str) {
    let territories = parse_territories_csv(csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let svg = render_territory_svg(&territories, &TerritoryVisualOptions::default());
    println!("{svg}");
}

fn print_geojson_for_csv(csv: &str) {
    let territories = parse_territories_csv(csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let geojson = render_territory_geojson(&territories);
    println!("{geojson}");
}

fn run_ownership_svg_command(territory_path: Option<&String>, capacity_path: Option<&String>) {
    let (territories, capacities) = read_territories_and_capacities(territory_path, capacity_path);
    let svg = render_territory_svg_with_capacity(
        &territories,
        &capacities,
        &TerritoryVisualOptions::default(),
    );
    println!("{svg}");
}

fn run_ownership_geojson_command(territory_path: Option<&String>, capacity_path: Option<&String>) {
    let (territories, capacities) = read_territories_and_capacities(territory_path, capacity_path);
    let geojson = render_territory_geojson_with_capacity(&territories, &capacities);
    println!("{geojson}");
}

fn read_territories_and_capacities(
    territory_path: Option<&String>,
    capacity_path: Option<&String>,
) -> (
    Vec<terrain_core::Territory>,
    Vec<terrain_core::AssigneeCapacity>,
) {
    let Some(capacity_path) = capacity_path else {
        eprintln!("missing capacity CSV path");
        print_help();
        std::process::exit(2);
    };
    let territory_csv = read_csv_file(territory_path);
    let capacity_csv = read_csv_file(Some(capacity_path));
    let territories = parse_territories_csv(&territory_csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let capacities = parse_assignee_capacity_csv(&capacity_csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    (territories, capacities)
}

fn print_partition_audit(csv: &str, target_count: usize) {
    let territories = partition_csv(csv, target_count);
    print_audit(&territories);
}

fn print_partition_svg(csv: &str, target_count: usize) {
    let territories = partition_csv(csv, target_count);
    let svg = render_territory_svg(&territories, &TerritoryVisualOptions::default());
    println!("{svg}");
}

fn print_partition_geojson(csv: &str, target_count: usize) {
    let territories = partition_csv(csv, target_count);
    let geojson = render_territory_geojson(&territories);
    println!("{geojson}");
}

fn partition_csv(csv: &str, target_count: usize) -> Vec<terrain_core::Territory> {
    let sites = parse_sites_csv(csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    partition_sites(sites, target_count).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    })
}

fn run_csv_command(path: Option<&String>, handler: fn(&str)) {
    let csv = read_csv_file(path);
    handler(&csv);
}

fn read_csv_file(path: Option<&String>) -> String {
    let Some(path) = path else {
        eprintln!("missing CSV path");
        print_help();
        std::process::exit(2);
    };
    std::fs::read_to_string(path).unwrap_or_else(|error| {
        eprintln!("failed to read {path}: {error}");
        std::process::exit(1);
    })
}

fn run_partition_command(
    path: Option<&String>,
    target_count: Option<&String>,
    handler: fn(&str, usize),
) {
    let Some(target_count) = target_count else {
        eprintln!("missing target territory count");
        print_help();
        std::process::exit(2);
    };
    let target_count = target_count.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("target territory count must be a positive integer");
        std::process::exit(2);
    });
    let csv = read_csv_file(path);
    handler(&csv, target_count);
}

fn run_compare_command(baseline_path: Option<&String>, proposed_path: Option<&String>) {
    let Some(proposed_path) = proposed_path else {
        eprintln!("missing proposed CSV path");
        print_help();
        std::process::exit(2);
    };
    let baseline_csv = read_csv_file(baseline_path);
    let proposed_csv = read_csv_file(Some(proposed_path));
    let baseline = parse_territories_csv(&baseline_csv).unwrap_or_else(|error| {
        eprintln!("baseline {error}");
        std::process::exit(1);
    });
    let proposed = parse_territories_csv(&proposed_csv).unwrap_or_else(|error| {
        eprintln!("proposed {error}");
        std::process::exit(1);
    });
    let comparison = compare_territory_plans(&baseline, &proposed, 0.05, 0.05);
    println!("TERRAIN scenario comparison");
    println!(
        "baseline_status={} proposed_status={} demand_spread_delta={:.3} revenue_spread_delta={:.3}",
        if comparison.baseline.passes {
            "pass"
        } else {
            "review"
        },
        if comparison.proposed.passes {
            "pass"
        } else {
            "review"
        },
        comparison.proposed.demand_spread_ratio - comparison.baseline.demand_spread_ratio,
        comparison.proposed.revenue_spread_ratio - comparison.baseline.revenue_spread_ratio,
    );
    println!(
        "territory,site_count_delta,demand_delta,revenue_delta,baseline_demand,proposed_demand"
    );
    for delta in comparison.territory_deltas {
        println!(
            "{},{},{:.1},{:.0},{:.1},{:.1}",
            delta.territory_id,
            delta.site_count_delta,
            delta.demand_delta,
            delta.revenue_delta,
            delta.baseline_demand,
            delta.proposed_demand,
        );
    }
}

fn run_movement_command(baseline_path: Option<&String>, proposed_path: Option<&String>) {
    let Some(proposed_path) = proposed_path else {
        eprintln!("missing proposed CSV path");
        print_help();
        std::process::exit(2);
    };
    let baseline_csv = read_csv_file(baseline_path);
    let proposed_csv = read_csv_file(Some(proposed_path));
    let baseline = parse_territories_csv(&baseline_csv).unwrap_or_else(|error| {
        eprintln!("baseline {error}");
        std::process::exit(1);
    });
    let proposed = parse_territories_csv(&proposed_csv).unwrap_or_else(|error| {
        eprintln!("proposed {error}");
        std::process::exit(1);
    });
    let movements = site_movements(&baseline, &proposed);
    let moved_count = movements
        .iter()
        .filter(|movement| movement.movement_kind != "unchanged")
        .count();
    println!(
        "status=review movement_count={moved_count} site_count={}",
        movements.len()
    );
    println!("site_id,baseline_territory,proposed_territory,movement_kind,demand,revenue");
    for movement in movements {
        println!(
            "{},{},{},{},{:.1},{:.0}",
            movement.site_id,
            movement.baseline_territory_id.unwrap_or_default(),
            movement.proposed_territory_id.unwrap_or_default(),
            movement.movement_kind,
            movement.demand,
            movement.revenue,
        );
    }
}

fn run_compactness_command(path: Option<&String>, threshold: Option<&String>) {
    let Some(threshold) = threshold else {
        eprintln!("missing compactness threshold");
        print_help();
        std::process::exit(2);
    };
    let threshold = threshold.parse::<f64>().unwrap_or_else(|_| {
        eprintln!("compactness threshold must be a number");
        std::process::exit(2);
    });
    let csv = read_csv_file(path);
    let territories = parse_territories_csv(&csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let exceptions = compactness_exceptions(&territories, threshold);
    println!(
        "status={} exception_count={} threshold_degrees={:.3}",
        if exceptions.is_empty() {
            "pass"
        } else {
            "review"
        },
        exceptions.len(),
        threshold,
    );
    println!("territory,sites,max_radius_degrees,threshold_degrees");
    for exception in exceptions {
        println!(
            "{},{},{:.6},{:.6}",
            exception.territory_id,
            exception.site_count,
            exception.max_radius_degrees,
            exception.threshold_degrees,
        );
    }
}

fn run_sweep_command(
    path: Option<&String>,
    min_count: Option<&String>,
    max_count: Option<&String>,
) {
    let min_count = parse_count_arg(min_count, "missing minimum territory count");
    let max_count = parse_count_arg(max_count, "missing maximum territory count");
    let csv = read_csv_file(path);
    let sites = parse_sites_csv(&csv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });
    let sweep =
        partition_count_sweep(&sites, min_count, max_count, 0.50, 0.50).unwrap_or_else(|error| {
            eprintln!("{error}");
            std::process::exit(1);
        });
    println!(
        "status=pass scenario_count={} min_count={} max_count={}",
        sweep.len(),
        min_count.min(max_count),
        min_count.max(max_count),
    );
    println!("target_count,actual_count,passes,demand_spread,revenue_spread,max_radius_degrees");
    for result in sweep {
        println!(
            "{},{},{},{:.3},{:.3},{:.3}",
            result.target_territory_count,
            result.actual_territory_count,
            result.audit.passes,
            result.audit.demand_spread_ratio,
            result.audit.revenue_spread_ratio,
            result.audit.max_radius_degrees,
        );
    }
}

fn parse_count_arg(value: Option<&String>, missing_message: &str) -> usize {
    let Some(value) = value else {
        eprintln!("{missing_message}");
        print_help();
        std::process::exit(2);
    };
    value.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("territory count must be a positive integer");
        std::process::exit(2);
    })
}

fn run_packet_command(
    baseline_path: Option<&String>,
    proposed_path: Option<&String>,
    output_dir: Option<&String>,
) {
    let Some(proposed_path) = proposed_path else {
        eprintln!("missing proposed CSV path");
        print_help();
        std::process::exit(2);
    };
    let Some(output_dir) = output_dir else {
        eprintln!("missing output directory");
        print_help();
        std::process::exit(2);
    };
    let baseline_csv = read_csv_file(baseline_path);
    let proposed_csv = read_csv_file(Some(proposed_path));
    let baseline = parse_territories_csv(&baseline_csv).unwrap_or_else(|error| {
        eprintln!("baseline {error}");
        std::process::exit(1);
    });
    let proposed = parse_territories_csv(&proposed_csv).unwrap_or_else(|error| {
        eprintln!("proposed {error}");
        std::process::exit(1);
    });
    let comparison = compare_territory_plans(&baseline, &proposed, 0.05, 0.05);
    let output_dir = std::path::Path::new(output_dir);
    std::fs::create_dir_all(output_dir).unwrap_or_else(|error| {
        eprintln!("failed to create {}: {error}", output_dir.display());
        std::process::exit(1);
    });
    write_packet_file(
        output_dir,
        "scenario-summary.csv",
        &scenario_summary_csv(&comparison),
    );
    write_packet_file(
        output_dir,
        "territory-deltas.csv",
        &territory_deltas_csv(&comparison),
    );
    write_packet_file(
        output_dir,
        "movement-manifest.csv",
        &movement_manifest_csv(&baseline, &proposed),
    );
    write_packet_file(
        output_dir,
        "baseline-diagnostics.csv",
        &diagnostics_csv(&baseline_csv),
    );
    write_packet_file(
        output_dir,
        "proposed-diagnostics.csv",
        &diagnostics_csv(&proposed_csv),
    );
    write_packet_file(
        output_dir,
        "compactness-exceptions.csv",
        &compactness_exceptions_csv(&proposed, 0.06),
    );
    write_packet_file(
        output_dir,
        "proposed.svg",
        &render_territory_svg(&proposed, &TerritoryVisualOptions::default()),
    );
    write_packet_file(
        output_dir,
        "proposed.geojson",
        &render_territory_geojson(&proposed),
    );
    println!(
        "wrote scenario packet to {} with 8 files",
        output_dir.display()
    );
}

fn run_field_review_command(baseline_path: Option<&String>, proposed_path: Option<&String>) {
    let Some(proposed_path) = proposed_path else {
        eprintln!("missing proposed CSV path");
        print_help();
        std::process::exit(2);
    };
    let baseline_csv = read_csv_file(baseline_path);
    let proposed_csv = read_csv_file(Some(proposed_path));
    let baseline = parse_territories_csv(&baseline_csv).unwrap_or_else(|error| {
        eprintln!("baseline {error}");
        std::process::exit(1);
    });
    let proposed = parse_territories_csv(&proposed_csv).unwrap_or_else(|error| {
        eprintln!("proposed {error}");
        std::process::exit(1);
    });
    let comparison = compare_territory_plans(&baseline, &proposed, 0.05, 0.05);
    let movements = site_movements(&baseline, &proposed);
    let moved_sites = movements
        .iter()
        .filter(|movement| movement.movement_kind != "unchanged")
        .collect::<Vec<_>>();
    let compactness = compactness_exceptions(&proposed, 0.06);

    println!("TERRAIN field review");
    println!(
        "Recommendation: {}",
        if comparison.proposed.passes && compactness.is_empty() {
            "ready for field review"
        } else {
            "needs manager review"
        }
    );
    println!(
        "Balance: proposed plan is {} with demand spread {:.1}% and revenue spread {:.1}%.",
        if comparison.proposed.passes {
            "within threshold"
        } else {
            "outside threshold"
        },
        comparison.proposed.demand_spread_ratio * 100.0,
        comparison.proposed.revenue_spread_ratio * 100.0,
    );
    println!(
        "Movement: {} of {} sites change territory.",
        moved_sites.len(),
        movements.len()
    );
    for movement in moved_sites {
        println!(
            "- {} moves from {} to {} ({:.1} demand, ${:.0} revenue).",
            movement.site_id,
            movement
                .baseline_territory_id
                .as_deref()
                .unwrap_or("unassigned"),
            movement
                .proposed_territory_id
                .as_deref()
                .unwrap_or("unassigned"),
            movement.demand,
            movement.revenue,
        );
    }
    println!(
        "Compactness: {} territories exceed the review radius.",
        compactness.len()
    );
    for exception in compactness {
        println!(
            "- {} has max radius {:.3} degrees across {} sites.",
            exception.territory_id, exception.max_radius_degrees, exception.site_count,
        );
    }
}

fn run_fairness_packet_command(
    territory_path: Option<&String>,
    capacity_path: Option<&String>,
    output_dir: Option<&String>,
) {
    let Some(output_dir) = output_dir else {
        eprintln!("missing output directory");
        print_help();
        std::process::exit(2);
    };
    let (territories, capacities) = read_territories_and_capacities(territory_path, capacity_path);
    let output_dir = std::path::Path::new(output_dir);
    std::fs::create_dir_all(output_dir).unwrap_or_else(|error| {
        eprintln!("failed to create {}: {error}", output_dir.display());
        std::process::exit(1);
    });
    write_packet_file(
        output_dir,
        "capacity-roster.csv",
        &capacity_roster_csv(&capacities),
    );
    write_packet_file(
        output_dir,
        "capacity-overloads.csv",
        &capacity_overloads_csv(&territories, &capacities),
    );
    write_packet_file(
        output_dir,
        "ownership.svg",
        &render_territory_svg_with_capacity(
            &territories,
            &capacities,
            &TerritoryVisualOptions::default(),
        ),
    );
    write_packet_file(
        output_dir,
        "ownership.geojson",
        &render_territory_geojson_with_capacity(&territories, &capacities),
    );
    println!(
        "wrote fairness packet to {} with 4 files",
        output_dir.display()
    );
}

fn write_packet_file(output_dir: &std::path::Path, file_name: &str, contents: &str) {
    let path = output_dir.join(file_name);
    std::fs::write(&path, contents).unwrap_or_else(|error| {
        eprintln!("failed to write {}: {error}", path.display());
        std::process::exit(1);
    });
}

fn scenario_summary_csv(comparison: &terrain_core::ScenarioComparison) -> String {
    format!(
        "metric,baseline,proposed,delta\n\
passes,{},{},{}\n\
demand_spread_ratio,{:.6},{:.6},{:.6}\n\
revenue_spread_ratio,{:.6},{:.6},{:.6}\n\
max_radius_degrees,{:.6},{:.6},{:.6}\n",
        comparison.baseline.passes,
        comparison.proposed.passes,
        comparison.proposed.passes as i32 - comparison.baseline.passes as i32,
        comparison.baseline.demand_spread_ratio,
        comparison.proposed.demand_spread_ratio,
        comparison.proposed.demand_spread_ratio - comparison.baseline.demand_spread_ratio,
        comparison.baseline.revenue_spread_ratio,
        comparison.proposed.revenue_spread_ratio,
        comparison.proposed.revenue_spread_ratio - comparison.baseline.revenue_spread_ratio,
        comparison.baseline.max_radius_degrees,
        comparison.proposed.max_radius_degrees,
        comparison.proposed.max_radius_degrees - comparison.baseline.max_radius_degrees,
    )
}

fn territory_deltas_csv(comparison: &terrain_core::ScenarioComparison) -> String {
    let mut csv = String::from(
        "territory,baseline_sites,proposed_sites,site_count_delta,baseline_demand,proposed_demand,demand_delta,baseline_revenue,proposed_revenue,revenue_delta\n",
    );
    for delta in &comparison.territory_deltas {
        csv.push_str(&format!(
            "{},{},{},{},{:.1},{:.1},{:.1},{:.0},{:.0},{:.0}\n",
            delta.territory_id,
            delta.baseline_site_count,
            delta.proposed_site_count,
            delta.site_count_delta,
            delta.baseline_demand,
            delta.proposed_demand,
            delta.demand_delta,
            delta.baseline_revenue,
            delta.proposed_revenue,
            delta.revenue_delta,
        ));
    }
    csv
}

fn movement_manifest_csv(
    baseline: &[terrain_core::Territory],
    proposed: &[terrain_core::Territory],
) -> String {
    let mut csv = String::from(
        "site_id,baseline_territory,proposed_territory,movement_kind,demand,revenue\n",
    );
    for movement in site_movements(baseline, proposed) {
        csv.push_str(&format!(
            "{},{},{},{},{:.1},{:.0}\n",
            movement.site_id,
            movement.baseline_territory_id.unwrap_or_default(),
            movement.proposed_territory_id.unwrap_or_default(),
            movement.movement_kind,
            movement.demand,
            movement.revenue,
        ));
    }
    csv
}

fn diagnostics_csv(csv_input: &str) -> String {
    let mut csv = String::from("severity,line,field,message\n");
    for diagnostic in diagnose_territories_csv(csv_input) {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            diagnostic.severity,
            diagnostic.line,
            diagnostic.field,
            diagnostic.message.replace(',', ";"),
        ));
    }
    csv
}

fn compactness_exceptions_csv(territories: &[terrain_core::Territory], threshold: f64) -> String {
    let mut csv = String::from("territory,sites,max_radius_degrees,threshold_degrees\n");
    for exception in compactness_exceptions(territories, threshold) {
        csv.push_str(&format!(
            "{},{},{:.6},{:.6}\n",
            exception.territory_id,
            exception.site_count,
            exception.max_radius_degrees,
            exception.threshold_degrees,
        ));
    }
    csv
}

fn capacity_roster_csv(capacities: &[terrain_core::AssigneeCapacity]) -> String {
    let mut csv = String::from(
        "assignee,team,responsibility,asset_group,capacity,home_latitude,home_longitude,skills\n",
    );
    for capacity in capacities {
        csv.push_str(&format!(
            "{},{},{},{},{:.1},{:.4},{:.4},{}\n",
            capacity.assignee,
            capacity.team,
            capacity.responsibility,
            capacity.asset_group,
            capacity.capacity,
            capacity.home_latitude,
            capacity.home_longitude,
            capacity.skills.join(";"),
        ));
    }
    csv
}

fn capacity_overloads_csv(
    territories: &[terrain_core::Territory],
    capacities: &[terrain_core::AssigneeCapacity],
) -> String {
    let mut csv = String::from("territory,demand,capacity,overload,assignees\n");
    for exception in capacity_exceptions(territories, capacities) {
        csv.push_str(&format!(
            "{},{:.1},{:.1},{:.1},{}\n",
            exception.territory_id,
            exception.demand,
            exception.capacity,
            exception.overload,
            exception.assignees.join(";"),
        ));
    }
    csv
}
