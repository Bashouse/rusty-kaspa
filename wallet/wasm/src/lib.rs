use bascoin_cli_lib::bascoin_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_bascoin_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    bascoin_cli(options, None).await?;
    Ok(())
}
