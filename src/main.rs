use tera::Tera;
use tera::Context;
use std::process::exit;
use std::fs;
use std::path::Path;
use pulldown_cmark::{Parser, Options, html};
use serde::Serialize;
use copy_dir::copy_dir;
use yaml_rust::YamlLoader;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::html::ClassedHTMLGenerator;
use regex::Regex;
use syntect::html::ClassStyle;
use syntect::html::css_for_theme_with_class_style;
use chrono::prelude::*;

#[derive(Serialize, Debug)]
struct Content {
    filename: String,
    url: String,

    title: String,
    description: String,
    
    markdown: String,
    html: String,
    // html_preview: String,

    date_fr: String,
    date_en: String,
    date_rss: String,

    slides: Option<String>,
    video: Option<String>,
    tweet: Option<String>,
}

fn main() {
    fs::remove_dir_all("./build/posts").ok();
    fs::remove_dir_all("./build/images").ok();
    fs::remove_dir_all("./build/talks").ok();
    fs::remove_dir_all("./build/traces").ok();
    fs::remove_dir_all("./build/videos").ok();

    let posts = get_contents("posts");
    let talks = get_contents("talks");
    let mut traces: Vec<String> = fs::read_dir("content/traces").unwrap()
        .map(|maybe_path| maybe_path.unwrap().path().file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    traces.sort();

    let tera = match Tera::new("content/templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            exit(1);
        }
    };

    fs::create_dir("./build/posts").unwrap();
    let markdown_regexp = Regex::new(r#"```([a-z]*)\n((.|\s)*?)\n```"#).unwrap();
    let html_regexp = Regex::new(r#"<pre><code( class="language-([a-z]*)")?>((.|\s)*?)</code></pre>"#).unwrap();
    let ps = SyntaxSet::load_defaults_newlines();

    let theme = ThemeSet::get_theme("./content/css/verdandi.tmTheme").unwrap();

    let css = css_for_theme_with_class_style(&theme, ClassStyle::SpacedPrefixed { prefix: "syntect" });
    fs::write("./content/css/syntax.css", css).unwrap();

    for post in &posts {
        let mut context = Context::new();
        context.insert("post", &post);
        context.insert("title", &post.title);
        context.insert("subtitle", "Thibaud Dauce");
        context.insert("blog", &true);
        let mut post_html = tera.render("post.html", &context).unwrap();

        let number_of_codes = markdown_regexp.captures_iter(&post.markdown).count();
        for i in 0..number_of_codes {
            let found_in_markdown = markdown_regexp.captures_iter(&post.markdown).nth(i).unwrap();
            let found_in_html = html_regexp.captures_iter(&post_html).nth(i).unwrap();

            let lang = found_in_markdown.get(1).unwrap().as_str();

            if lang == "" {
                continue;
            }

            let code_in_markdown = found_in_markdown.get(2).unwrap();
            let code_in_html = found_in_html.get(3).unwrap();

            let syntax = ps.find_syntax_by_extension(lang).unwrap();

            let mut code = code_in_markdown.as_str().to_string();
            let mut without_php_opening = false;
            if !code.starts_with("<?php") && lang == "php" {
                without_php_opening = true;
                code = format!("<?php\n{}", code);
            }

            if lang == "hs" {
                code = format!("module Example where\n\n{}", code);
            } 

            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(&syntax, &ps, ClassStyle::SpacedPrefixed { prefix: "syntect" });
            for line in code.lines() {
                html_generator.parse_html_for_line(&line);
            }
            let mut output_html = html_generator.finalize();

            if without_php_opening {
                let end_of_first_line = output_html.find("\n").unwrap() + 1;
                output_html.replace_range(..end_of_first_line, "");
            }

            // let new_html = LinesWithEndings::from(code_in_markdown.as_str()).map(|line| {
            //     let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);

            //     ranges.iter()
            //         .filter(|range| range.1 != "")
            //         .map(|range| {
            //             format!(r#"<span style="color: rgba({}, {}, {}, {})">{}</span>"#, range.0.foreground.r, range.0.foreground.g, range.0.foreground.b, range.0.foreground.a, range.1)
            //         })
            //         .collect::<Vec<String>>()
            //         .join("")
            // }).collect::<Vec<String>>().join("\n");

            let range = code_in_html.range();
            post_html.replace_range(range, &output_html);
        }

        // for found in markdown_regexp.captures_iter(&post.markdown.clone()) {
        //     if found.get(1).unwrap().as_str() == "" {
        //         continue;
        //     }
        //     // dbg!(found.get(1).unwrap().as_str());

        //     let code = found.get(2).unwrap();

        //     // dbg!(&code.as_str());





        //     // post_html.replace_range(code.range(), &new_html);
        // }

        fs::write(format!("./build{}", post.url), post_html).unwrap();
    }

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("title", "Mes derniers articles");
    context.insert("subtitle", "Thibaud Dauce");
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
    context.insert("subtitle", "Thibaud Dauce");
    context.insert("about", &true);
    let about = tera.render("about.html", &context).unwrap();
    fs::write("build/about.html", about).unwrap();

    let mut context = Context::new();
    context.insert("title", "Mes dernières conférences");
    context.insert("subtitle", "Thibaud Dauce");
    context.insert("talks", &talks);
    let talks = tera.render("talks.html", &context).unwrap();
    fs::write("build/talks.html", talks).unwrap();

    let mut context = Context::new();
    context.insert("title", "Mes dernières randonnées");
    context.insert("subtitle", "Thibaud Dauce");
    context.insert("traces", &traces);
    context.insert("hiking", &true);
    let hiking = tera.render("hiking.html", &context).unwrap();
    fs::write("build/hiking.html", hiking).unwrap();

    copy_dir("./content/images", "./build/images").unwrap();
    copy_dir("./content/videos", "./build/videos").unwrap();
    copy_dir("./content/traces", "./build/traces").unwrap();
    copy_dir("./content/talks", "./build/talks").unwrap();
}

fn get_contents(directory: &str) -> Vec<Content> {
    let mut contents: Vec<Content> = fs::read_dir(format!("content/{}", directory)).unwrap()
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
    let parser = Parser::new_ext(&markdown, options);

    // Write to String buffer.
    let mut html = String::new();
    html::push_html(&mut html, parser);

    // let event_offsets: Vec<_> = Parser::new(&markdown)
    //     .into_offset_iter()
    //     .map(|event_and_range| {
    //         dbg!(&event_and_range);
    //         event_and_range.1
    //     })
    //     .collect();

    // let more_split: Vec<&str> = html.split("<!--more-->").collect();
    // let html_preview = more_split[0].to_string();

    Content {
        filename: filename.clone(),
        url: format!("/{}/{}", prefix, filename.replace(".md", ".html")),
        title: metadata["title"].as_str().unwrap().to_string(),
        description: metadata["description"].as_str().unwrap_or("").to_string(),
        markdown: markdown,
        html: html,
        // html_preview: html_preview,

        date_fr: format!("{} {} {}", date[2].trim_matches('0'), french_months(date[1]), date[0]),
        date_en: format!("{} {}, {}", english_months(date[1]), date[2].trim_matches('0'), date[0]),
        date_rss: Utc.ymd(date[0].parse().unwrap(), date[1].trim_matches('0').parse().unwrap(), date[2].trim_matches('0').parse().unwrap()).and_hms(0, 0, 0).to_rfc2822(),

        slides: metadata["slides"].as_str().map(|s| s.to_string()),
        video: metadata["video"].as_str().map(|s| s.to_string()),
        tweet: metadata["tweet"].as_str().map(|s| s.to_string()),
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
        }).to_string()
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
        }).to_string()
}