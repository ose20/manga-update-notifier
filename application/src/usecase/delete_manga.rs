use anyhow::Result;
use domain::command;
use registry::AppRegistry;

pub async fn execute(_registry: AppRegistry, _command: command::DeleteManga) -> Result<()> {
    todo!()
}
