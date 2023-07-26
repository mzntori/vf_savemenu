use crate::errors::Error;

use std::env;
use std::path::PathBuf;
use std::fs;
use std::fs::File;

use chrono::prelude::Local;

use directories::BaseDirs;

static VFMENU_FOLDER_NAME: &str = "VFMenu";

pub fn create_vf_menu_dir() -> Result<(), Error> {
    let vfmenu_path = vf_root_dir()
        .ok_or(Error::LocalLowNotFound)?
        .join(VFMENU_FOLDER_NAME);
    
    if !vfmenu_path.is_dir() {
        fs::create_dir(vfmenu_path)?
    }
    
    Ok(())
}

#[allow(unused)]
pub fn is_allowed_os() -> Result<(), Error> {
    return if env::consts::OS == "windows" {
        Ok(())
    } else {
        Err(Error::MismatchedOS)
    }
}

pub fn vf_root_dir() -> Option<PathBuf> {
    return if let Some(base_dirs) = BaseDirs::new() {
        Some(
            base_dirs
                .home_dir()
                .join("AppData")
                .join("LocalLow")
                .join("Sad Owl Studios")
                .join("Viewfinder")
        )
    } else {
        None
    }
}

pub fn save_file_name(slot: u8, extension: String) -> String {
    format!("viewfinder_{}.{}", slot, extension)
}

pub fn sav_file(slot: u8) -> PathBuf {
    vf_root_dir()
        .unwrap()
        .join(save_file_name(slot, "sav".to_string()))
}

pub fn bak_file(slot: u8) -> PathBuf {
    vf_root_dir()
        .unwrap()
        .join(save_file_name(slot, "bak".to_string()))
}

#[derive(Debug, Clone)]
pub struct Save {
    bak: PathBuf,
    sav: PathBuf,
    slot: u8,
}

impl Save {
    pub fn at_slot(slot: u8) -> Self {
        Save {
            bak: bak_file(slot),
            sav: sav_file(slot),
            slot
        }
    }
    
    pub fn create_recovery_save(&self) -> Result<(), Error> {
        if !self.sav.is_file() {
            return Ok(());
        }
        
        let recovery_dir_path = vf_root_dir()
            .ok_or(Error::LocalLowNotFound)?
            .join(VFMENU_FOLDER_NAME)
            .join(Local::now().format("%Y-%m-%dT%H-%M-%SZ").to_string());
        
        let recovery_sav = recovery_dir_path
            .join(format!("viewfinder_{}.sav", self.slot));
        
        // File creation
        if !recovery_dir_path.is_dir() {
            fs::create_dir(recovery_dir_path.as_path())?;
        }
        File::create(recovery_sav.as_path())?;
        
        // Copy the file
        fs::copy(self.sav.as_path(), recovery_sav.as_path())?;
        
        Ok(())
    }
    
    pub fn remove_files(&self) -> Result<(), Error> {
        let mut r: Result<(), Error> = Ok(());
        
        if let Err(_) = fs::remove_file(self.bak.as_path()) {
            r = Err(Error::FailedFileRemoval)
        }
        if let Err(_) = fs::remove_file(self.sav.as_path()) {
            r = Err(Error::FailedFileRemoval)
        }
        
        r
    }
    
    #[allow(unused)]
    pub fn exists(&self) -> bool {
        return if self.bak.is_file() && self.sav.is_file() {
            true
        } else {
            false
        }
    }
}