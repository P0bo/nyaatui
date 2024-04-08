use serde::{Deserialize, Serialize};

use crate::{app::Context, source::Item, util::CommandBuilder};

use super::ClientConfig;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CmdConfig {
    cmd: String,
    shell_cmd: String,
}

impl Default for CmdConfig {
    fn default() -> Self {
        CmdConfig {
            #[cfg(windows)]
            cmd: "curl \"{torrent}\" -o ~\\Downloads\\{file}".to_owned(),
            #[cfg(unix)]
            cmd: "curl \"{torrent}\" > ~/{file}".to_owned(),

            shell_cmd: CommandBuilder::default_shell(),
            // #[cfg(windows)]
            // shell_cmd: "powershell.exe -Command".to_owned(),
            // #[cfg(unix)]
            // shell_cmd: "sh -c".to_owned(),
        }
    }
}

pub fn load_config(app: &mut Context) {
    if app.config.client.cmd.is_none() {
        let mut def = CmdConfig::default();
        // Replace deprecated torrent_client_cmd with client.command config
        if let Some(cmd) = app.config.torrent_client_cmd.clone() {
            def.cmd = cmd;
        }
        app.config.client.cmd = Some(def);
        app.config.torrent_client_cmd = None;
    }
}

pub async fn download(item: Item, conf: ClientConfig) -> Result<String, String> {
    // load_config(app);
    let cmd = match conf.cmd.to_owned() {
        Some(c) => c,
        None => {
            return Err("Failed to get cmd config".to_owned());
        }
    };
    match CommandBuilder::new(cmd.cmd)
        .sub("{magnet}", &item.magnet_link)
        .sub("{torrent}", &item.torrent_link)
        .sub("{title}", &item.title)
        .sub("{file}", &item.file_name)
        .run(cmd.shell_cmd)
    {
        Ok(_) => Ok("Successfully ran command".to_owned()),
        Err(e) => Err(e.to_string()),
    }
}
