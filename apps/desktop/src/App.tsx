import { useEffect } from "react";
import {
  enable as enableAutostart,
  isEnabled as isAutostartEnabled,
} from "@tauri-apps/plugin-autostart";
import { useRoutes } from "react-router-dom";
import { generateBootSummary, getDashboardStats, getSettings, updateSettings } from "./lib/tauri";
import { routes } from "./routes";
import { useSettingsStore } from "./store/useSettingsStore";
import { useStatsStore } from "./store/useStatsStore";

const THREE_HOURS_MS = 3 * 60 * 60 * 1000;
const STATS_REFRESH_MS = 30 * 1000;
const TODAY_NOTES = [
  "先把今天最重要的一件事留在前台，别让注意力先散开。",
  "先稳住一个小目标，节奏一旦建立，后面会轻松很多。",
  "不用一下子做很多，只要让今天的第一段专注真正发生。",
  "把注意力当成预算，优先投给最值得的那一块。",
  "先完成，再优化。今天能往前拱一点，就已经很有价值。",
];

export function shouldRefreshSummary(lastRefreshedAt: number | null, now: number) {
  return lastRefreshedAt === null || now - lastRefreshedAt >= THREE_HOURS_MS;
}

export function getTodayNote(now: number) {
  const dayKey = new Date(now).toISOString().slice(0, 10);
  const seed = dayKey
    .split("-")
    .join("")
    .split("")
    .reduce((sum, digit) => sum + Number(digit), 0);
  return TODAY_NOTES[seed % TODAY_NOTES.length];
}

export function shouldRunForegroundRefresh() {
  return document.visibilityState === "visible" && document.hasFocus();
}

function AppBootstrap() {
  const setStats = useStatsStore((state) => state.setStats);
  const setSummaryBundle = useStatsStore((state) => state.setSummaryBundle);
  const setTodayNote = useStatsStore((state) => state.setTodayNote);
  const hydrateSettings = useSettingsStore((state) => state.hydrate);
  const setAutostartEnabled = useSettingsStore((state) => state.setAutostartEnabled);

  useEffect(() => {
    let cancelled = false;
    let lastSummaryRefreshedAt: number | null = null;
    let lastTodayNoteDayKey: string | null = null;
    let timer: number | null = null;

    const syncTodayNote = (now: number) => {
      const dayKey = new Date(now).toISOString().slice(0, 10);
      if (dayKey !== lastTodayNoteDayKey && !cancelled) {
        setTodayNote(getTodayNote(now));
        lastTodayNoteDayKey = dayKey;
      }
    };

    const refreshSummaryIfNeeded = async (force = false) => {
      const now = Date.now();
      syncTodayNote(now);

      if (!force && !shouldRefreshSummary(lastSummaryRefreshedAt, now)) {
        return;
      }

      try {
        const summary = await generateBootSummary();
        if (!cancelled) {
          setSummaryBundle(summary);
          lastSummaryRefreshedAt = now;
        }
      } catch (error) {
        console.error("刷新总结和鼓励语失败", error);
      }
    };

    const loadInitialState = async () => {
      try {
        const [stats, summary, settings, currentAutostartEnabled] = await Promise.all([
          getDashboardStats(),
          generateBootSummary(),
          getSettings(),
          isAutostartEnabled(),
        ]);
        let autostartEnabled = currentAutostartEnabled;

        if (cancelled) {
          return;
        }

        if (!autostartEnabled) {
          await enableAutostart();
          autostartEnabled = true;
        }

        setStats(stats);
        setSummaryBundle(summary);
        setTodayNote(getTodayNote(Date.now()));
        lastSummaryRefreshedAt = Date.now();
        lastTodayNoteDayKey = new Date().toISOString().slice(0, 10);
        hydrateSettings(settings);
        setAutostartEnabled(autostartEnabled);
      } catch (error) {
        console.error("初始化应用状态失败", error);
      }
    };

    const refreshStats = async () => {
      if (!shouldRunForegroundRefresh()) {
        return;
      }

      try {
        const stats = await getDashboardStats();
        if (!cancelled) {
          setStats(stats);
        }
        await refreshSummaryIfNeeded(false);
      } catch (error) {
        console.error("刷新监控数据失败", error);
      }
    };

    const stopPolling = () => {
      if (timer !== null) {
        window.clearInterval(timer);
        timer = null;
      }
    };

    const startPolling = () => {
      if (timer !== null || !shouldRunForegroundRefresh()) {
        return;
      }

      timer = window.setInterval(() => {
        void refreshStats();
      }, STATS_REFRESH_MS);
    };

    void loadInitialState();
    startPolling();

    const handleVisibilityWake = () => {
      if (shouldRunForegroundRefresh()) {
        startPolling();
        void refreshStats();
      } else {
        stopPolling();
      }
    };

    window.addEventListener("focus", handleVisibilityWake);
    document.addEventListener("visibilitychange", handleVisibilityWake);

    return () => {
      cancelled = true;
      stopPolling();
      window.removeEventListener("focus", handleVisibilityWake);
      document.removeEventListener("visibilitychange", handleVisibilityWake);
    };
  }, [hydrateSettings, setAutostartEnabled, setStats, setSummaryBundle, setTodayNote]);

  useEffect(() => {
    void updateSettings(useSettingsStore.getState().toDto()).catch((error) => {
      console.error("同步设置失败", error);
    });
  }, []);

  return null;
}

function App() {
  const element = useRoutes(routes);

  return (
    <>
      <AppBootstrap />
      {element}
    </>
  );
}

export default App;
