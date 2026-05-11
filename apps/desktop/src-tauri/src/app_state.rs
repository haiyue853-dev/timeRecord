use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use anyhow::Result;
use chrono::{DateTime, Duration as ChronoDuration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    config::AppSettings,
    history::{
        build_learning_heatmap, build_session_trend_points, build_weekly_summary, DailyHistory,
        LearningHeatmapCell, SessionTrendBucket, SessionTrendPoint, WeeklySummary,
    },
    summary::{LocalSummaryProvider, SummaryContext},
    tracking::{
        foreground::{ForegroundSource, WindowsForegroundSource},
        idle::read_last_input_at,
        media::{looks_like_media_app, MediaPlaybackState},
        session::ForegroundSnapshot,
    },
};

#[derive(Debug, Clone)]
pub struct DashboardAppItem {
    pub app_name: String,
    pub seconds: i64,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct DashboardSnapshot {
    pub total_active_seconds: i64,
    pub today_active_seconds: i64,
    pub current_app_name: String,
    pub current_window_title: String,
    pub apps: Vec<DashboardAppItem>,
    pub today_apps: Vec<DashboardAppItem>,
    pub summary: String,
    pub encouragement: String,
    pub summary_source: String,
    pub trend_points: Vec<SessionTrendPoint>,
    pub weekly_summary: WeeklySummary,
    pub learning_heatmap: Vec<LearningHeatmapCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsageAggregate {
    app_name: String,
    process_name: String,
    category: String,
    seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DailyAppHistory {
    date: NaiveDate,
    app_name: String,
    category: String,
    seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PersistedHistory {
    current_boot_id: String,
    current_boot_started_at: Option<DateTime<Utc>>,
    current_total_active_seconds: i64,
    current_apps: Vec<UsageAggregate>,
    current_trend_buckets: Vec<SessionTrendBucket>,
    daily_history: Vec<DailyHistory>,
    daily_app_history: Vec<DailyAppHistory>,
}

#[derive(Debug)]
struct TrackingState {
    boot_id: String,
    started_at: DateTime<Utc>,
    last_snapshot: Option<ForegroundSnapshot>,
    last_tick_at: Option<DateTime<Utc>>,
    total_active_seconds: i64,
    current_app_name: String,
    current_window_title: String,
    apps: HashMap<String, UsageAggregate>,
    trend_buckets: BTreeMap<DateTime<Utc>, i64>,
}

impl TrackingState {
    fn new() -> Self {
        let started_at = current_boot_started_at();
        let boot_id = current_boot_id(started_at);

        Self {
            boot_id,
            started_at,
            last_snapshot: None,
            last_tick_at: None,
            total_active_seconds: 0,
            current_app_name: "等待活动采集".into(),
            current_window_title: "等待检测到前台窗口".into(),
            apps: HashMap::new(),
            trend_buckets: BTreeMap::new(),
        }
    }

    fn hydrate_from_history(&mut self, history: &PersistedHistory) {
        if history.current_boot_id != self.boot_id {
            return;
        }

        if let Some(started_at) = history.current_boot_started_at {
            self.started_at = started_at;
        }
        self.total_active_seconds = history.current_total_active_seconds;
        self.apps = history
            .current_apps
            .iter()
            .cloned()
            .map(|item| (item.app_name.clone(), item))
            .collect();
        self.trend_buckets = history
            .current_trend_buckets
            .iter()
            .map(|bucket| (bucket.bucket_started_at, bucket.active_seconds))
            .collect();

        if let Some(top_app) = history
            .current_apps
            .iter()
            .max_by_key(|item| item.seconds)
            .cloned()
        {
            self.current_app_name = top_app.app_name;
        }
    }

    fn observe(
        &mut self,
        snapshot: ForegroundSnapshot,
        settings: &AppSettings,
        history: &mut PersistedHistory,
    ) {
        let now = snapshot.captured_at;
        let last_input_at = read_last_input_at(now).unwrap_or(now);

        if let (Some(previous), Some(last_tick_at)) = (&self.last_snapshot, self.last_tick_at) {
            let elapsed = (now - last_tick_at).num_seconds().clamp(1, 5);
            let category = categorize_activity(&previous.process_name, &previous.window_title);
            let count_as_active = should_count_as_active(
                now,
                last_input_at,
                &previous.process_name,
                settings.idle_seconds,
            );

            if count_as_active && elapsed > 0 {
                self.total_active_seconds += elapsed;

                let entry = self
                    .apps
                    .entry(previous.app_name.clone())
                    .or_insert_with(|| UsageAggregate {
                        app_name: previous.app_name.clone(),
                        process_name: previous.process_name.clone(),
                        category: category.to_string(),
                        seconds: 0,
                    });
                entry.seconds += elapsed;
                entry.process_name = previous.process_name.clone();
                entry.category = category.to_string();

                let bucket_started_at = trend_bucket_started_at(self.started_at, now);
                *self.trend_buckets.entry(bucket_started_at).or_insert(0) += elapsed;

                update_daily_history(
                    &mut history.daily_history,
                    now.date_naive(),
                    elapsed,
                    category,
                );
                update_daily_app_history(
                    &mut history.daily_app_history,
                    now.date_naive(),
                    &previous.app_name,
                    category,
                    elapsed,
                );

                history.current_boot_id = self.boot_id.clone();
                history.current_boot_started_at = Some(self.started_at);
                history.current_total_active_seconds = self.total_active_seconds;
                history.current_apps = self.apps.values().cloned().collect();
                history.current_trend_buckets = self
                    .trend_buckets
                    .iter()
                    .map(|(bucket_started_at, active_seconds)| SessionTrendBucket {
                        bucket_started_at: *bucket_started_at,
                        active_seconds: *active_seconds,
                    })
                    .collect();
            }
        }

        self.current_app_name = snapshot.app_name.clone();
        self.current_window_title = if snapshot.window_title.trim().is_empty() {
            format!("{} 正在前台运行", snapshot.app_name)
        } else {
            snapshot.window_title.clone()
        };
        self.last_tick_at = Some(now);
        self.last_snapshot = Some(snapshot);
    }

    fn view(&self) -> DashboardSnapshotView {
        let mut apps = self.apps.values().cloned().collect::<Vec<_>>();
        apps.sort_by(|left, right| right.seconds.cmp(&left.seconds));

        DashboardSnapshotView {
            total_active_seconds: self.total_active_seconds,
            current_app_name: self.current_app_name.clone(),
            current_window_title: self.current_window_title.clone(),
            apps,
            started_at: self.started_at,
            trend_buckets: self
                .trend_buckets
                .iter()
                .map(|(bucket_started_at, active_seconds)| SessionTrendBucket {
                    bucket_started_at: *bucket_started_at,
                    active_seconds: *active_seconds,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct DashboardSnapshotView {
    total_active_seconds: i64,
    current_app_name: String,
    current_window_title: String,
    apps: Vec<UsageAggregate>,
    started_at: DateTime<Utc>,
    trend_buckets: Vec<SessionTrendBucket>,
}

#[derive(Clone)]
pub struct AppState {
    settings: Arc<Mutex<AppSettings>>,
    settings_path: Arc<Mutex<Option<PathBuf>>>,
    history_path: Arc<Mutex<Option<PathBuf>>>,
    tracking: Arc<Mutex<TrackingState>>,
    history: Arc<Mutex<PersistedHistory>>,
    tracking_started: Arc<AtomicBool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(Mutex::new(AppSettings::default())),
            settings_path: Arc::new(Mutex::new(None)),
            history_path: Arc::new(Mutex::new(None)),
            tracking: Arc::new(Mutex::new(TrackingState::new())),
            history: Arc::new(Mutex::new(PersistedHistory::default())),
            tracking_started: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn new_for_test() -> Result<Self> {
        Ok(Self::new())
    }

    pub fn initialize(&self, app_data_dir: PathBuf) -> Result<()> {
        fs::create_dir_all(&app_data_dir)?;
        let settings_path = app_data_dir.join("settings.json");
        let history_path = app_data_dir.join("history.json");

        if settings_path.exists() {
            let contents = fs::read_to_string(&settings_path)?;
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&contents) {
                self.replace_settings_internal(settings);
            }
        }

        if history_path.exists() {
            let contents = fs::read_to_string(&history_path)?;
            if let Ok(history) = serde_json::from_str::<PersistedHistory>(&contents) {
                self.replace_history_internal(history);
            }
        }

        *self.settings_path.lock().expect("settings path mutex poisoned") = Some(settings_path);
        *self.history_path.lock().expect("history path mutex poisoned") = Some(history_path);

        if self
            .settings_path
            .lock()
            .expect("settings path mutex poisoned")
            .as_ref()
            .is_some()
        {
            self.persist_settings(&self.settings())?;
        }
        if self
            .history_path
            .lock()
            .expect("history path mutex poisoned")
            .as_ref()
            .is_some()
        {
            self.persist_history()?;
        }

        let history = self.history.lock().expect("history mutex poisoned").clone();
        self.tracking
            .lock()
            .expect("tracking mutex poisoned")
            .hydrate_from_history(&history);

        Ok(())
    }

    pub fn start_tracking_loop(&self) {
        if self.tracking_started.swap(true, Ordering::SeqCst) {
            return;
        }

        let state = self.clone();
        thread::spawn(move || {
            let source = WindowsForegroundSource;

            loop {
                if let Ok(snapshot) = source.snapshot() {
                    state.record_snapshot(snapshot);
                }

                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    pub fn settings(&self) -> AppSettings {
        self.settings.lock().expect("settings mutex poisoned").clone()
    }

    pub fn replace_settings(&self, settings: AppSettings) -> Result<()> {
        self.replace_settings_internal(settings.clone());
        self.persist_settings(&settings)?;
        Ok(())
    }

    pub fn dashboard_snapshot(&self) -> DashboardSnapshot {
        let tracker = self.tracking.lock().expect("tracking mutex poisoned");
        let history = self.history.lock().expect("history mutex poisoned");
        let view = tracker.view();
        let today = Utc::now().date_naive();
        let weekly_summary = build_weekly_summary(today, &history.daily_history);
        let learning_heatmap = build_learning_heatmap(today, 28, &history.daily_history);
        let summary_bundle = LocalSummaryProvider::new(view.started_at.timestamp().unsigned_abs())
            .generate(&build_summary_context(today, &history.daily_history, &weekly_summary));
        let today_active_seconds = history
            .daily_history
            .iter()
            .find(|item| item.date == today)
            .map(|item| item.total_active_seconds)
            .unwrap_or(0);
        let today_apps = build_today_apps(today, &history.daily_app_history);

        DashboardSnapshot {
            total_active_seconds: view.total_active_seconds,
            today_active_seconds,
            current_app_name: view.current_app_name,
            current_window_title: view.current_window_title,
            apps: if view.apps.is_empty() {
                vec![placeholder_app()]
            } else {
                view.apps
                    .into_iter()
                    .map(|item| DashboardAppItem {
                        app_name: item.app_name,
                        seconds: item.seconds,
                        category: item.category,
                    })
                    .collect()
            },
            today_apps: if today_apps.is_empty() {
                vec![placeholder_app()]
            } else {
                today_apps
            },
            summary: summary_bundle.summary,
            encouragement: summary_bundle.encouragement,
            summary_source: summary_bundle.source,
            trend_points: build_session_trend_points(&view.trend_buckets),
            weekly_summary,
            learning_heatmap,
        }
    }

    pub fn summary_context(&self) -> SummaryContext {
        let history = self.history.lock().expect("history mutex poisoned");
        let today = Utc::now().date_naive();
        let weekly_summary = build_weekly_summary(today, &history.daily_history);

        build_summary_context(today, &history.daily_history, &weekly_summary)
    }

    fn record_snapshot(&self, snapshot: ForegroundSnapshot) {
        let settings = self.settings();
        let mut tracking = self.tracking.lock().expect("tracking mutex poisoned");
        let mut history = self.history.lock().expect("history mutex poisoned");
        tracking.observe(snapshot, &settings, &mut history);
        drop(history);
        drop(tracking);

        let _ = self.persist_history();
    }

    fn replace_settings_internal(&self, settings: AppSettings) {
        *self.settings.lock().expect("settings mutex poisoned") = settings;
    }

    fn replace_history_internal(&self, history: PersistedHistory) {
        *self.history.lock().expect("history mutex poisoned") = history;
    }

    fn persist_settings(&self, settings: &AppSettings) -> Result<()> {
        if let Some(path) = self
            .settings_path
            .lock()
            .expect("settings path mutex poisoned")
            .clone()
        {
            let contents = serde_json::to_string_pretty(settings)?;
            fs::write(path, contents)?;
        }

        Ok(())
    }

    fn persist_history(&self) -> Result<()> {
        if let Some(path) = self
            .history_path
            .lock()
            .expect("history path mutex poisoned")
            .clone()
        {
            let history = self.history.lock().expect("history mutex poisoned").clone();
            let contents = serde_json::to_string_pretty(&history)?;
            fs::write(path, contents)?;
        }

        Ok(())
    }
}

fn placeholder_app() -> DashboardAppItem {
    DashboardAppItem {
        app_name: "等待活动采集".into(),
        seconds: 0,
        category: "system".into(),
    }
}

fn build_today_apps(date: NaiveDate, items: &[DailyAppHistory]) -> Vec<DashboardAppItem> {
    let mut apps = items
        .iter()
        .filter(|item| item.date == date)
        .cloned()
        .collect::<Vec<_>>();
    apps.sort_by(|left, right| right.seconds.cmp(&left.seconds));

    apps.into_iter()
        .map(|item| DashboardAppItem {
            app_name: item.app_name,
            seconds: item.seconds,
            category: item.category,
        })
        .collect()
}

fn should_count_as_active(
    now: DateTime<Utc>,
    last_input_at: DateTime<Utc>,
    process_name: &str,
    idle_seconds: u64,
) -> bool {
    crate::tracking::idle::IdleDecision::from_inputs(
        now,
        last_input_at,
        looks_like_media_app(process_name),
        MediaPlaybackState::Unknown,
        idle_seconds,
    )
    .count_as_active
}

fn categorize_activity(process_name: &str, window_title: &str) -> &'static str {
    let process_name = process_name.to_ascii_lowercase();
    let title = window_title.to_ascii_lowercase();

    if matches!(
        process_name.as_str(),
        "code.exe" | "devenv.exe" | "idea64.exe" | "pycharm64.exe" | "webstorm64.exe"
    ) {
        return "development";
    }

    if matches!(
        process_name.as_str(),
        "chrome.exe" | "msedge.exe" | "firefox.exe"
    ) && contains_any(
        &title,
        &[
            "course",
            "lesson",
            "study",
            "bilibili",
            "mooc",
            "class",
            "课程",
            "学习",
            "网课",
            "慕课",
        ],
    ) {
        return "learning";
    }

    if matches!(
        process_name.as_str(),
        "chrome.exe" | "msedge.exe" | "firefox.exe" | "wechat.exe" | "qq.exe"
    ) {
        return "communication";
    }

    if matches!(process_name.as_str(), "vlc.exe" | "potplayer.exe") {
        return "media";
    }

    "general"
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn update_daily_history(
    items: &mut Vec<DailyHistory>,
    date: NaiveDate,
    elapsed_seconds: i64,
    category: &str,
) {
    if let Some(day) = items.iter_mut().find(|item| item.date == date) {
        day.total_active_seconds += elapsed_seconds;
        if category == "learning" {
            day.learning_seconds += elapsed_seconds;
        }
        if category == "development" {
            day.development_seconds += elapsed_seconds;
        }
        return;
    }

    let mut day = DailyHistory {
        date,
        total_active_seconds: elapsed_seconds,
        learning_seconds: 0,
        development_seconds: 0,
    };

    if category == "learning" {
        day.learning_seconds = elapsed_seconds;
    }
    if category == "development" {
        day.development_seconds = elapsed_seconds;
    }

    items.push(day);
    items.sort_by_key(|item| item.date);
}

fn update_daily_app_history(
    items: &mut Vec<DailyAppHistory>,
    date: NaiveDate,
    app_name: &str,
    category: &str,
    elapsed_seconds: i64,
) {
    if let Some(app) = items
        .iter_mut()
        .find(|item| item.date == date && item.app_name == app_name)
    {
        app.seconds += elapsed_seconds;
        app.category = category.to_string();
        return;
    }

    items.push(DailyAppHistory {
        date,
        app_name: app_name.to_string(),
        category: category.to_string(),
        seconds: elapsed_seconds,
    });
    items.sort_by(|left, right| left.date.cmp(&right.date));
}

fn trend_bucket_started_at(session_started_at: DateTime<Utc>, now: DateTime<Utc>) -> DateTime<Utc> {
    let elapsed_seconds = (now - session_started_at).num_seconds().max(0);
    let bucket_index = elapsed_seconds / 300;
    session_started_at + ChronoDuration::seconds(bucket_index * 300)
}

fn build_summary_context(
    today: NaiveDate,
    items: &[DailyHistory],
    weekly_summary: &WeeklySummary,
) -> SummaryContext {
    let yesterday = find_day(items, today - ChronoDuration::days(1));
    let previous_day = find_day(items, today - ChronoDuration::days(2));

    SummaryContext {
        yesterday_total_seconds: yesterday.total_active_seconds,
        yesterday_learning_seconds: yesterday.learning_seconds,
        yesterday_development_seconds: yesterday.development_seconds,
        previous_day_total_seconds: previous_day.total_active_seconds,
        current_week_total_seconds: weekly_summary.current_week_total_seconds,
        current_week_average_seconds: weekly_summary.current_week_average_seconds,
    }
}

fn find_day(items: &[DailyHistory], date: NaiveDate) -> DailyHistory {
    items
        .iter()
        .find(|item| item.date == date)
        .cloned()
        .unwrap_or(DailyHistory {
            date,
            total_active_seconds: 0,
            learning_seconds: 0,
            development_seconds: 0,
        })
}

#[cfg(target_os = "windows")]
fn current_boot_started_at() -> DateTime<Utc> {
    use windows::Win32::System::SystemInformation::GetTickCount64;

    let uptime_millis = unsafe { GetTickCount64() } as i64;
    Utc::now() - ChronoDuration::milliseconds(uptime_millis)
}

#[cfg(not(target_os = "windows"))]
fn current_boot_started_at() -> DateTime<Utc> {
    Utc::now()
}

fn current_boot_id(started_at: DateTime<Utc>) -> String {
    started_at.format("boot-%Y%m%d-%H%M%S").to_string()
}
