// use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::thread;
use std::time::Duration;

use csv::Writer;
use thirtyfour::{
    // prelude::{ElementWaitable, WebDriverError},
    prelude::WebDriverError,
    By,
    DesiredCapabilities,
    Key,
    WebDriver,
    WebElement,
};


// wait

const ACTION_CLICK: &str = "action_click";
const ACTION_FORM_FILL_FIELD: &str = "action_form_fill_field";
const ACTION_SCREENSHOT_WEB_ELEMENT: &str = "screenshot_web_element";

// &["6",ACTION_,"",""],
const WEB_XPATH: &[&[&str]] = &[
    //No.,Action,FieldName,xpath
    &[
        "1",
        ACTION_CLICK,
        "accept",
        "/html/body/div[1]/div/div/div/div[2]/div/button[3]",
    ],
    &["2",ACTION_FORM_FILL_FIELD,"TREX","/html/body/table[1]/tbody/tr[1]/td/table/tbody/tr/td[1]/table/tbody/tr[2]/td/div/label/div/input"],
    &["3",ACTION_SCREENSHOT_WEB_ELEMENT,"chart_stock.png","/html/body/div[4]/div[3]/div[3]/div[1]/div/div[1]/div/div[2]/div/div[2]/canvas[2]"],
    &["4",ACTION_SCREENSHOT_WEB_ELEMENT,"table_chart_data.png","/html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[1]/tbody/tr/td/div[2]/table"],
    &["5",ACTION_SCREENSHOT_WEB_ELEMENT,"table_income_statement.png","/html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[2]"],
    &["6",ACTION_CLICK,"balance sheet","/html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[1]/tbody/tr/td[1]/span/a[2]"],
    &["7",ACTION_SCREENSHOT_WEB_ELEMENT,"table_balance_sheet.png","/html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[2]"],
    &["8",ACTION_CLICK,"cash_flow","/html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[1]/tbody/tr/td[1]/span/a[3]"],
    &["9",ACTION_SCREENSHOT_WEB_ELEMENT,"table_cash_flow.png","/html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[2]"],


    // => /html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[1]/tbody/tr/td[1]/span/a[3]
    // => /html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[2]
    // => /html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[2]/tbody/tr[11]/td/div/table[2]                                                      
    

    // => /html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[1]/tbody/tr/td/div[2]/table
    // => /html/body/div[4]/div[3]/div[4]/table/tbody/tr/td/div/table[1]/tbody/tr/td/div[2]
    //  &["5","select Market Cap","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[1]/td[2]/select/option[3]"],
    //  &["6","select Option/Short","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[8]/td[10]/select/option[2]"],
    //  &["7","200-Day Simple Moving Average","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[8]/select/option[12]"],
    //  &["8","sma_over_50_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[6]/select/option[8]"],
    //  &["9","sma_over_20_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[4]/select/option[8]"],
    //  &["10","price_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[13]/td[8]/select/option[39]"],
    //  &["11","pattern_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[13]/td[8]/select/option[39]"],
    //  &["12","peg_over_one_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]"],
    //  &["13","eps_year_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[3]/td[8]/select/option[3]"],
    //  &["14","eps_qtr_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[4]/td[8]/select/option[3]"],
    //  &["15","peg_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]"],
    //  &["16","beta_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[12]/td[6]/select/option[7]"],
];

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}

async fn run() -> color_eyre::Result<(), Box<dyn Error>> {
    let _place: &str = "Place";
    let _driver = initialize_driver().await?;

    _driver.goto("https://finviz.com").await?;
    thread::sleep(Duration::from_secs(2));

    search_location(&_driver, _place).await?;
    thread::sleep(Duration::from_secs(2));

    path_to(_driver.clone()).await?;
    #[allow(unreachable_code)]
    screenshot_browser(_driver.clone()).await?;
    // process::exit(0);

    // NOT NEED please clean
    // save_result_table(_driver.clone()).await?;
    // close_browser(_driver.clone()).await?;

    Ok(())
}

#[allow(dead_code)]
async fn close_browser(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    // Always explicitly close the browser.
    _driver.quit().await?;

    Ok(())
}

async fn screenshot_browser(driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    // FROM HERE  screenshot of browser windows
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch

    let _screenshot = driver.screenshot_as_png().await?;

    // FROM HERE  write to file
    // https://doc.rust-lang.org/std/fs/struct.File.html
    let mut _file = File::create("screenshot.png")?;
    _file.write_all(&_screenshot)?;

    // println!("Screenshot of browser windows => {:?} ",screenshot);
    Ok(())
}

async fn screenshot_web_element (
    web_element: WebElement,
    screenshot_name: &str,
) -> color_eyre::Result<(), Box<dyn Error>> {
    
    // FROM HERE screenshot of browser windows
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch
    
    let _screenshot = web_element.screenshot_as_png().await?;

    // FROM HERE  write to file
    // https://doc.rust-lang.org/std/fs/struct.File.html

    let mut _file = File::create(screenshot_name)?;
    _file.write_all(&_screenshot)?;

    // println!("Screenshot of browser windows => {:?} ",screenshot);
    Ok(())
}

async fn wait_seconds_of_browser(
    _driver: WebDriver,
    waiting_period: u64,
) -> color_eyre::Result<(), Box<dyn Error>> {
    // wait for page already load
    println!("Status driver => {:?}", _driver.status().await?);
    println!("Wait {} seconds for browser",waiting_period);
    tokio::time::sleep(Duration::from_secs(waiting_period)).await;

    Ok(())
}

async fn path_to(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    // wait browser already load
    // ElementWaitable
    wait_seconds_of_browser(_driver.clone(), 20).await?;

    for field in 0..WEB_XPATH.len() {
        println!("No.   => {}", WEB_XPATH[field][0]);
        println!("Action => {}", WEB_XPATH[field][1]);
        println!("Field => {}", WEB_XPATH[field][2]);

        if ACTION_CLICK == WEB_XPATH[field][1] {
            println!("Action =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);

            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            elem_form.click().await?;
            wait_seconds_of_browser(_driver.clone(), 5).await?;
        } else if ACTION_FORM_FILL_FIELD == WEB_XPATH[field][1] {
            println!(
                "Action =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            // elem_form.send_keys("TREX").await?;
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            //old
            //elem_form.send_keys(Key::Enter.to_string()).await?;
            elem_form.send_keys(Key::Enter).await?;
            // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            // /html/body/table[1]/tbody/tr[1]/td/table/tbody/tr/td[1]/table/tbody/tr[2]/td/div/label/div/input
        } else if ACTION_SCREENSHOT_WEB_ELEMENT == WEB_XPATH[field][1] {
            println!(
                "Action =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            let _web_element: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            let _screenshot_name: &str = WEB_XPATH[field][2];

            screenshot_web_element(_web_element, _screenshot_name).await?;
        } else {
            println!("ACTION NOT FOUND");
            process::exit(1);
            // error not found
        }

        // old cleanup please
        //println!("XPath => {}",WEB_XPATH[field][2]);

        // #[allow(unreachable_code)]
        // let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][2])).await?;
        // #[allow(unreachable_code)]
        // elem_form.click().await?;
        // #[allow(unreachable_code)]
        // wait_seconds_of_browser(_driver.clone(), 5).await?;
    }

    wait_seconds_of_browser(_driver.clone(), 20).await?;

    Ok(())
}

//save_result_table
#[allow(dead_code)]
async fn save_result_table(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    const RESULT_TABLE:&[&[&str]] = &[
     //No.,FieldName,xpath        
     &["t1","colum_name","/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/thead/tr"],
     &["t2","No.:",      "/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/tbody/tr"],
      ];

    let mut field = 0;

    let mut wtr = Writer::from_writer(vec![]);

    // debug
    println!("No.   => {}", RESULT_TABLE[field][0]);
    println!("Field => {}", RESULT_TABLE[field][1]);
    println!("XPath => {}", RESULT_TABLE[field][2]);

    let thead_rows_vec: Vec<WebElement> = _driver.find_all(By::XPath(RESULT_TABLE[0][2])).await?;

    println!("DEBUG: thead_rows_vec len => {:?}", thead_rows_vec.len());

    let mut row = 0;

    for thead_row in thead_rows_vec {
        let thead_cell_vec: Vec<WebElement> = thead_row.find_all(By::XPath("th")).await?;

        println!("DEBUG: thead_cell_vec len => {:?}", thead_cell_vec.len());

        let mut column = 0;
        for thead_cell in thead_cell_vec {
            column = column + 1;
            let cell_text = thead_cell.text().await?;
            println!(
                "DEBUG: write_field row/column {}/{} => {}",
                row, column, cell_text
            );
            wtr.write_field(cell_text)?;
        } //finish inner for loop => thead_cell

        println!("DEBUG: write_record");
        let _ = &wtr.write_record(None::<&[u8]>)?;

        field = 1;

        // debug
        println!("No.   => {}", RESULT_TABLE[field][0]);
        println!("Field => {}", RESULT_TABLE[field][1]);
        println!("XPath => {}", RESULT_TABLE[field][2]);

        let tbody_row_vec: Vec<WebElement> =
            _driver.find_all(By::XPath(RESULT_TABLE[field][2])).await?;

        row = 0;
        for tbody_row in tbody_row_vec {
            row = row + 1;

            let tbody_cell_vec: Vec<WebElement> = tbody_row.find_all(By::XPath("td")).await?;
            let mut column = 0;
            for tbody_cell in tbody_cell_vec {
                column = column + 1;
                let cell_text = tbody_cell.text().await?;
                println!(
                    "DEBUG: write_field row/column {}/{} => {}",
                    row, column, cell_text
                );
                wtr.write_field(cell_text)?;
            } //finish inner for loop => tbody_cell

            println!("DEBUG: write_record");

            // don't use result
            let _ = &wtr.write_record(None::<&[u8]>)?;
        } //finish for loop => tbody_row
    } //finish for loop => thead_row

    let mut file = File::create("result.csv")?;
    file.write_all(&wtr.into_inner()?)?;

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let _caps = DesiredCapabilities::chrome();

    // let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless")?;
    // caps.add_chrome_arg("--no-sandbox")?;
    //  caps.add_chrome_arg("--disable-dev-shm-usage")?;

    let driver = WebDriver::new("http://localhost:9515", _caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

async fn search_location(_driver: &WebDriver, _place: &str) -> Result<(), WebDriverError> {
    // click_choose_place(driver).await?;

    // write_place(driver, place).await?;

    // click_search_button(driver).await?;

    Ok(())

    // sudo dbus-daemon --system &> /dev/null
    // https://github.com/cypress-io/cypress/issues/4925
}

/*
rustfmt  ./examples/tokio_finviz_method_five.rs
*/
