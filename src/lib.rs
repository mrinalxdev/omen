use anyhow::Result;
use askama::Template;
use std::fs;
use std::path::Path;

#[derive(Template)]
#[template(path = "main.rs.txt")]
struct Maintemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "lib.rs.txt")]
struct LibTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "Cargo.toml.txt")]
struct CargoTemplate {
    name: String,
}

pub fn build_micro(name: &str) -> Result<()> {
    let project_dir = Path::new(name);
    fs::create_dir(project_dir)?;
    fs::create_dir(project_dir.join("src"))?;

    let main_template = Maintemplate {
        name: name.to_string(),
    };
    fs::write(project_dir.join("src/main.rs"), main_template.render()?)?;
    let lib_template = LibTemplate {
        name: name.to_string(),
    };
    fs::write(project_dir.join("src/lib.rs"), lib_template.render()?)?;

    let cargo_template = CargoTemplate {
        name: name.to_string(),
    };
    fs::write(project_dir.join("Cargo.toml"), cargo_template.render()?)?;

    Ok(())
}

mod templates {
    use askama::Template;

    #[derive(Template)]
    #[template(path = "main.rs.txt")]
    pub(crate) struct MainTemplate {
        pub name: String,
    }

    #[derive(Template)]
    #[template(path = "lib.rs.txt")]
    pub(crate) struct LibTemplate {
        pub name: String,
    }

    #[derive(Template)]
    #[template(path = "Cargo.toml.txt")]
    pub(crate) struct CargoTemplate {
        pub name: String,
    }
}
