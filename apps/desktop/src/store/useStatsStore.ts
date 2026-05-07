import { create } from "zustand";
import type { DashboardStats } from "../types/stats";

const defaultStats: DashboardStats = {
  totalActiveSeconds: 0,
  currentAppName: "TimeRecord",
  currentWindowTitle: "等待活动采集",
  apps: [
    {
      appName: "TimeRecord",
      seconds: 0,
      category: "system",
    },
  ],
  summary: "这次开机还没积累到足够多的使用数据。",
};

type StatsState = DashboardStats & {
  setStats: (stats: DashboardStats) => void;
};

export const useStatsStore = create<StatsState>((set) => ({
  ...defaultStats,
  setStats: (stats) => set(stats),
}));
