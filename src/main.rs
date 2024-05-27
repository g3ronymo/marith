use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
    extract::Query,
    http::StatusCode,
    extract::State,
};
use clap::Parser;

use marith::{Config, Operator};

#[derive(Parser)]
struct Args {
    /// Path to template file.
    template_path: String,
    /// Port number to use. 
    #[arg(short, default_value_t = 8014)]
    port: u16,
    /// Use ipv6 instead of the default ipv4
    #[arg(short='6')]
    ipv6: bool,
    /// Listen to all available addresses (by default listen only to localhost).
    #[arg(short)]
    all: bool
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // get socket address
    let ip_addr = if args.ipv6 {
        if args.all {
            // same as IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0))
            IpAddr::V6(Ipv6Addr::UNSPECIFIED)
        } else {
            // same as IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
            IpAddr::V6(Ipv6Addr::LOCALHOST)
        }
    } else if args.all {
        // same as IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    } else {
        // same as IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    };
    let socket_address = SocketAddr::new(ip_addr, args.port);


    // init template
    let err_msg = format!("Failed to open {}", &args.template_path);
    let template = std::fs::read_to_string(args.template_path).expect(&err_msg);

    let app = Router::new()
        .route("/", get(handle_get))
        .with_state(template);
    // run our app with hyper, listening locally on port 3000
    let listener = tokio::net::TcpListener::bind(socket_address)
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_get(State(mut template): State<String>, Query(params): Query<HashMap<String, String>>) 
    -> impl IntoResponse {
    // build config from query
    let mut config = Config::default();
    // variable_num
    if let Some(s) = params.get("variableNum") {
        if let Ok(i) = s.parse::<u8>() {
            config.variable_num = i;
        }
    }
    // variable_range
    if let Some(min) = params.get("variableMinValue") {
        if let Ok(min) = min.parse::<i32>() {
            if let Some(max) = params.get("variableMaxValue") {
                if let Ok(max) = max.parse::<i32>() {
                    if min < max {
                        config.variable_range = min..=max;

                    }
                }
            }
        }
    }
    // operators
    if params.get("addition").is_some() {
        config.operators.push(Operator::Addition);
    }
    if params.get("subtraction").is_some() {
        config.operators.push(Operator::Subtraction);
    }
    if params.get("multiplication").is_some() {
        config.operators.push(Operator::Multiplication);
    }
    if params.get("division").is_some() {
        config.operators.push(Operator::Division);
    }
    // variable_decimal_points
    if let Some(s) = params.get("variableDecimalPoints") {
        if let Ok(i) = s.parse::<u8>() {
            config.variable_decimal_points = i;
        }
    }
    // result_decimal_points 
    if let Some(s) = params.get("resultDecimalPoints") {
        if let Ok(i) = s.parse::<u8>() {
            config.result_decimal_points = i;
        }
    }
    // num_tasks
    if let Some(s) = params.get("numTasks") {
        if let Ok(i) = s.parse::<u8>() {
            config.num_tasks = i;
        }
    }

    if !config.is_valid() {
        config = Config::default();
    }
    let tasks = config.generate_new_tasks();

    // fill template
    template = template.replace("{%variableNum%}", 
        config.variable_num.to_string().as_str());
    template = template.replace("{%variableMinValue%}", 
        config.variable_range.start().to_string().as_str());
    template = template.replace("{%variableMaxValue%}", 
        config.variable_range.end().to_string().as_str());
    let mut addition_checked = "";
    let mut subtraction_checked = "";
    let mut multiplication_checked = "";
    let mut division_checked = "";
    for op in config.operators {
        match op {
            Operator::Addition => addition_checked = "checked",
            Operator::Subtraction => subtraction_checked = "checked",
            Operator::Multiplication => multiplication_checked = "checked",
            Operator::Division => division_checked = "checked",
        }
    }
    template = template.replace("{%addition%}", addition_checked);
    template = template.replace("{%subtraction%}", subtraction_checked);
    template = template.replace("{%multiplication%}", multiplication_checked);
    template = template.replace("{%division%}", division_checked);

    template = template.replace("{%variableDecimalPoints%}", 
        config.variable_decimal_points.to_string().as_str());
    template = template.replace("{%resultDecimalPoints%}", 
        config.result_decimal_points.to_string().as_str());
    template = template.replace("{%numTasks%}", 
        config.num_tasks.to_string().as_str());

    let mut tasks_string = String::new();
    let mut correct_answers = String::new();
    for t in tasks {
        tasks_string.push('"');
        tasks_string.push_str(&t.task_string);
        tasks_string.push('"');
        tasks_string.push(',');

        correct_answers.push_str(t.result.to_string().as_str());
        correct_answers.push(',');
    }
    template = template.replace("{%tasks%}", &tasks_string);
    template = template.replace("{%correctAnswers%}", &correct_answers);
    let headers = [
        ("Content-Type", "text/html; charset=utf-8")
    ];
    (StatusCode::OK, headers, template)
}
