export interface AppUsageItem {
  appName: string;
  seconds: number;
  category: string;
}

export interface TrendPoint {
  label: string;
  activeSeconds: number;
}

export interface DailyHistoryItem {
  date: string;
  totalActiveSeconds: number;
  learningSeconds: number;
  developmentSeconds: number;
}

export interface WeeklySummary {
  currentWeekTotalSeconds: number;
  previousWeekTotalSeconds: number;
  currentWeekLearningSeconds: number;
  currentWeekAverageSeconds: number;
  deltaSeconds: number;
  bestDay: DailyHistoryItem;
}

export interface HeatmapCell {
  date: string;
  learningSeconds: number;
  level: number;
}

export interface DashboardStats {
  totalActiveSeconds: number;
  currentAppName: string;
  currentWindowTitle: string;
  apps: AppUsageItem[];
  summary: string;
  encouragement: string;
  summarySource: string;
  trendPoints: TrendPoint[];
  weeklySummary: WeeklySummary;
  learningHeatmap: HeatmapCell[];
}
