use std::fs::{set_permissions, File};
use std::thread::{sleep, spawn};
use std::time::Duration;

#[test]
//#[should_panic]
fn soft_error() {
    let mortem = mortem::soft();

    let handler = spawn(|| {
        let path = std::env::current_exe().expect("failed to get current exe");
        let exe = File::open(&path).expect("failed to open self");
        let metadata = exe.metadata().expect("failed to get metadata");
        let original_permissions = metadata.permissions();

        let mut readonly = original_permissions.clone();
        readonly.set_readonly(true);

        dbg!(&path);
        dbg!(&original_permissions);
        dbg!(&readonly);
        set_permissions(&path, readonly).expect("failed to set readonly");

        sleep(Duration::from_secs(4));

        //set_permissions(&path, original_permissions).expect("failed to revert permissions");
    });

    sleep(Duration::from_secs(2));
    drop(mortem); // simulate exiting main

    handler.join().expect("failed to wait on handler");
}
