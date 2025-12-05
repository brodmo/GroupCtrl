use crate::app::App;
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSString;

pub trait Open {
    // TODO return Result<(), OpenAppError>
    fn open(&self);
}

impl Open for App {
    fn open(&self) {
        unsafe {
            let workspace = NSWorkspace::sharedWorkspace();
            let bundle_id = NSString::from_str(self.bundle_id.as_str());
            // TODO use thiserror to define legible errors and return them
            let app_url = workspace
                .URLForApplicationWithBundleIdentifier(&bundle_id)
                .expect("Could not find app with this Bundle ID");
            // TODO use openApplicationAtUrl (requires async)
            if !workspace.openURL(&app_url) {
                panic!("macOS failed to launch the application");
            }
        }
    }
}
