use config::Config;

#[test]
fn wrapping_u16() {
    let c = Config::builder()
        .add_source(config::File::from_str(
            r#"
            [settings]
            port = 66000
            "#,
            config::FileFormat::Toml,
        ))
        .build()
        .unwrap();

    let port: u16 = c.get("settings.port").unwrap();
    assert_eq!(port, 464);
}

#[test]
fn nonwrapping_u32() {
    let c = Config::builder()
        .add_source(config::File::from_str(
            r#"
            [settings]
            port = 66000
            "#,
            config::FileFormat::Toml,
        ))
        .build()
        .unwrap();

    let port: u32 = c.get("settings.port").unwrap();
    assert_eq!(port, 66000);
}
