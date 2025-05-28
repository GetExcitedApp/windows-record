pub mod audio;
pub mod video;

pub use audio::*;
pub use video::*;

use log::info;
use windows::Win32::{
  Foundation::{CO_E_ALREADYINITIALIZED, RPC_E_CHANGED_MODE},
  System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
};

fn ensure_com_initialized() -> windows::core::Result<()> {
  let coinit_result = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
  if let Err(e) = coinit_result {
    // Accept both already initialized and changed mode errors (Electron compatibility)
    if e.code() != CO_E_ALREADYINITIALIZED && e.code() != RPC_E_CHANGED_MODE {
      return Err(e);
    }
    info!("COM already initialized (possibly with different threading model by Electron)");
  }
  Ok(())
}
