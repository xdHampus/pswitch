//https://rust-cli.github.io/book/tutorial/index.html
#![allow(unused)]

mod profile;

use clap::{builder::Str, Parser};
use profile::manager::ProfileManagerDefault;
use profile::Profile;
use profile::ProfileManager;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The profile type to use
    #[arg(name = "profile-type", default_value = &"git", required = true)]
    profile_type: String,
    /// The specific profile to use
    #[arg(name = "profile-title", default_value = &"default", required = true)]
    profile_title: String,
    /// Whether to apply the profile globally or not
    #[arg(name = "global", long, short = 'g', action)]
    global_scope: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut profile_manager: ProfileManagerDefault = ProfileManagerDefault::new();
    profile_manager.parse_config("/home/personal/.config/pswitch/config.ini")?;

    let profiles = profile_manager.get_profiles_with_title(&args.profile_type, &args.profile_title);

    if profiles.len() == 0 {
        return Err("No profiles found".into());
    } else {
        println!(
            "Found {} profile{}",
            profiles.len(),
            if profiles.len() > 1 { "s" } else { "" }
        );
    }
    let profile = profiles.get(0).unwrap();

    profile.apply()?;

    println!(
        "The {} {} profile was applied",
        profile.title().unwrap(),
        profile.profile_type()
    );

    Ok(())
}
