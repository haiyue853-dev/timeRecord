import { create } from "zustand";
import type { AppSettingsDto } from "../lib/tauri";

type SettingsState = {
  idleSeconds: number;
  aiEnabled: boolean;
  deepseekBaseUrl: string;
  deepseekApiKey: string;
  deepseekModel: string;
  autostartEnabled: boolean;
  hydrate: (settings: AppSettingsDto) => void;
  setIdleSeconds: (value: number) => void;
  setAiEnabled: (value: boolean) => void;
  setDeepseekApiKey: (value: string) => void;
  setDeepseekModel: (value: string) => void;
  setAutostartEnabled: (value: boolean) => void;
  toDto: () => AppSettingsDto;
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  idleSeconds: 180,
  aiEnabled: false,
  deepseekBaseUrl: "https://api.deepseek.com",
  deepseekApiKey: "",
  deepseekModel: "deepseek-chat",
  autostartEnabled: false,
  hydrate: (settings) =>
    set({
      idleSeconds: settings.idle_seconds,
      aiEnabled: settings.ai_enabled,
      deepseekBaseUrl: settings.deepseek_base_url,
      deepseekApiKey: settings.deepseek_api_key,
      deepseekModel: settings.deepseek_model,
    }),
  setIdleSeconds: (value) => set({ idleSeconds: value }),
  setAiEnabled: (value) => set({ aiEnabled: value }),
  setDeepseekApiKey: (value) => set({ deepseekApiKey: value }),
  setDeepseekModel: (value) => set({ deepseekModel: value }),
  setAutostartEnabled: (value) => set({ autostartEnabled: value }),
  toDto: () => ({
    idle_seconds: get().idleSeconds,
    ai_enabled: get().aiEnabled,
    deepseek_base_url: get().deepseekBaseUrl,
    deepseek_api_key: get().deepseekApiKey,
    deepseek_model: get().deepseekModel,
  }),
}));
