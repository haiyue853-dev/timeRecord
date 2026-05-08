import { disable, enable } from "@tauri-apps/plugin-autostart";
import { generateBootSummary, updateSettings } from "../lib/tauri";
import { useSettingsStore } from "../store/useSettingsStore";
import { useStatsStore } from "../store/useStatsStore";

export function SettingsPage() {
  const idleSeconds = useSettingsStore((state) => state.idleSeconds);
  const aiEnabled = useSettingsStore((state) => state.aiEnabled);
  const deepseekApiKey = useSettingsStore((state) => state.deepseekApiKey);
  const deepseekModel = useSettingsStore((state) => state.deepseekModel);
  const autostartEnabled = useSettingsStore((state) => state.autostartEnabled);
  const setIdleSeconds = useSettingsStore((state) => state.setIdleSeconds);
  const setAiEnabled = useSettingsStore((state) => state.setAiEnabled);
  const setDeepseekApiKey = useSettingsStore((state) => state.setDeepseekApiKey);
  const setDeepseekModel = useSettingsStore((state) => state.setDeepseekModel);
  const setAutostartEnabled = useSettingsStore((state) => state.setAutostartEnabled);
  const toDto = useSettingsStore((state) => state.toDto);
  const setSummaryBundle = useStatsStore((state) => state.setSummaryBundle);

  const syncSettings = async () => {
    try {
      await updateSettings(toDto());
      const summary = await generateBootSummary();
      setSummaryBundle(summary);
    } catch (error) {
      console.error("保存设置失败", error);
    }
  };

  const setAutostartState = async (enabled: boolean) => {
    if (enabled) {
      await enable();
    } else {
      await disable();
    }
  };

  return (
    <section className="dashboard-page settings-page">
      <section className="page-hero">
        <span className="page-hero__eyebrow">PREFERENCES</span>
        <h2>偏好设置</h2>
        <p>这里可以控制空闲判定、开机自启和昨日 AI 总结。一般情况下你只需要填入 DeepSeek API Key。</p>
      </section>

      <div className="settings-grid">
        <div className="settings-field settings-field--feature">
          <h3>空闲判定阈值</h3>
          <p>当前设置为 {Math.floor(idleSeconds / 60)} 分钟。超过这个时长没有输入活动时，将不计入活跃时长。</p>
          <input
            aria-label="空闲判定阈值"
            type="range"
            min={60}
            max={600}
            step={60}
            value={idleSeconds}
            onChange={(event) => {
              setIdleSeconds(Number(event.target.value));
              void syncSettings();
            }}
          />
        </div>

        <label className="settings-toggle settings-toggle--feature">
          <div>
            <strong>开机自启</strong>
            <p>开启后应用会在 Windows 登录后自动启动，这样才能从本次开机一开始就持续记录前台时长。</p>
          </div>
          <input
            aria-label="开机自启"
            type="checkbox"
            checked={autostartEnabled}
            onChange={(event) => {
              const checked = event.target.checked;
              setAutostartEnabled(checked);
              void setAutostartState(checked).catch((error) => {
                console.error("切换开机自启失败", error);
                setAutostartEnabled(!checked);
              });
            }}
          />
        </label>

        <label className="settings-toggle settings-toggle--feature">
          <div>
            <strong>启用 AI 总结</strong>
            <p>开启后会优先调用 DeepSeek 按昨天的数据输出小结和鼓励语；如果请求失败，会自动回退本地模板。</p>
          </div>
          <input
            aria-label="启用 AI 总结"
            type="checkbox"
            checked={aiEnabled}
            onChange={(event) => {
              setAiEnabled(event.target.checked);
              void syncSettings();
            }}
          />
        </label>

        <label className="settings-field settings-field--feature">
          <span>DeepSeek API Key</span>
          <input
            aria-label="DeepSeek API Key"
            type="text"
            value={deepseekApiKey}
            placeholder="只需要填 API Key"
            onChange={(event) => {
              setDeepseekApiKey(event.target.value);
            }}
            onBlur={() => {
              void syncSettings();
            }}
          />
        </label>

        <label className="settings-field settings-field--feature">
          <span>DeepSeek 模型</span>
          <input
            aria-label="DeepSeek 模型"
            type="text"
            value={deepseekModel}
            onChange={(event) => {
              setDeepseekModel(event.target.value);
            }}
            onBlur={() => {
              void syncSettings();
            }}
          />
        </label>
      </div>
    </section>
  );
}
