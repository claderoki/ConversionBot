use std::collections::HashMap;
use crate::core::conversion::MeasurementKind::Unit;
use crate::core::conversion::{Measurement, MeasurementKind};
use sqlx::{MySql, Pool};

pub async fn load_units(pool: &Pool<MySql>) -> Result<Vec<Measurement>, String> {
    Ok(sqlx::query!("SELECT * FROM `measurement`")
        .fetch_all(pool)
        .await
        .map_err(|_| String::from("Error"))?
        .into_iter()
        .map(|r| Measurement {
            symbol: r.symbol,
            code: r.code,
            rate: r.rate,
            name: r.name,
            kind: Unit,
        })
        .collect())
}

pub async fn load_currencies(pool: &Pool<MySql>) -> Result<Vec<Measurement>, String> {
    Ok(sqlx::query!("SELECT * FROM `currency`")
        .fetch_all(pool)
        .await
        .map_err(|_| String::from("Error"))?
        .into_iter()
        .map(|r| Measurement {
            symbol: r.symbol,
            code: r.code,
            rate: r.rate,
            name: r.name,
            kind: MeasurementKind::Currency,
        })
        .collect())
}

pub fn _get_others() {
    let a = vec![
        vec!["c", "f"],
        vec!["kg", "lb"],
        vec!["g", "oz"],
        vec!["cm", "inch", "ft"],
        vec!["km", "mi"],
        vec!["m", "yd", "ft"],
    ];

    #[inline]
    fn test(_f: Vec<&str>) -> Vec<(String, Vec<String>)> {
        vec![]
    }
    let _b: HashMap<String, Vec<String>> = a.into_iter().flat_map(test).collect();
}
