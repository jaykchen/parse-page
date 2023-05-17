use std::path::Path;
use std::{path::PathBuf, process::exit};

use anyhow::Result;
use article_scraper::{
    ArticleScraper, FtrConfigEntry, FullTextParser,
    Readability::{self},
};
use headless_chrome::{
    protocol::cdp::Page::CaptureScreenshotFormatOption, types::PrintToPdfOptions, Browser,
    LaunchOptions,
};
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::fs;
use tokio::sync::mpsc::{self, Sender};
use url::Url;
use poppler::PopplerDocument;
use poppler::PopplerPage;

#[tokio::main]
async fn main() -> Result<()> {
    let options = LaunchOptions {
        headless: true,
        window_size: Some((1200, 1920)),
        ..Default::default()
    };

    let browser = Browser::new(options)?;

    let url = "https://github.com/amiiiiii830?tab=repositories";
    let url = "https://www.theverge.com/2023/5/16/23726119/congress-ai-hearing-sam-altman-openai";
    // let url = "https://www.wsj.com/articles/the-return-to-the-office-has-stalled-e0af9741?mod=hp_lead_pos1";

    let tab = browser.new_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(10));
    tab.navigate_to(url)?;

    // tab.wait_for_element_with_custom_timeout(
    //     ".js-details-container",
    //     std::time::Duration::from_secs(10),
    // )?;

    let html = tab.get_content()?;

    let base_url = Url::parse("https://github.com/").unwrap();
    let base_url = Url::parse("https://www.theverge.com/").unwrap();
    // let base_url = Url::parse("https://online.wsj.com/").unwrap();
    // let jpeg_data =
    //     tab.capture_screenshot(CaptureScreenshotFormatOption::Jpeg, Some(75), None, true)?;
    // fs::write("screenshot.verge.jpg", jpeg_data)?;

    let pdf_options: Option<PrintToPdfOptions> = Some(PrintToPdfOptions {
        landscape: Some(false),
        display_header_footer: Some(false),
        print_background: Some(false),
        paper_width: Some(11.0),
        paper_height: Some(17.0),
        ignore_invalid_page_ranges: Some(true),
        prefer_css_page_size: Some(false),
        transfer_mode: None,
        ..Default::default()
    });

    //  Some(PrintToPdfOptions {
    //      landscape: Some(false),
    //      display_header_footer: Some(false),
    //      print_background: Some(false),
    //      paper_width: Some(279.0),
    //      paper_height: Some(432.0),
    //      ignore_invalid_page_ranges: Some(true),
    //      prefer_css_page_size: Some(false),
    //      transfer_mode: None,  // Option<TransferMode>,      Some(Page::PrintToPDFTransfer_modeOption::ReturnAsBase64) // Some(Page::PrintToPDFTransfer_modeOption::ReturnAsStream)
    //      ..Default::default()
    // });

    let local_pdf = tab.print_to_pdf(pdf_options)?;

    fs::write("story.verge.pdf", local_pdf.clone())?;

    let mut page_pdf = local_pdf.to_vec();

    let doc = PopplerDocument::new_from_data(&mut page_pdf, "")?;

    let page = doc.get_page(0).unwrap();
    let content = page.get_text ().unwrap();

    println!("page {:?}", doc ());
    println!("-------------------------------");
    println!("page {:?}", content);

    fs::write("raw.verge.html", html.clone())?;

    extract_readability(
        html,
        Some(base_url.to_string()),
        Some(PathBuf::from("clean.verge.html")),
    )
    .await;
    // println!("{:?}", extracted_content.);
    // let bytes = std::fs::read("new-ithub.pdf")?;
    // let out = pdf_extract::extract_text_from_mem(&bytes)?;

    Ok(())
}

async fn extract_readability(html: String, base_url: Option<String>, output: Option<PathBuf>) {
    let base_url = base_url.map(|url| Url::parse(&url).expect("invalid base url"));
    let result = match Readability::extract(&html, base_url).await {
        Ok(res) => res,
        Err(err) => {
            exit(0);
        }
    };

    let output = if let Some(output) = output {
        output
    } else {
        PathBuf::from("result.html")
    };

    match std::fs::write(&output, result) {
        Ok(()) => {}
        Err(err) => {
            exit(0);
        }
    }
}
