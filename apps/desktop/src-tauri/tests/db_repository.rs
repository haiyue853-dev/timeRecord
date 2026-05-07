use timerecord_lib::config::AppSettings;
use timerecord_lib::db::repository::SettingsRepository;

#[test]
fn saves_and_loads_settings() {
    let conn = timerecord_lib::db::open_in_memory().unwrap();
    timerecord_lib::db::run_migrations(&conn).unwrap();

    let repo = SettingsRepository::new(conn);
    let settings = AppSettings {
        idle_seconds: 180,
        ai_enabled: false,
        deepseek_base_url: "https://api.deepseek.com".into(),
        deepseek_model: "deepseek-chat".into(),
    };

    repo.save(&settings).unwrap();
    let loaded = repo.load().unwrap();

    assert_eq!(loaded.idle_seconds, 180);
    assert!(!loaded.ai_enabled);
    assert_eq!(loaded.deepseek_model, "deepseek-chat");
}
