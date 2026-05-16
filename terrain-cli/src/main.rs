use terrain_core::{
    TerritoryVisualOptions, audit_territories, compactness_exceptions, compare_territory_plans,
    diagnose_territories_csv, parse_sites_csv, parse_territories_csv, partition_sites,
    render_territory_geojson, render_territory_svg, sample_proposed_territories_csv,
    sample_sites_csv, sample_territories, sample_territories_csv,
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
        "audit-csv" => run_csv_command(args.get(1), print_audit_for_csv),
        "diagnose-csv" => run_csv_command(args.get(1), print_diagnostics_for_csv),
        "compare-csv" => run_compare_command(args.get(1), args.get(2)),
        "compactness-csv" => run_compactness_command(args.get(1), args.get(2)),
        "svg-csv" => run_csv_command(args.get(1), print_svg_for_csv),
        "geojson-csv" => run_csv_command(args.get(1), print_geojson_for_csv),
        "partition-csv" => run_partition_command(args.get(1), args.get(2), print_partition_audit),
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
    println!("  audit-csv PATH Audit a territory CSV file");
    println!("  diagnose-csv PATH Report territory CSV intake diagnostics");
    println!("  compare-csv BASELINE PROPOSED Compare two territory CSV plans");
    println!("  compactness-csv PATH THRESHOLD Report max-radius compactness exceptions");
    println!("  svg-csv PATH   Emit a data-bound SVG split from a CSV file");
    println!("  geojson-csv PATH Emit a data-bound GeoJSON split from a CSV file");
    println!("  partition-csv PATH COUNT Audit a deterministic partition from site rows");
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
