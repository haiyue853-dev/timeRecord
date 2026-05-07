export type AppSection = {
  to: string;
  label: string;
  title: string;
  description: string;
  end?: boolean;
};

export const appSections: AppSection[] = [
  {
    to: "/",
    label: "仪表盘",
    title: "本次开机概览",
    description: "从这里快速查看本次开机要恢复的应用、窗口和最近执行状态。",
    end: true,
  },
  {
    to: "/apps",
    label: "软件统计",
    title: "软件统计",
    description: "查看本次开机涉及的软件数量、分类和恢复覆盖情况。",
  },
  {
    to: "/windows",
    label: "窗口明细",
    title: "窗口明细",
    description: "查看待恢复窗口的标题、来源应用和布局信息。",
  },
  {
    to: "/history",
    label: "历史趋势",
    title: "历史趋势",
    description: "按时间维度回看开机恢复结果和使用趋势变化。",
  },
  {
    to: "/settings",
    label: "设置",
    title: "设置",
    description: "配置 TimeRecord 的采集、恢复与偏好选项。",
  },
];
