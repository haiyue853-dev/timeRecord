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
    label: "概览",
    title: "今日概览",
    description: "查看今日累计时长、今日热门应用、分布图和一句简短总结。",
    end: true,
  },
  {
    to: "/apps",
    label: "应用统计",
    title: "应用统计",
    description: "按应用维度查看今日前台活跃时长排行，快速看清时间流向。",
  },
  {
    to: "/history",
    label: "会话走势",
    title: "会话走势",
    description: "回看本次开机后的活跃节奏变化，以及最近一周的学习状态。",
  },
  {
    to: "/settings",
    label: "偏好设置",
    title: "偏好设置",
    description: "配置空闲判定、AI 总结、DeepSeek 接口和开机自启等行为。",
  },
];
