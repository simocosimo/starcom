#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    sync::{mpsc::{self, Sender, TryRecvError}, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use eframe::egui;
use egui::Ui;

fn main() {
    let options = eframe::NativeOptions::default();
    let app = MyApp::default();
    _ = eframe::run_native(
        "Test",
        options,
        Box::new(move |_cc| Box::new(app)),
    );
}
 
struct MyApp {
    field: Arc<Mutex<i128>>,
    counting: Arc<Mutex<bool>>,
    thread: Option<JoinHandle<()>>,
    tx: Option<Sender<bool>>
}
 
impl Default for MyApp {
    fn default() -> Self {
        Self { 
            field: Arc::new(Mutex::new(0)),
            counting: Arc::new(Mutex::new(false)),
            thread: None,
            tx: None
        }
    }
}
 
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(format!("{}", self.field.lock().unwrap()));
                let local_bool = *self.counting.lock().unwrap();
                match local_bool {
                    false => ui_start_counting(ui, ctx, self),
                    true => ui_stop_counting(ui, ctx, self)
                }
            }
        );
    }
}

fn ui_start_counting(ui: &mut Ui, ctx: &egui::Context, myapp: &mut MyApp) -> () {
    if ui.button("Start Counting").clicked() {
        *myapp.counting.lock().unwrap() = true;

        let th_ctx = ctx.clone();
        let th_data = myapp.field.clone();

        match myapp.thread {
            None => {
                let (tx, rx) = mpsc::channel::<bool>();
                myapp.tx = Some(tx);
                myapp.thread = Some(thread::spawn(move || {
                    loop {
                        match rx.try_recv() {
                            Ok(_) | Err(TryRecvError::Disconnected) => {
                                break;
                            }
                            Err(TryRecvError::Empty) => {}
                        }
                        thread::sleep(Duration::from_secs(1));
                        let mut th_field = th_data.lock().unwrap();
                        *th_field += 1;
                        th_ctx.request_repaint();
                    }
                    ()
                }));
            },
            Some(_) => {
                unreachable!()
            }
        }
    };
}

fn ui_stop_counting(ui: &mut Ui, ctx: &egui::Context, myapp: &mut MyApp) -> () {
    if ui.button("Stop Counting").clicked() {
        *myapp.counting.lock().unwrap() = false;
        match &myapp.tx {
            Some(txx) => {
                let _ = txx.send(false);
                let thh = myapp.thread.take().expect("no thread");
                let _ = thh.join();
            },
            None => {}
        }
    };
    ctx.request_repaint();
}