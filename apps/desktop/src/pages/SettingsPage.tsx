import { useSettingsStore } from "../store/useSettingsStore";

export function SettingsPage() {
  const idleSeconds = useSettingsStore((state) => state.idleSeconds);
  const aiEnabled = useSettingsStore((state) => state.aiEnabled);
  const deepseekModel = useSettingsStore((state) => state.deepseekModel);
  const setIdleSeconds = useSettingsStore((state) => state.setIdleSeconds);
  const setAiEnabled = useSettingsStore((state) => state.setAiEnabled);
  const setDeepseekModel = useSettingsStore((state) => state.setDeepseekModel);

  return (
    <section className="dashboard-page settings-page">
      <h2>设置</h2>
      <div className="settings-grid">
        <div className="settings-field">
          <h3>空闲判定</h3>
          <p>当前阈值：{Math.floor(idleSeconds / 60)} 分钟</p>
          <input
            aria-label="空闲分钟数"
            type="range"
            min={60}
            max={600}
            step={60}
            value={idleSeconds}
            onChange={(event) => setIdleSeconds(Number(event.target.value))}
          />
        </div>
        <label className="settings-toggle">
          启用 AI 总结
          <input
            aria-label="启用 AI 总结"
            type="checkbox"
            checked={aiEnabled}
            onChange={(event) => setAiEnabled(event.target.checked)}
          />
        </label>
        <label className="settings-field">
          <span>DeepSeek 模型</span>
          <input
            aria-label="DeepSeek 模型"
            type="text"
            value={deepseekModel}
            onChange={(event) => setDeepseekModel(event.target.value)}
          />
        </label>
      </div>
    </section>
  );
}
