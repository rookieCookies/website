use std::{fmt::Write, time::SystemTime};

use chrono::Datelike;

fn main() {
    let index_template = include_str!("../index_template.html");
    let blog_template = include_str!("../blog_template.html");
    let mut blogs = vec![];
    for item in std::fs::read_dir("blogs").unwrap() {
        let item = item.unwrap();
        let name = item.file_name().to_string_lossy().to_string();
        let metadata = item.metadata().unwrap();

        if !metadata.is_dir() {
            println!("skipping '{name}' because it's not a valid directory");
            continue;
        }

        let index = item.path().join("index.md");
        let created = std::fs::metadata(&index).unwrap().modified().unwrap();
        let index = std::fs::read_to_string(index).unwrap();

        std::fs::write(&format!("blogs/{name}/index.html"), markdown::to_html(&index)).unwrap();

        blogs.push(Blog {
            ident: name,
            markdown: index,
            creation_date: created,
        });

    }

    blogs.sort_by_key(|x| x.creation_date);

    let mut blogs_section = String::new();
    for (i, blog) in blogs.iter().enumerate().rev() {
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
        let thumbnail = format!("blogs/{}/thumbnail.png", blog.ident);
        let thumbnail = if std::fs::exists(&thumbnail).unwrap() { thumbnail.as_str() }
                        else { "https://placehold.co/1900x1600" };

        let path = format!("blogs/{ident}");
        let _ = writeln!(
            &mut blogs_section,
            "
                <a class=\"blog-card\" href=\"{path}\">
                    <img src=\"{thumbnail}\" alt=\"Blog Image\">
                    <span class=\"titlecard\"><h3>{title}</h3></span>
                    <h4>{read_time} min. read</h4>
                    <p>{description_html}</p>
                </a>
            "
        );


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


        let template = blog_template
            .replace("<!-- expand-date -->", &format!("{} {}", month, date.day()))
            .replace("<!-- expand-iso-date -->", &format!("{}", date.format("%Y-%m-%dT%H:%M:%S%.fZ")))
            .replace("<!-- expand-title -->", &title)
            .replace("<!-- expand-read-time -->", &read_time.to_string())
            .replace("<!-- expand-description -->", &description)
            .replace("<!-- expand-path -->", &path)
            .replace("<!-- expand-body -->", &html);

        std::fs::write(format!("blogs/{ident}/index.html"), template).unwrap();

    }


    let output = index_template.replace("<!-- expand-blogs -->", &blogs_section);
    std::fs::write("index.html", output).unwrap();

}


struct Blog {
    ident: String,
    markdown: String,
    creation_date: SystemTime,
}
