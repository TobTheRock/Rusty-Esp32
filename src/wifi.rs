use esp_wifi::wifi::{ClientConfiguration, Configuration, WifiController, WifiEvent};
use log::{info, warn};

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

pub struct ClientWifiConnection<'a> {
    controller: WifiController<'a>,
}

// TODO return result
impl<'a> ClientWifiConnection<'a> {
    pub async fn start_new(mut controller: WifiController<'a>) -> Self {
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = ClientConfiguration {
                ssid: SSID.try_into().unwrap(),
                password: PASSWORD.try_into().unwrap(),
                ..Default::default()
            };
            controller
                .set_configuration(&Configuration::Client(client_config))
                .unwrap();
            info!("Starting wifi");
            controller.start().await.unwrap();
            info!("Wifi started!");
        } else {
            warn!("Wifi already started");
            // TODO error?
        }

        Self { controller }
    }

    // TODO return result
    pub async fn connect(&mut self) {
        info!("About to connect...");

        match self.controller.connect().await {
            Ok(_) => info!("Wifi connected!"),
            Err(e) => {
                info!("Failed to connect to wifi: {e:?}");
                return;
            }
        }

        self.controller
            .wait_for_event(WifiEvent::StaDisconnected)
            .await;
    }
}
