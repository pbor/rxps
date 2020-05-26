use clap::{crate_version, App};
use env_logger::Env;
use log::info;
use rxps::{CairoRenderer, XPS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("rxps dump")
        .version(crate_version!())
        .about("dump xps information")
        .args_from_usage(
            "--log-level=[LEVEL] 'Control verbosity of the logs'
             [FILE]              'XPS file'
            ",
        );

    let matches = app.get_matches();

    // FIXME: we do not handle invalid values
    let log_level = matches.value_of("log-level").unwrap_or("info");
    let _ = env_logger::from_env(Env::default().default_filter_or(log_level)).try_init();

    let file = matches.value_of("FILE").expect("Specify XPS file");

    let xps = XPS::load(file)?;

    info!(
        "Loaded XPS file {}, it contains {} documents",
        file,
        xps.documents().len()
    );

    for (i, d) in xps.documents().iter().enumerate() {
        info!("document #{} contains {} pages", i + 1, d.pages().len());

        if let Some(outline) = d.outline() {
            info!(
                "document #{} outline contains {} entries",
                i + 1,
                outline.entries().len()
            );
        }
    }

    // Render a page for testing purposes

    let p = &xps.documents()[0].pages()[0];
    let (w, h) = p.size();
    let s = cairo::ImageSurface::create(cairo::Format::ARgb32, w as i32, h as i32).unwrap();
    let cr = cairo::Context::new(&s);
    let r = CairoRenderer::new(cr);
    p.render(&r)?;

    Ok(())
}
