use reqwest;
use serde_json::Value;
use tokio;
use chrono::{Utc, TimeZone};  // Removed DateTime
use plotters::prelude::*;
use std::cmp::Ordering;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Corrected timestamp calculation
    let start_timestamp = Utc.ymd(2023, 7, 1).and_hms(0, 0, 0).timestamp();
    let end_timestamp = Utc::now().timestamp();
    let url = format!("https://api.coingecko.com/api/v3/coins/ethereum/market_chart/range?vs_currency=usd&from={}&to={}", start_timestamp, end_timestamp);
    let resp = reqwest::get(&url).await?;
    let body = resp.text().await?;

    let v: Value = serde_json::from_str(&body)?;

    let mut price_points = vec![];

    if let Some(prices) = v.get("prices") {
        if prices.is_array() {
            for price in prices.as_array().unwrap() {
                let date = Utc.timestamp_opt((price[0].as_f64().unwrap() / 1000.0) as i64, 0).single().unwrap();
                let price = price[1].as_f64().unwrap();
                price_points.push((date, price));
            }
        }
    }



    // Sorting the prices by date.
    price_points.sort_by(|a, b| a.0.cmp(&b.0));

    let root = BitMapBackend::new("plot.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_price = price_points.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)).unwrap().1;
    let max_price = price_points.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)).unwrap().1;

    let mut chart = ChartBuilder::on(&root)
        .caption("ETH Price", ("sans-serif", 50).into_font())
        .x_label_area_size(100)
        .y_label_area_size(100)
        .build_cartesian_2d(
            price_points.first().unwrap().0..price_points.last().unwrap().0,
            min_price*0.95..max_price*1.05,
        )?;

    chart
        .configure_mesh()
        .x_labels(80)
        .y_labels(80)
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            price_points,
            &RED,
        ))?
        .label("ETH Price")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
