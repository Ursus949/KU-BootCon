use std::process::Command;
use std::process::Output;
use whoami;
use local_ip_address::local_ip;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BootCon {
    target: String,

    #[serde(skip)]
    host: String,
    #[serde(skip)]
    weather: String,
    #[serde(skip)]
    public_ip: Output,
}

impl Default for BootCon {
    fn default() -> Self {
        Self {
            host: "example.com".to_string(),
            target: "127.0.0.1".to_string(),
            weather: "Kansas+City".to_string(),
            public_ip: Command::new("curl")
                                .arg("ipinfo.io/ip")
                                .output()
                                .expect("Public IP? command failed to start"),
        }
    }
}

impl BootCon {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BootCon {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Clear the Terminal").clicked() {

                        if cfg!(target_os = "windows") {
                            Command::new("powershell")
                                    .arg("-c")
                                    .arg("clear")
                                    .spawn()
                                    .expect("Clear Term cmd failed to start");
                        } else {
                            Command::new("clear")
                                    .spawn()
                                    .expect("Clear Term 'else' cmd failed to start");
                        }
                    }
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });

                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Common Commands");

            if ui.button("Hook into Term").clicked() {
                if cfg!(target_os = "windows") {
                    Command::new("powershell.exe")
                            .spawn()
                            .expect("PowerShell command failed to start");
                } else {
                    Command::new("zsh")
                            .spawn()
                            .expect("Terminal command failed to start");
                }
            }

            if ui.button("Local Network Config").clicked() {
                if cfg!(target_os = "windows") {
                    Command::new("ipconfig")
                            .arg("/all")
                            .spawn()
                            .expect("Windows ipconfig command failed to start");
                } else if cfg!(target_os = "macos") {
                    Command::new("ifconfig")
                            .spawn()
                            .expect("Mac's ifconfig command failed to start");
                } else {
                    Command::new("ip")
                            .arg("addr")
                            .arg("show")
                            .spawn()
                            .expect("Linux's `ip addr show` command failed to start");
                }
            }

            ui.separator();
            ui.collapsing("NMAP", |ui| {
            ui.horizontal(|ui| {
                ui.label("Target: ");
                ui.text_edit_singleline(&mut self.target);
            });

            if ui.button("Send it!").clicked() {
                if cfg!(target_os = "windows") {
                    Command::new("nmap")
                            .args(["-sC", "-sV", "-v", &self.target, "-oA", &self.target])
                            .spawn()
                            .expect("NMAP WINDOWS(Hardcoded) command Failed to start.");
                }
                else {
                    Command::new("sudo")
                            .args(["nmap", "-sC", "-sV", "-v", &self.target, "-oA", &self.target])
                            .spawn()
                            .expect("NMAP(Hardcoded) command Failed to start");
                }
            }
});
            ui.separator();
            ui.collapsing("Network Tools", |ui| {
            ui.horizontal(|ui| {
                ui.label("Target: ");
                ui.text_edit_singleline(&mut self.host);
            });

            ui.horizontal(|ui| {
                ui.label("NSLOOKUP:");
            if ui.button("NS").clicked() {
                    Command::new("nslookup")
                            .args(["-type=NS",&self.host])
                            .spawn()
                            .expect("nslookup (NS) command Failed to start.");
            }
            if ui.button("MX").clicked() {
                    Command::new("nslookup")
                            .args(["-type=MX",&self.host])
                            .spawn()
                            .expect("nslookup (MX) command Failed to start.");
            }
            if ui.button("TXT").clicked() {
                    Command::new("nslookup")
                            .args(["-type=TXT",&self.host])
                            .spawn()
                            .expect("nslookup (txt) command Failed to start.");
            }
            if ui.button("ANY").clicked() {
                    Command::new("nslookup")
                            .args(["-type=any",&self.host])
                            .spawn()
                            .expect("nslookup (any) command Failed to start.");
            }
    });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("DIG").clicked() {
                    if cfg!(target_os = "windows") {
                        Command::new("dig")
                                .arg(&self.host)
                                .spawn()
                                .expect("Dig Windows failed to start");
                    } else {
                        Command::new("dig")
                                .arg(&self.host)
                                .spawn()
                                .expect("DIG command failed to start");
                    }
                }

                if ui.button("WHOIS").clicked() {
                    if cfg!(target_os = "windows") {
                        Command::new("whois")
                                .arg(&self.host)
                                .spawn()
                                .expect("whois Windows failed to start");
                    } else {
                        Command::new("whois")
                                .arg(&self.host)
                                .spawn()
                                .expect("WHOIS command failed to start");
                    }
                }

                if ui.button("PING").clicked() {
                    if cfg!(target_os = "windows") {
                        Command::new("ping")
                                .arg(&self.host)
                                .spawn()
                                .expect("PING (windows) command failed to start");
                    } else {
                        Command::new("ping")
                                .args(["-c", "4", &self.host])
                                .spawn()
                                .expect("PING command failed to start");
                    }
                }
            });
            ui.separator();
});

            ui.separator();
            ui.collapsing("Weather", |ui| {
            ui.horizontal(|ui| {
                ui.label("Closest Major City: ");
                ui.text_edit_singleline(&mut self.weather);
            });

            if ui.button("Current Weather").clicked() {
                Command::new("curl")
                        .arg("-s")
                        .arg("http://wttr.in/".to_owned()+&self.weather+"?format=3")
                        .spawn()
                        .expect("Weather (current) command failed to start");
            }
            if ui.button("3-Day Forcast").clicked() {
                Command::new("curl")
                        .arg("-s")
                        .arg("http://wttr.in/".to_owned()+&self.weather)
                        .spawn()
                        .expect("Weather (3-day) command failed to start");
            }
            ui.separator();
});
            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("Curiosity", "https://www.merriam-webster.com/dictionary/curiosity");
                    ui.label(" and ");
                    ui.hyperlink_to("Insomnia", "https://www.mayoclinic.org/diseases-conditions/insomnia/symptoms-causes/syc-20355167");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("KU Cybersecurity 2022");
            ui.hyperlink_to("BootCampSpot", "https://bootcampspot.com/login");
            ui.hyperlink_to("KU GitLab", "https://ku.bootcampcontent.com/");
            egui::warn_if_debug_build(ui);

            ui.separator();

            ui.collapsing("PEAS Download:", |ui| {
            ui.label("    - This button will allow you to download the PEAS, no matter what OS you are using (winPEAS, linPEAS)

        - PEAS = Privilege Escalation Awesome Script

        - PEAS searches for possible paths to escalate privileges

    - Will download the file to your $HOME DIR or the same DIR the app was ran from.");
            ui.hyperlink_to("Hack Tricks","https://book.hacktricks.xyz/");
            ui.horizontal(|ui| {
                if ui.button("Download PEAs").clicked() {
                    if cfg!(target_os = "windows") {
                        Command::new("curl")
                                .arg("-L")
                                .arg("-O")
                                .arg("https://github.com/carlospolop/PEASS-ng/releases/latest/download/winPEAS.bat")
                                .spawn()
                                .expect("WinPEAS command failed to start");
                    } else {
                        Command::new("curl")
                                .arg("-L")
                                .arg("-O")
                                .arg("https://github.com/carlospolop/PEASS-ng/releases/latest/download/linpeas.sh")
                                .spawn()
                                .expect("LinPEAS command failed to start");
                    }
                }
                if ui.button("Run PEAs").clicked() {
                    if cfg!(target_os = "windows") {
                        Command::new("powershell.exe")
                                .arg("-c")
                                .arg(".\\winPEAS.bat")
                                .spawn()
                                .expect("RUN WinPEAS.bat command failed to start");
                    } else {
                        Command::new("sh")
                                .arg("./linpeas.sh")
                                .spawn()
                                .expect("RUN LinPEAS.sh command failed to start");
                    }
                }
            });
});
            ui.separator();
            ui.collapsing("Host Info", |ui| {
                ui.label(format!("Public IP: {}", String::from_utf8_lossy(&self.public_ip.stdout)));
                let local_ip = local_ip().unwrap();
                ui.label(format!("Local IP: {}", local_ip));
                ui.label(format!("Device Platform: {}", whoami::platform()));
                ui.label(format!("OS Distro: {}", whoami::distro()));
                ui.label(format!("Device's 'Pretty' Name: {}", whoami::devicename()));
                ui.label(format!("Hostname: {}", whoami::hostname()));
                ui.label(format!("Desktop Env: {}", whoami::desktop_env()));

                ui.separator();
                ui.heading("User Info:");
                ui.label(format!("User's Name: {}", whoami::realname()));
                ui.label(format!("User's Username: {}", whoami::username()));
                ui.label(format!("User's Language: {:?}", whoami::lang().collect::<Vec<String>>()));
            });
            ui.separator();
            ui.collapsing("Disclaimer:", |ui| {
                ui.label("\t- Nmap is currently hardcoded to run with \"-sC\", \"-sV\", and \"-v\" untill I can get the checkboxes to function correctly");
                ui.label("\t- If running on Windows `NMAP`, `DIG`, and `WHOIS` will make the program crash unless you have those programs installed correctly");
                ui.label("\t\t- You can use Chocolatey on Windows to install DIG and WHOIS");
                ui.label("\t\t- DIG -- `choco install bind-toolsonly`");
                ui.label("\t\t- WHOIS -- `choco install whois`");
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Created by Ursus949");
                });
            });
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
