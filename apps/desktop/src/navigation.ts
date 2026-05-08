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
    title: "本次开机总览",
    description: "查看当前前台状态、累计活跃时长、分布图和一句总结。",
    end: true,
  },
  {
    to: "/apps",
    label: "应用统计",
    title: "应用统计",
    description: "按软件维度查看本次开机期间的前台活跃时长排行。",
  },
  {
    to: "/windows",
    label: "窗口焦点",
    title: "窗口焦点",
    description: "观察当前正在记录的窗口标题，方便区分具体任务内容。",
  },
  {
    to: "/history",
    label: "会话走势",
    title: "会话走势",
    description: "用趋势视图回看本次开机内的活跃节奏变化。",
  },
  {
    to: "/settings",
    label: "偏好设置",
    title: "偏好设置",
    description: "配置空闲判定、AI 总结开关，以及 DeepSeek 模型名称。",
  },
];
