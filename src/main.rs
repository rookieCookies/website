use std::{fmt::Write, time::SystemTime};

use atom_syndication::Category;
use chrono::Datelike;
use image::codecs::webp::WebPEncoder;
use rss_gen::{generate_rss, RssVersion};

fn main() {
    let index_template = include_str!("../index_template.html");
    let blog_template = include_str!("../blog_template.html");
    let mut blogs = vec![];
    let mut dirs = vec![];

    dirs.push((false, std::fs::read_dir("blogs").unwrap()));
    dirs.push((true, std::fs::read_dir("hidden_blogs").unwrap()));

    for (is_hidden, dir) in dirs {
        for item in dir {
            let item = item.unwrap();
            let name = item.file_name().to_string_lossy().to_string();
            let metadata = item.metadata().unwrap();

            if !metadata.is_dir() {
                println!("skipping '{name}' because it's not a valid directory");
                continue;
            }

            let index = item.path().join("index.md");
            let created = std::fs::metadata(item.path().join("thumbnail.png")).map(|x| x.created().unwrap()).unwrap_or(SystemTime::now());
            let index = std::fs::read_to_string(index).unwrap();

            std::fs::write(&format!("{}/index.html", &*item.path().to_string_lossy()), markdown::to_html(&index)).unwrap();

            blogs.push(Blog {
                ident: item.path().to_string_lossy().to_string(),
                markdown: index,
                creation_date: created,
                is_hidden,
            });

        }
    }

    blogs.sort_by_key(|x| x.creation_date);

    let mut blogs_section = String::new();
    let mut rss = rss_gen::RssData::new(Some(RssVersion::RSS2_0))
        .title("daymare.net")
        .link("https://daymare.net/")
        .description("Explore my personal projects, technical blogs, and creative coding experiments at daymare.net.")
        .language("en-us");

    let mut atom_entries = vec![];

    for blog in blogs.iter().rev() {
        let ident = &blog.ident;
        let title = blog.markdown.lines().next().unwrap_or("# Untitled");
        let title = title.split_once('#').unwrap().1.trim();

        let read_time = {
            let word_count = blog.markdown.split_whitespace().count();
            let wpm = 200;
            word_count.div_ceil(wpm)
        };

        let mut description = String::new();

        for line in blog.markdown.lines().skip(1) {
            description.push_str(line);
            if line.ends_with('\\') && !line.ends_with("\\\\") {
                description.pop();
                continue;
            }
            break;
        }

        let description_html = markdown::to_html(&description);
        let thumbnail = format!("{}/thumbnail.png", ident);
        let thumbnail = if std::fs::exists(&thumbnail).unwrap() { thumbnail.as_str() }
                        else { "https://placehold.co/1900x1600" };

        if !std::fs::exists(format!("{ident}/assets")).unwrap() {
            std::fs::create_dir(format!("{ident}/assets")).unwrap();
        }

        // downscale thumbnail & convert to webp
        {
            let img = image::open(&thumbnail).unwrap();
            assert!(img.width() == 1900 && img.height() == 1600, "thumbnail image must be 1900x1600 pixels");

            for size in [400, 800, 1200, 1600] {
                let resized = img.resize_exact(size, size * 1600 / 1900, image::imageops::FilterType::Lanczos3);
                let output_path = format!("{}/assets/thumbnail_{}x{}.webp", ident, size, size * 1600 / 1900);
                let mut output_file = std::fs::File::create(&output_path).unwrap();
                let encoder = WebPEncoder::new_lossless(&mut output_file);
                encoder.encode(&resized.to_rgba8(), resized.width(), resized.height(), image::ExtendedColorType::Rgba8).unwrap();
            }
            
        }

        if !blog.is_hidden {
            let _ = writeln!(
                &mut blogs_section,
                "
                    <a class=\"blog-card\" href=\"{ident}\">
                        <img src=\"{thumbnail}\" alt=\"Blog Image\">
                        <span class=\"titlecard\"><h3>{title}</h3></span>
                        <h4>{read_time} min. read</h4>
                        <p>{description_html}</p>
                    </a>
                "
            );
        }


        // the blog's index.html
        let html = markdown::to_html(blog.markdown.split_once('\n').unwrap().1);
        let date = chrono::DateTime::<chrono::prelude::Utc>::from(blog.creation_date);
        let month = match date.month() {
            1  => "Jan",
            2  => "Feb",
            3  => "Mar",
            4  => "Apr",
            5  => "May",
            6  => "Jun",
            7  => "Jul",
            8  => "Aug",
            9  => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => unreachable!(),

        };


        let iso_date = date.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
        let template = blog_template
            .replace("<!-- expand-date -->", &format!("{} {}", month, date.day()))
            .replace("<!-- expand-iso-date -->", &iso_date)
            .replace("<!-- expand-title -->", &title)
            .replace("<!-- expand-read-time -->", &read_time.to_string())
            .replace("<!-- expand-description -->", &description)
            .replace("<!-- expand-path -->", &ident)
            .replace("<!-- expand-body -->", &html);

        std::fs::write(format!("{ident}/index.html"), template).unwrap();

        // generate rss item
        if blog.is_hidden {
            continue;
        }


        let rfc_date = date.format("%a, %d %b %Y %H:%M:%S %z").to_string();
        let item = rss_gen::RssItem::new()
            .title(title)
            .link(format!("https://daymare.net/{}", ident))
            .description(description)
            .guid(format!("https://daymare.net/{}", ident))
            .pub_date(rfc_date)
            .enclosure(format!("https://daymare.net/{}/thumbnail.png", ident));

        rss.add_item(item);


        atom_entries.push(atom_syndication::Entry {
            title: title.into(),
            id: format!("https://daymare.net/{}", ident),
            updated: date.into(),
            authors: vec![atom_syndication::Person {
                name: "daymare".into(),
                email: None,
                uri: Some("https://daymare.net/".into()),
            }],
            links: vec![atom_syndication::Link {
                href: format!("https://daymare.net/{}", ident),
                rel: "alternate".into(),
                mime_type: None,
                hreflang: None,
                title: None,
                length: None,
            }],
            content: Some(atom_syndication::Content {
                value: Some(html.replace("<img src=\"", format!("<img src=\"https://daymare.net/{}/", ident).as_str())),
                src: None,
                content_type: Some("html".into()),
                ..Default::default()
            }),
            categories: vec![Category {
                term: "Blog".into(),
                scheme: None,
                label: None,
            }],
            published: Some(date.into()),
            summary: Some(description_html.into()),
            ..Default::default()
        });
    }

    let atom = atom_syndication::Feed {
        title: "daymare.net".into(),
        id: "https://daymare.net/".into(),
        updated: chrono::Utc::now().into(),
        authors: vec![atom_syndication::Person {
            name: "daymare".into(),
            email: None,
            uri: Some("https://daymare.net/".into()),
        }],
        links: vec![atom_syndication::Link {
            href: "https://daymare.net/".into(),
            rel: "self".into(),
            mime_type: None,
            hreflang: None,
            title: None,
            length: None,
        }],
        entries: atom_entries,
        ..Default::default()
    };


    let output = index_template.replace("<!-- expand-blogs -->", &blogs_section);
    std::fs::write("index.html", output).unwrap();
    std::fs::write("rss.xml", generate_rss(&rss).unwrap()).unwrap();
    std::fs::write("atom.xml", atom_syndication::Feed::to_string(&atom)).unwrap();

}


struct Blog {
    ident: String,
    markdown: String,
    creation_date: SystemTime,
    is_hidden: bool,
}
