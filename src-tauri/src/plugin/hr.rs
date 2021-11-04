use std::sync::atomic::Ordering;

use bleasy::{common::characteristics::HEART_RATE_MEASUREMENT, ScanConfig, Scanner};
use futures::StreamExt;
use serde_json::Value as JsonValue;
use tauri::{
    AppHandle,
    async_runtime::{block_on, JoinHandle, spawn},
    Invoke, Manager, PageLoadPayload, plugin::Plugin, Runtime, Window,
};
use tokio::time::{Duration, sleep};

const SCANNER_START_RETRY_INTERVAL: u64 = 5000;

pub struct HrPlugin {
    thread: Option<JoinHandle<()>>,
}

impl HrPlugin {
    pub fn new() -> Self {
        Self { thread: None }
    }
}

impl<R: Runtime> Plugin<R> for HrPlugin {
    fn name(&self) -> &'static str {
        "hr"
    }

    fn initialize(&mut self, app: &AppHandle<R>, config: JsonValue) -> tauri::plugin::Result<()> {
        log::info!("Initializing HR plugin");

        Ok(())
    }

    fn created(&mut self, window: Window<R>) {
        window.set_decorations(true).unwrap();

        self.thread = Some(start_hr_thread(window));
    }
}

fn start_hr_thread<R: Runtime>(window: Window<R>) -> JoinHandle<()> {
    spawn(async move {
        loop {
            loop {
                let scan_filter = ScanConfig::default()
                    .filter_by_characteristics(|chars| chars.contains(&HEART_RATE_MEASUREMENT))
                    .stop_after_first_match();

                let mut scanner = Scanner::new().await.unwrap();
                scanner.start(scan_filter).await;

                log::info!("Scanning for HR sensors ...");

                let mut device_stream = scanner.device_stream();
                let mut sensor = if let Some(sensor) = device_stream.next().await {
                    sensor
                } else {
                    sleep(Duration::from_millis(SCANNER_START_RETRY_INTERVAL)).await;
                    continue;
                };


                let hr_measurement = sensor.characteristic(HEART_RATE_MEASUREMENT).await.unwrap().unwrap();

                log::info!("Subscribing to hr measurement");
                if let Ok(mut hr_stream) = hr_measurement.subscribe().await {
                    log::info!("Reading heart rate values");
                    while let Some(hr_value) = hr_stream.next().await {
                        let hr_value = hr_value[1];
                        log::info!("HR: {}", hr_value);
                        window.emit_all("current_hr", hr_value).unwrap();
                    }
                }
            }
        }
    })
}
