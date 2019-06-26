//! Measurement receiver.
use crate::db::*;
use crate::measurement::*;
use log::info;
use std::sync::mpsc::Receiver;

/// Run the receiver.
pub fn run(rx: Receiver<Measurement>, db: &Db) {
    for measurement in rx {
        info!("{}: {:?}", &measurement.sensor, &measurement.value);
        db.save_measurement(&measurement);
    }
}
