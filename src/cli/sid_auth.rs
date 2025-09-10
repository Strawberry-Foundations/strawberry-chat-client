use tokio::time::{self, Duration};

use libstrawberry::colors::{BLUE, BOLD, C_RESET, CYAN, GREEN, RED, RESET, YELLOW};
use libstrawberry::id::error::CredentialsError;
use libstrawberry::id::StrawberryId;

use crate::constants::STRAWBERRY_ID_API;
use crate::global::STRINGS;


pub async fn login() -> eyre::Result<()> {
    println!("--- {CYAN}{BOLD}Strawberry ID {}{C_RESET} ---", STRINGS.get("Auth"));

    let code = StrawberryId::request_code().await?;

    println!("{} \n{BOLD}{BLUE}{STRAWBERRY_ID_API}de/login/oauth_dialog/stbchat?code={code}{C_RESET}", STRINGS.get("ContinueSidLogin"));

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        match StrawberryId::callback(code.clone()).await {
            Ok(id) => {
                if let Some(id) = id {
                    println!("{GREEN}{BOLD}{}{C_RESET}", STRINGS.get("AuthSuccess"));
                    println!("{GREEN}{BOLD}{} {} (@{}){C_RESET}", STRINGS.get("LoggedInAs"), id.full_name, id.username);

                    let credentials = id.to_credentials();
                    match credentials.save() {
                        Ok(..) => {}
                        Err(err) => {
                            match err.downcast::<CredentialsError>()? {
                                CredentialsError::AlreadyExists(path) => {
                                    println!("{YELLOW}{BOLD}{}{C_RESET}", STRINGS.get_with_params("AlreadyLoggedIn", &[&path.parent().unwrap().display()]));
                                }
                                CredentialsError::DirectoryCreationError(err) => {
                                    eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} {err}{C_RESET}");
                                }
                                CredentialsError::WriteError(err) => {
                                    eprintln!("{RED}{BOLD}Error while writing file:{RESET} {err}{C_RESET}");
                                }
                                CredentialsError::SerializeError(err) => {
                                    eprintln!("{RED}{BOLD}Error while serializing data:{RESET} {err}{C_RESET}");
                                }
                                CredentialsError::HomeNotFound => {
                                    eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} Home directory not found.{C_RESET}");
                                }
                                CredentialsError::MissingCredentials => unreachable!(),
                            }
                        }
                    }
                    break
                }
            },
            Err(..) => {
                std::process::exit(1);
            }
        }

        interval.tick().await;
    }

    Ok(())
}