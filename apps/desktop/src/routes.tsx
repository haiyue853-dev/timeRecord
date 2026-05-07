import type { ReactNode } from "react";
import type { RouteObject } from "react-router-dom";
import { AppFrame } from "./components/layout/AppFrame";
import { DashboardPage } from "./pages/DashboardPage";

function PlaceholderPage({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <section className="dashboard-page">
      <h2>{title}</h2>
      <p>{description}</p>
    </section>
  );
}

function page(title: string, description: string): ReactNode {
  return <PlaceholderPage title={title} description={description} />;
}

export const routes: RouteObject[] = [
  {
    path: "/",
    element: <AppFrame />,
    children: [
      {
        index: true,
        element: <DashboardPage />,
      },
      {
        path: "apps",
        element: page("软件统计", "查看本次开机涉及的软件数量、分类和恢复覆盖情况。"),
      },
      {
        path: "windows",
        element: page("窗口明细", "查看待恢复窗口的标题、来源应用和布局信息。"),
      },
      {
        path: "history",
        element: page("历史趋势", "按时间维度回看开机恢复结果和使用趋势变化。"),
      },
      {
        path: "settings",
        element: page("设置", "配置 TimeRecord 的采集、恢复与偏好选项。"),
      },
    ],
  },
];
