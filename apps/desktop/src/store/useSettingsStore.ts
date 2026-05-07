import { create } from "zustand";

type SettingsState = {
  idleSeconds: number;
  aiEnabled: boolean;
  deepseekModel: string;
  setIdleSeconds: (value: number) => void;
  setAiEnabled: (value: boolean) => void;
  setDeepseekModel: (value: string) => void;
};

export const useSettingsStore = create<SettingsState>((set) => ({
  idleSeconds: 180,
  aiEnabled: false,
  deepseekModel: "deepseek-chat",
  setIdleSeconds: (value) => set({ idleSeconds: value }),
  setAiEnabled: (value) => set({ aiEnabled: value }),
  setDeepseekModel: (value) => set({ deepseekModel: value }),
}));
