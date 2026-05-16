use terrain_core::{
    TerritoryVisualOptions, audit_territories, compactness_exceptions, compare_territory_plans,
    diagnose_territories_csv, parse_assignee_capacity_csv, parse_sites_csv, parse_territories_csv,
    partition_count_sweep, partition_sites, render_territory_geojson, render_territory_svg,
    sample_assignee_capacity_csv, sample_proposed_territories_csv, sample_sites_csv,
    sample_territories, sample_territories_csv, site_movements,
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
        "sample-svg" => print_sample_svg(),
        "sample-geojson" => print_sample_geojson(),
        "sample-csv" => print_sample_csv(),
        "sample-proposed-csv" => print_sample_proposed_csv(),
        "sample-sites-csv" => print_sample_sites_csv(),
        "sample-capacity-csv" => print_sample_capacity_csv(),
        "audit-csv" => run_csv_command(args.get(1), print_audit_for_csv),
        "capacity-csv" => run_csv_command(args.get(1), print_capacity_for_csv),
        "diagnose-csv" => run_csv_command(args.get(1), print_diagnostics_for_csv),
        "compare-csv" => run_compare_command(args.get(1), args.get(2)),
        "movement-csv" => run_movement_command(args.get(1), args.get(2)),
        "compactness-csv" => run_compactness_command(args.get(1), args.get(2)),
        "packet-csv" => run_packet_command(args.get(1), args.get(2), args.get(3)),
        "field-review-csv" => run_field_review_command(args.get(1), args.get(2)),
        "svg-csv" => run_csv_command(args.get(1), print_svg_for_csv),
        "geojson-csv" => run_csv_command(args.get(1), print_geojson_for_csv),
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
    println!("  sample-audit   Run the built-in territory balance audit fixture");
    println!("  sample-svg     Emit a data-bound SVG territory split fixture");
    println!("  sample-geojson Emit a data-bound GeoJSON territory split fixture");
    println!("  sample-csv     Emit the built-in CSV intake fixture");
    println!("  sample-proposed-csv Emit a proposed-plan CSV fixture for comparison");
    println!("  sample-sites-csv Emit the built-in unassigned site CSV fixture");
    println!("  sample-capacity-csv Emit the built-in assignee capacity fixture");
    println!("  audit-csv PATH Audit a territory CSV file");
    println!("  capacity-csv PATH Summarize assignee capacity CSV");
    println!("  diagnose-csv PATH Report territory CSV intake diagnostics");
    println!("  compare-csv BASELINE PROPOSED Compare two territory CSV plans");
    println!("  movement-csv BASELINE PROPOSED List stable site movement between plans");
    println!("  compactness-csv PATH THRESHOLD Report max-radius compactness exceptions");
    println!("  packet-csv BASELINE PROPOSED OUT_DIR Write a scenario review packet");
    println!("  field-review-csv BASELINE PROPOSED Emit a plain-language field review");
    println!("  svg-csv PATH   Emit a data-bound SVG split from a CSV file");
    println!("  geojson-csv PATH Emit a data-bound GeoJSON split from a CSV file");
    println!("  partition-csv PATH COUNT Audit a deterministic partition from site rows");
    println!("  sweep-csv PATH MIN MAX Compare deterministic partition counts");
    println!("  partition-svg-csv PATH COUNT Emit SVG for a deterministic partition");
    println!("  partition-geojson-csv PATH COUNT Emit GeoJSON for a deterministic partition");
}

fn print_sample_audit() {
    print_audit(&sample_territories());
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
    println!("assignee,team,capacity,home_latitude,home_longitude,skills");
    for capacity in capacities {
        println!(
            "{},{},{:.1},{:.4},{:.4},{}",
            capacity.assignee,
            capacity.team,
            capacity.capacity,
            capacity.home_latitude,
            capacity.home_longitude,
            capacity.skills.join(";"),
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
