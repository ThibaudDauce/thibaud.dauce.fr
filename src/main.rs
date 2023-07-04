use chrono::prelude::*;
use copy_dir::copy_dir;
use geoutils::Location;
use gpx::read;
use gpx::Waypoint;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
use syntect::highlighting::ThemeSet;
use syntect::html::css_for_theme_with_class_style;
use syntect::html::ClassStyle;
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;
use tera::Context;
use tera::Tera;
use yaml_rust::YamlLoader;

#[derive(Serialize, Debug)]
struct Content {
    filename: String,
    url: String,

    title: String,
    description: String,

    markdown: String,
    html: String,

    date: chrono::DateTime<Utc>,
    date_fr: String,
    date_en: String,
    date_rss: String,

    lang: String,

    slides: Option<String>,
    video: Option<String>,
    tweet: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
enum TracesType {
    Bike,
    Hike,
}

#[derive(Serialize, Debug)]
struct Trip {
    r#type: TracesType,
    type_as_string: &'static str,
    color: &'static str,
    date: String,
    name: String,
    distance: f64,
    distance_format: String,
    elevation_positive: f64,
    elevation_positive_format: String,
    elevation_negative: f64,
    elevation_negative_format: String,
    traces: Vec<Trace>,
}

#[derive(Serialize, Debug)]
struct Trace {
    path: String,
    name: String,
    distance: f64,
    distance_format: String,
    elevation_positive: f64,
    elevation_positive_format: String,
    elevation_negative: f64,
    elevation_negative_format: String,
}

fn main() {
    let start = Instant::now();

    fs::remove_dir_all("./build/posts").ok();
    fs::remove_dir_all("./build/images").ok();
    fs::remove_dir_all("./build/talks").ok();
    fs::remove_dir_all("./build/traces").ok();
    fs::remove_dir_all("./build/videos").ok();
    fs::remove_dir_all("./build/css/files").ok();

    let mut posts = get_contents("posts");
    let talks = get_contents("talks");

    let tera = Tera::new("content/templates/**/*.html")
        .unwrap_or_else(|e| panic!("Parsing error(s): {}", e));

    fs::create_dir("./build").ok();
    fs::create_dir("./build/css").ok();
    fs::create_dir("./build/posts").ok();

    let markdown_regexp = Regex::new(r#"```([a-z]*)\n((.|\s)*?)\n```"#).unwrap();
    let html_regexp =
        Regex::new(r#"<pre><code( class="language-([a-z]*)")?>((.|\s)*?)</code></pre>"#).unwrap();
    let ps = SyntaxSet::load_defaults_newlines();

    let theme = ThemeSet::get_theme("./content/css/verdandi.tmTheme").unwrap();

    let css =
        css_for_theme_with_class_style(&theme, ClassStyle::SpacedPrefixed { prefix: "syntect" });
    fs::write("./content/css/syntax.css", css).unwrap();

    for post in &posts {
        let mut context = Context::new();
        context.insert("post", &post);
        context.insert("title", &post.title);
        context.insert("blog", &true);
        context.insert("description", &post.description);
        let mut post_html = tera.render("post.html", &context).unwrap();

        let number_of_codes = markdown_regexp.captures_iter(&post.markdown).count();
        for i in 0..number_of_codes {
            let found_in_markdown = markdown_regexp
                .captures_iter(&post.markdown)
                .nth(i)
                .unwrap();
            let found_in_html = html_regexp.captures_iter(&post_html).nth(i).unwrap();

            let lang = found_in_markdown.get(1).unwrap().as_str();

            if lang == "" {
                continue;
            }

            let code_in_markdown = found_in_markdown.get(2).unwrap();
            let code_in_html = found_in_html.get(3).unwrap();

            let syntax = ps
                .find_syntax_by_extension(lang)
                .expect(&format!("Cannot find {lang}"));

            let mut code = code_in_markdown.as_str().to_string();
            let mut without_php_opening = false;

            // Add fake opening PHP tags if missing
            if !code.starts_with("<?php") && lang == "php" {
                without_php_opening = true;
                code = format!("<?php\n{}", code);
            }

            // Add fake module opening if lang is Haskell
            if lang == "hs" {
                code = format!("module Example where\n\n{}", code);
            }

            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                &syntax,
                &ps,
                ClassStyle::SpacedPrefixed { prefix: "syntect" },
            );
            for line in code.lines() {
                html_generator.parse_html_for_line(&line);
            }
            let mut output_html = html_generator.finalize();

            if without_php_opening {
                let end_of_first_line = output_html.find("\n").unwrap() + 1;
                output_html.replace_range(..end_of_first_line, "");
            }

            let range = code_in_html.range();
            post_html.replace_range(range, &output_html);
        }

        fs::write(format!("./build{}", post.url), post_html).unwrap();
    }

    posts = posts
        .into_iter()
        .filter(|post| post.date < chrono::Utc::now())
        .collect();

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("title", "Mes derniers articles");
    context.insert("blog", &true);
    let index = tera.render("index.html", &context).unwrap();
    fs::write("build/index.html", index).unwrap();

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("latest_post", &posts.last().unwrap());
    let index = tera.render("rss.xml.html", &context).unwrap();
    fs::write("build/feed.xml", index).unwrap();

    let mut context = Context::new();
    context.insert("title", "À propos");
    context.insert("about", &true);
    let about = tera.render("about.html", &context).unwrap();
    fs::write("build/about.html", about).unwrap();

    let mut context = Context::new();
    context.insert("title", "Mes dernières conférences");
    context.insert("talks", &talks);
    let talks = tera.render("talks.html", &context).unwrap();
    fs::write("build/talks.html", talks).unwrap();

    let mut context = Context::new();
    context.insert("title", "Mes dernières randonnées à pied et à vélo");

    context.insert("trips", &load_trips());
    context.insert("hiking", &true);
    let hiking = tera.render("hiking.html", &context).unwrap();
    fs::write("build/hiking.html", hiking).unwrap();

    copy_dir("./content/images", "./build/images").unwrap();
    copy_dir("./content/videos", "./build/videos").unwrap();
    copy_dir("./content/traces", "./build/traces").unwrap();
    copy_dir("./content/talks", "./build/talks").unwrap();
    copy_dir(
        "./node_modules/typeface-merriweather/files",
        "./build/css/files",
    )
    .unwrap();

    println!("End of build: {:?}", start.elapsed());
}

fn get_contents(directory: &str) -> Vec<Content> {
    let mut contents: Vec<Content> = fs::read_dir(format!("content/{}", directory))
        .unwrap()
        .map(|maybe_path| maybe_path.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| get_content(directory, &path))
        .collect();
    contents.sort_by_key(|content| content.filename.clone());
    contents
}

fn get_content(prefix: &str, path: &Path) -> Content {
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();

    let date: Vec<&str> = filename.split("-").collect();

    let content = fs::read_to_string(path.clone()).unwrap();
    let mut blocks: Vec<&str> = content.split("---").collect();
    blocks.remove(0); // remove first ---
    let metadata = &YamlLoader::load_from_str(blocks.first().unwrap()).unwrap()[0];
    blocks.remove(0);
    let markdown = blocks.join("---").to_string();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown, options);

    let mut html = String::new();
    html::push_html(&mut html, parser);

    Content {
        filename: filename.clone(),
        url: format!("/{}/{}", prefix, filename.replace(".md", ".html")),
        title: metadata["title"].as_str().unwrap().to_string(),
        description: metadata["description"].as_str().unwrap_or("").to_string(),
        markdown: markdown,
        html: html,

        date: Utc
            .ymd(
                date[0].parse().unwrap(),
                date[1].trim_start_matches('0').parse().unwrap(),
                date[2].trim_start_matches('0').parse().unwrap(),
            )
            .and_hms(0, 0, 0),
        date_fr: format!(
            "{} {} {}",
            date[2].trim_start_matches('0'),
            french_months(date[1]),
            date[0]
        ),
        date_en: format!(
            "{} {}, {}",
            english_months(date[1]),
            date[2].trim_start_matches('0'),
            date[0]
        ),
        date_rss: Utc
            .ymd(
                date[0].parse().unwrap(),
                date[1].trim_start_matches('0').parse().unwrap(),
                date[2].trim_start_matches('0').parse().unwrap(),
            )
            .and_hms(0, 0, 0)
            .to_rfc2822(),

        lang: metadata["lang"].as_str().unwrap_or("fr").to_string(),

        slides: metadata["slides"].as_str().map(|s| s.to_string()),
        video: metadata["video"].as_str().map(|s| s.to_string()),
        tweet: metadata["tweet"].as_str().map(|s| s.to_string()),
    }
}

fn load_trips() -> Vec<Trip> {
    let mut trips = vec![];

    for trip_dir in fs::read_dir("content/traces").unwrap() {
        let trip_dir = trip_dir.unwrap();
        let trip_path = trip_dir.path();
        let trip_filename = trip_path.file_name().unwrap().to_string_lossy();
        let (date, name) = trip_filename.split_once(' ').unwrap();

        let r#type = if name.ends_with("[bike]") {
            TracesType::Bike
        } else {
            TracesType::Hike
        };

        // if r#type == TracesType::Hike {
        //     continue;
        // }

        let (name, _) = name.split_once('[').unwrap();

        let (year, month) = date.split_once('-').unwrap();

        let mut trip = Trip {
            r#type,
            type_as_string: if r#type == TracesType::Bike {
                "Vélo"
            } else {
                "Randonnée"
            },
            date: format!("{} {year}", french_months(month)),
            color: if r#type == TracesType::Bike {
                "#2563eb"
            } else {
                "#ea580c"
            },
            name: name.trim().to_string(),
            distance: 0.0,
            distance_format: "".to_string(),
            elevation_negative: 0.0,
            elevation_negative_format: "".to_string(),
            elevation_positive: 0.0,
            elevation_positive_format: "".to_string(),
            traces: vec![],
        };

        for trace_dir in fs::read_dir(trip_dir.path()).unwrap() {
            let trace_dir = trace_dir.unwrap();
            let trace_filename = trace_dir.file_name();
            let trace_filename_as_string = trace_filename.to_string_lossy().to_string();

            let file = File::open(trace_dir.path()).unwrap();
            let reader = BufReader::new(file);

            let mut trace = Trace {
                path: format!("{trip_filename}/{trace_filename_as_string}"),
                name: trace_filename_as_string
                    .strip_suffix(".gpx")
                    .unwrap()
                    .to_string(),
                distance: 0.0,
                distance_format: "".to_string(),
                elevation_negative: 0.0,
                elevation_negative_format: "".to_string(),
                elevation_positive: 0.0,
                elevation_positive_format: "".to_string(),
            };

            // read takes any io::Read and gives a Result<Gpx, Error>.
            let gpx = read(reader).unwrap();

            let mut previous_point: Option<&Waypoint> = None;

            for track in &gpx.tracks {
                for segment in &track.segments {
                    for point in &segment.points {
                        if point.hdop.unwrap() > 10.0 {
                            continue;
                        }
                        if let Some(previous_point) = previous_point {
                            let start = Location::new(
                                previous_point.point().lat(),
                                previous_point.point().lng(),
                            );
                            let end = Location::new(point.point().lat(), point.point().lng());
                            let distance = start.distance_to(&end).unwrap();
                            if distance.meters() < 5.0 {
                                continue;
                            }

                            trace.distance += distance.meters();
                            trip.distance += distance.meters();

                            let elevation =
                                point.elevation.unwrap() - previous_point.elevation.unwrap();

                            if elevation < -0.5 || elevation > 0.5 {
                                if elevation > 0.0 {
                                    trace.elevation_positive += elevation;
                                    trip.elevation_positive += elevation;
                                } else {
                                    trace.elevation_negative += elevation * -1.0;
                                    trip.elevation_negative += elevation * -1.0;
                                }
                            }
                        }

                        previous_point = Some(point);
                    }
                }
            }

            trace.distance_format = format_meters(trace.distance);
            trace.elevation_positive_format = format_meters(trace.elevation_positive);
            trace.elevation_negative_format = format_meters(trace.elevation_negative);

            trip.traces.push(trace);
        }

        trip.distance_format = format_meters(trip.distance);
        trip.elevation_positive_format = format_meters(trip.elevation_positive);
        trip.elevation_negative_format = format_meters(trip.elevation_negative);

        trips.push(trip);
    }

    // let mut traces: Vec<String> =
    //     .unwrap()
    //     .map(|maybe_path| {
    //         dbg!(&maybe_path);
    //         maybe_path
    //             .unwrap()
    //             .path()
    //             .file_name()
    //             .unwrap()
    //             .to_str()
    //             .unwrap()
    //             .to_string()
    //     })
    //     .collect();
    // traces.sort();

    trips
}

fn format_meters(distance: f64) -> String {
    if distance > 1500.0 {
        format!("{:.2}km", distance / 1000.0)
    } else {
        format!("{distance:.2}m")
    }
}

fn french_months(month: &str) -> String {
    (match month {
        "01" => "janvier",
        "02" => "février",
        "03" => "mars",
        "04" => "avril",
        "05" => "mai",
        "06" => "juin",
        "07" => "juillet",
        "08" => "août",
        "09" => "septembre",
        "10" => "octobre",
        "11" => "novembre",
        "12" => "décembre",
        _ => panic!(),
    })
    .to_string()
}
fn english_months(month: &str) -> String {
    (match month {
        "01" => "January",
        "02" => "February",
        "03" => "March",
        "04" => "April",
        "05" => "May",
        "06" => "June",
        "07" => "July",
        "08" => "August",
        "09" => "September",
        "10" => "Obctober",
        "11" => "November",
        "12" => "December",
        _ => panic!(),
    })
    .to_string()
}
