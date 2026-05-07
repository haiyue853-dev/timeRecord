import { invoke } from "@tauri-apps/api/core";
import type { DashboardStats } from "../types/stats";

export interface AppSettingsDto {
  idle_seconds: number;
  ai_enabled: boolean;
  deepseek_base_url: string;
  deepseek_model: string;
}

export function getSettings() {
  return invoke<AppSettingsDto>("get_settings");
}

export function getDashboardStats() {
  return invoke<DashboardStats>("get_dashboard_stats");
}

export function generateBootSummary() {
  return invoke<string>("generate_boot_summary");
}
