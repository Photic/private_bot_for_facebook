use std::path::Path;

use thirtyfour::prelude::*;

mod control;

#[tokio::main(worker_threads = 1)]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto("https://www.facebook.com").await?;

    driver.screenshot(Path::new("./test.png")).await?;

    // #email
    let email_field = driver.find(By::Id("email")).await?;
    email_field.send_keys("test").await?;

    // #pass
    let pass_field = driver.find(By::Id("pass")).await?;
    pass_field.send_keys("test").await?;

    // Login with #u_0_5_Nj
    let login_button = driver.find(By::Name("login")).await?;
    login_button.click().await?;

    driver.screenshot(Path::new("./login_form.png")).await?;

    driver.quit().await?;

    Ok(())
}
