use fobword_core::app::App;
use fobword_core::yaml_config::Config;

mod db;

fn main() -> std::io::Result<()>
{
    let config = Config::default();
    let mut app = App::new(config)?;
    app.main_loop()?;
    Ok(())
}

