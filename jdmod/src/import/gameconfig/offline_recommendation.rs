//! # Offline recommendations
//! Import all offline recommendations
use std::{borrow::Cow, collections::HashSet, fs::File};

use anyhow::Error;
use ubiart_toolkit::json_types::OfflineRecommendation;

use crate::types::ImportState;

/// Import all offline recommendations
pub fn import_v20v22(
    is: &ImportState<'_>,
    new_recommendations: OfflineRecommendation<'_>,
) -> Result<(), Error> {
    println!("Importing offline recommendations...");

    let recommendations_path = is.dirs.config().join("offline_recommendations.json");
    let mut recommendations: HashSet<Cow<'_, str>> =
        if let Ok(file) = File::open(&recommendations_path) {
            serde_json::from_reader(file)?
        } else {
            HashSet::new()
        };

    for recommendation in new_recommendations {
        recommendations.insert(recommendation);
    }

    let recommendations_file = File::create(recommendations_path)?;
    serde_json::to_writer_pretty(recommendations_file, &recommendations)?;

    Ok(())
}
