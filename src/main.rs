use chrono::Local;
use clap::{ArgAction, Parser};
use regex::RegexBuilder;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct AssignmentFiles {
    paths: Vec<String>,
}

impl FromStr for AssignmentFiles {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_chars: Vec<char> = s.chars().collect();

        let s = String::from_iter(as_chars.as_slice().iter());
        let paths: Vec<&str> = s.split(' ').collect();

        if paths.len() == 0 {
            return Err(String::from("No file paths have been supplied."));
        }

        let regex = RegexBuilder::new("(\\/?[^:/\0]+)+").build().unwrap();

        let mut v: Vec<String> = Vec::new();

        for path in &paths {
            let captures = regex.captures(path);
            if let None = captures {
                return Err(format!("{} is not a valid path", *path));
            }
            let captures = captures.unwrap();
            if *path != captures.get_match().as_str() {
                return Err(format!("{} is not a valid path.", path));
            }
            v.push(String::from(*path));
        }

        Ok(Self { paths: v })
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    class: String,

    #[arg(short, long)]
    student_number: u32,

    #[arg(short, long, value_parser = clap::value_parser!(AssignmentFiles), action = ArgAction::Append, num_args = 1.., required = true)]
    assignment_files: Vec<AssignmentFiles>,

    #[arg(short, long)]
    tutorial: bool,

    #[arg(short, long)]
    week: u8,

    #[arg(short, long)]
    output_file: Option<String>,
}

fn write_header_on_file(
    buf_writer: &mut BufWriter<File>,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let h1_text = if args.tutorial == true {
        format!("# Tutorial week {}  \n", args.week)
    } else {
        format!("# Assignment week {}  \n", args.week)
    };

    let student_name_text = format!("Name: {}  \n", args.name);
    let student_number_text = format!("Student number: {}  \n", args.student_number);
    let class_text = format!("Class: {}  \n", args.class);
    let date_text = format!("Date: {}  \n", Local::now().format("%d/%m/%Y"));

    buf_writer.write(h1_text.as_bytes())?;
    buf_writer.write(String::from("  \n").as_bytes())?;
    buf_writer.write(student_name_text.as_bytes())?;
    buf_writer.write(student_number_text.as_bytes())?;
    buf_writer.write(class_text.as_bytes())?;
    buf_writer.write(date_text.as_bytes())?;
    buf_writer.write(String::from("  \n").as_bytes())?;

    Ok(())
}

fn get_file_name_from_path(path: &str) -> &str {
    let regex = RegexBuilder::new("([^:/\0]+)+").build().unwrap();
    regex.find_iter(path).last().unwrap().as_str()
}

fn get_escape_expression_from_file_extension(
    file: &str,
    file_path: &str,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    let extension_regex = RegexBuilder::new("\\.[^:/\0]{1,3}").build()?;
    if let Some(r) = extension_regex.find_iter(file).last() {
        match r.as_str() {
            ".c" | ".h" => Ok("c"),
            ".cpp" | ".hpp" => Ok("cpp"),
            _ => Err(format!("Unsupported extension {}.", r.as_str()).into()),
        }
    } else {
        Err(format!(
            "No extension was found for file with name '{}' on path '{}'.",
            file, file_path
        )
        .into())
    }
}

fn write_assignments(
    buf_writer: &mut BufWriter<File>,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    for (i, assignment_files) in args.assignment_files.iter().enumerate() {
        if args.tutorial == true {
            buf_writer.write(format!("## Tutorial {}  \n  \n", i + 1).as_bytes())?;
        } else {
            buf_writer.write(format!("## Assignment {}  \n  \n", i + 1).as_bytes())?;
        }

        for path in &assignment_files.paths {
            let file_name = get_file_name_from_path(path);
            buf_writer.write(format!("### File: {}  \n  \n", file_name).as_bytes())?;
            buf_writer.write(
                format!(
                    "```{}\n",
                    get_escape_expression_from_file_extension(file_name, path)?
                )
                .as_bytes(),
            )?;

            let read_file = File::open(path)?;
            let mut buf_reader = BufReader::new(read_file);
            let mut v = Vec::new();
            buf_reader.read_to_end(&mut v)?;
            buf_writer.write(v.as_slice())?;
            buf_writer.write(format!("```  \n").as_bytes())?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let output_file = if let Some(p) = &args.output_file {
        // p will have size at least one as it will be validated by clap
        p.clone()
    } else {
        format!("week{}.md", args.week)
    };

    let file = File::options()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open(&output_file)?;
    let mut buf_writer = BufWriter::new(file);

    write_header_on_file(&mut buf_writer, &args)?;
    write_assignments(&mut buf_writer, &args)?;

    Ok(())
}
