use fobword_core::app::App;
use fobword_core::yaml_config::Config;

fn main() -> std::io::Result<()>
{
    let config = Config::default();
    let mut app = App::new(config)?;
    app.main_loop()?;
    Ok(())
}

