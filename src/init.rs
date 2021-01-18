use super::settings::InitMode;

pub struct Init {
}

impl Init {
    pub fn new(init_mode: &InitMode) {
        match init_mode {
            InitMode::Bash => {
                Init::init_bash();
            }
            InitMode::Zsh => {
                Init::init_zsh();
            }
            InitMode::Fish => {
                Init::init_fish();
            }
        }
    }
    pub fn init_bash() {   
        let script = include_str!("../mcfly.bash");
        print!("{}", script);
    }
    pub fn init_zsh() {   
        let script = include_str!("../mcfly.zsh");
        print!("{}", script);
    }
    pub fn init_fish() {   
        let script = include_str!("../mcfly.fish");
        print!("{}", script);
    }
}