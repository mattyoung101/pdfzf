use clap::Parser;
use inline_colorization::*;
use lopdf::Document;
use nucleo::pattern::Pattern;
use nucleo::Config;
use nucleo::Matcher;
use rayon::prelude::*;

#[derive(Debug, Parser)]
struct Args {
    /// Search term to fuzzy find
    term: String,
    /// List of PDFs to search
    path: Vec<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    const SCORE_THRESHOLD: u32 = 200; // TODO make configurable

    args.path.par_iter().for_each(|path| {
        let doc = Document::load(path.clone());
        // PERF This is extremely cursed from a perf perspective, but I think it's the only way
        // Maybe a thread local would fix it?
        let mut matcher = Matcher::new(Config::DEFAULT);

        match doc {
            Ok(document) => {
                let pages = document.get_pages();

                for (i, _) in pages.iter().enumerate() {
                    let page_number = (i + 1) as u32;
                    let pdf_result = document.extract_text(&[page_number]);

                    match pdf_result {
                        Ok(text) => {
                            let matches = Pattern::parse(
                                &args.term,
                                nucleo::pattern::CaseMatching::Smart,
                                nucleo::pattern::Normalization::Smart,
                            ).match_list(text.lines(), &mut matcher);

                            for the_match in matches {
                                let (match_, score) = the_match;
                                if score <= SCORE_THRESHOLD {
                                    continue;
                                }
                                // println!("SCORE {score}\n{}\n", match_);

                                // TODO highlight match -> we want indices
                                println!("{color_cyan}{}{color_reset} {color_blue}(page {page_number}){color_reset}: {}", path.to_string_lossy(), match_);
                            }
                        }

                        Err(error) => {
                            eprintln!("{color_red}Could not read text from PDF '{}': {error}{color_reset}", path.to_string_lossy());
                            break;
                        }
                    }
                }

                println!();
            }

            Err(error) => {
                eprintln!("{color_red}Could not read PDF file '{}': {error}{color_reset}", path.to_string_lossy());
            }
        }
    });
}
