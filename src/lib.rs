use chrono::{TimeZone, Utc};
use fluvio_smartmodule::{smartmodule, Result as SmartModuleResult, Record, RecordData};

#[smartmodule(aggregate)]
pub fn aggregate(accumulator: RecordData, current: &Record) -> SmartModuleResult<RecordData> {
    let mut accumulated_stats: AggregateStats = AggregateStats::from(accumulator);

    if let Ok(warrant) = serde_json::from_slice::<WarrantOptions>(current.value().as_ref()) {
        accumulated_stats.warrants.push(warrant);
    }

    if let Ok(quote) = serde_json::from_slice::<FinhubQuoteData>(current.value().as_ref()) {
        accumulated_stats.current_price = quote.current_price;
        let dt = Utc.timestamp(quote.unix_timestamp, 0);
        accumulated_stats.timestamp = dt.to_rfc2822();
    }
    accumulated_stats.update_profit();

    let accumulated_stats: RecordData = accumulated_stats.try_into()?;
    Ok(accumulated_stats)
}

impl AggregateStats {
    fn update_profit(&mut self) {
        let mut profit = 0.0;
        for warrant in &self.warrants {
            profit += (self.current_price - (warrant.exercise_price + warrant.purchase_price))*warrant.count as f64;
        }
        self.current_profit = profit;
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
struct AggregateStats {
    current_price: f64,
    warrants: Vec<WarrantOptions>,
    timestamp: String,
    current_profit: f64,
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct WarrantOptions {
    expiry_date: String,
    exercise_price: f64,
    purchase_price: f64,
    count: i32,
}

/* From
 * https://finnhub.io/docs/api/quote
{
  "c": 261.74, // Current price
  "t": 1582641000 // Unix timestamp
}
*/
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct FinhubQuoteData {
    #[serde(rename = "c")]
    current_price: f64,
    #[serde(rename = "t")]
    unix_timestamp: i64,
}


impl From<RecordData> for AggregateStats {
    fn from(record: RecordData) -> AggregateStats {
        serde_json::from_slice(record.as_ref()).unwrap_or_default()
    }
}
impl From<Record> for AggregateStats {
    fn from(record: Record) -> AggregateStats {
        serde_json::from_slice(record.value().as_ref()).unwrap_or_default()
    }
}

impl TryFrom<AggregateStats> for RecordData {
    type Error = serde_json::Error;
    fn try_from(value: AggregateStats) -> Result<Self, Self::Error> {
        let summed_stars_bytes = serde_json::to_vec(&value)?;
        Ok(summed_stars_bytes.into())
    }
}

#[test]
fn warrant_deserialize() {
    let input = r#"{"expiry_date": "Tue, 13 Apr 2022 13:50:37 +0000", "exercise_price": 110.0, "warrant_purchase_price": 1.0, "count": 10}"#;
    let res = serde_json::from_slice::<WarrantOptions>(input.as_bytes());
    println!("res: {:?}", res);
    assert!(res.is_ok());

}
