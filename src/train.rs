use burn::prelude::*;
use polars::prelude::*;

pub fn train() {
    // Load all three datasets
    let (gita, bible, quran) = load_datasets();
}

pub fn load_datasets() -> (DataFrame, DataFrame, DataFrame) {
    let gita = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/gitaDataset.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
    println!(
        "Loaded gita dataset with shape: ({}, {})",
        gita.height(),
        gita.width()
    );

    let bible = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/bibleDataset.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
    println!(
        "Loaded bible dataset with shape: ({}, {})",
        bible.height(),
        bible.width()
    );

    let quran = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("./data/quranDataset.csv".into()))
        .unwrap()
        .finish()
        .unwrap();
    println!(
        "Loaded quran dataset with shape: ({}, {})",
        quran.height(),
        quran.width()
    );

    (gita, bible, quran)
}
