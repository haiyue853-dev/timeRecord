import { create } from "zustand";
import type { SummaryBundleDto } from "../lib/tauri";
import type { DashboardStats } from "../types/stats";

const defaultStats: DashboardStats = {
  totalActiveSeconds: 0,
  todayActiveSeconds: 0,
  currentAppName: "等待活动采集",
  currentWindowTitle: "等待检测到前台窗口",
  apps: [
    {
      appName: "等待活动采集",
      seconds: 0,
      category: "system",
    },
  ],
  todayApps: [
    {
      appName: "等待活动采集",
      seconds: 0,
      category: "system",
    },
  ],
  summary: "昨天的数据还在积累中，先把今天最重要的一段专注稳稳放到前台。",
  encouragement: "先专注 25 分钟，状态通常会慢慢回来。",
  summarySource: "local",
  trendPoints: [],
  weeklySummary: {
    currentWeekTotalSeconds: 0,
    previousWeekTotalSeconds: 0,
    currentWeekLearningSeconds: 0,
    currentWeekAverageSeconds: 0,
    deltaSeconds: 0,
    bestDay: {
      date: new Date().toISOString().slice(0, 10),
      totalActiveSeconds: 0,
      learningSeconds: 0,
      developmentSeconds: 0,
    },
  },
  learningHeatmap: [],
};

type StatsState = DashboardStats & {
  todayNote: string;
  setStats: (stats: DashboardStats) => void;
  setSummaryBundle: (bundle: SummaryBundleDto) => void;
  setTodayNote: (todayNote: string) => void;
};

export const useStatsStore = create<StatsState>((set) => ({
  ...defaultStats,
  todayNote: "先把今天最重要的事情放到前台，节奏就会慢慢稳下来。",
  setStats: (stats) =>
    set((state) => ({
      ...stats,
      summary: state.summary,
      encouragement: state.encouragement,
      summarySource: state.summarySource,
      todayNote: state.todayNote,
    })),
  setSummaryBundle: (bundle) =>
    set({
      summary: bundle.summary,
      encouragement: bundle.encouragement,
      summarySource: bundle.source,
    }),
  setTodayNote: (todayNote) => set({ todayNote }),
}));
