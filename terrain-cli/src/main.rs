use terrain_core::{audit_territories, sample_territories};

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
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!("terrain — balanced territory planning");
    println!();
    println!("Commands:");
    println!("  sample-audit   Run the built-in territory balance audit fixture");
}

fn print_sample_audit() {
    let audit = audit_territories(&sample_territories(), 0.05, 0.05);
    println!("TERRAIN sample balance audit");
    println!(
        "status={} demand_spread={:.3} revenue_spread={:.3} max_radius_deg={:.3}",
        if audit.passes { "pass" } else { "review" },
        audit.demand_spread_ratio,
        audit.revenue_spread_ratio,
        audit.max_radius_degrees,
    );
    println!("territory,sites,demand,revenue,centroid_lat,centroid_lon");
    for summary in audit.summaries {
        println!(
            "{},{},{:.1},{:.0},{:.4},{:.4}",
            summary.territory_id,
            summary.site_count,
            summary.demand,
            summary.revenue,
            summary.centroid_latitude,
            summary.centroid_longitude,
        );
    }
}
