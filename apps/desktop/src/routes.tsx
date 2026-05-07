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
        element: page("应用", "集中管理需要在开机阶段恢复的应用。"),
      },
      {
        path: "windows",
        element: page("窗口", "记录并恢复每次开机时需要打开的窗口布局。"),
      },
      {
        path: "history",
        element: page("历史", "查看历史开机记录和恢复执行结果。"),
      },
      {
        path: "settings",
        element: page("设置", "配置 TimeRecord 的采集、恢复与偏好选项。"),
      },
    ],
  },
];
