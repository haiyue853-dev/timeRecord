export interface AppUsageItem {
  appName: string;
  seconds: number;
  category: string;
}

export interface DashboardStats {
  totalActiveSeconds: number;
  currentAppName: string;
  currentWindowTitle: string;
  apps: AppUsageItem[];
  summary: string;
}
