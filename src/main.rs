use std::{fmt::Write, time::SystemTime};

fn main() {
    let input = include_str!("../template.html");
    let mut blogs = vec![];
    for item in std::fs::read_dir("blogs").unwrap() {
        let item = item.unwrap();
        let name = item.file_name().to_string_lossy().to_string();
        let metadata = item.metadata().unwrap();

        if !metadata.is_dir() {
            println!("skipping '{name}' because it's not a valid directory");
            continue;
        }

        let created = metadata.created().unwrap();
        let index = item.path().join("index.md");
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
    for blog in blogs {
        let ident = &blog.ident;
        let title = blog.markdown.lines().next().unwrap_or("# Untitled");
        let title = title.split_once('#').unwrap().1;

        let read_time = {
            let word_count = blog.markdown.split_whitespace().count();
            let wpm = 200;
            println!("{word_count}");
            word_count.div_ceil(wpm)
        };

        let mut html = String::new();

        for line in blog.markdown.lines().skip(1) {
            html.push_str(line);
            if line.ends_with('\\') && !line.ends_with("\\\\") {
                html.pop();
                continue;
            }
            break;
        }

        let html = markdown::to_html(&html);
        let thumbnail = format!("blogs/{}/thumbnail.png", blog.ident);
        let thumbnail = if std::fs::exists(&thumbnail).unwrap() { thumbnail.as_str() }
                        else { "https://placehold.co/1900x1600" };


        let _ = writeln!(
            &mut blogs_section,
            "
                <a class=\"blog-card\" href=\"blogs//{ident}/index.html\">
                    <img src=\"{thumbnail}\" alt=\"Blog Image\">
                    <h2>{title}</h2>
                    <h3>{read_time} min. read</h3>
                    <p>{html}</p>
                </a>
            "
        );
    }


    let output = input.replace("<!-- expand-blogs -->", &blogs_section);
    std::fs::write("index.html", output).unwrap();

}


struct Blog {
    ident: String,
    markdown: String,
    creation_date: SystemTime,
}
