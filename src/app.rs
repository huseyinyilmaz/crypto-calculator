use eframe::{egui, epi};
use std::error::Error;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    buy_amount: String,
    buy_price: String,
    sell_price: String,
}

struct Values {
    amount: f64,
    buy: f64,
    sell: f64,
}

impl App {
    fn calculate_fee(&self, price: f64) -> f64 {
        // fee is %0.010
        price * 0.001
    }

    fn get_vals(&self) -> Result<Values, Box<dyn Error>> {
        let amount = self.buy_amount.trim().parse::<f64>()?;
        let buy = self.buy_price.trim().parse::<f64>()?;
        let sell = self.sell_price.trim().parse::<f64>()?;
        Ok(Values { amount, buy, sell })
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            buy_amount: "0".to_owned(),
            buy_price: "0".to_owned(),
            sell_price: "0".to_owned(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Crypto Profit Calculator"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let Self {
                buy_amount,
                buy_price,
                sell_price,
            } = self;

            ui.heading("Crypto profit calculator");

            ui.horizontal(|ui| {
                ui.label("Buy Amount: ");
                ui.text_edit_singleline(buy_amount);
                ui.label("BTC");
            });

            ui.horizontal(|ui| {
                ui.label("Buy Price: $");
                ui.text_edit_singleline(buy_price);
            });

            ui.horizontal(|ui| {
                ui.label("Sell Price: $");
                ui.text_edit_singleline(sell_price);
            });

            self.get_vals()
                .map(|vals| {
                    let buy_value = vals.buy * vals.amount;
                    let sell_value = vals.sell * vals.amount;
                    let fee = &self.calculate_fee(sell_value);
                    let profit = (sell_value - buy_value) - fee;
                    ui.label(String::from("Purchase amount paid: $") + &buy_value.to_string());
                    ui.label(String::from("Sell amount paid: $") + &sell_value.to_string());
                    ui.label(String::from("Fee:") + &fee.to_string());
                    ui.label(String::from("Profit:") + &profit.to_string());
                })
                .or_else(|_err| -> Result<(), Box<dyn Error>> {
                    ui.label("Invalid numbers");
                    Ok(())
                })
                .unwrap();
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
