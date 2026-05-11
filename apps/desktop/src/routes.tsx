import type { RouteObject } from "react-router-dom";
import { AppFrame } from "./components/layout/AppFrame";
import { AppsPage } from "./pages/AppsPage";
import { DashboardPage } from "./pages/DashboardPage";
import { HistoryPage } from "./pages/HistoryPage";
import { PlaceholderPage } from "./pages/PlaceholderPage";
import { SettingsPage } from "./pages/SettingsPage";

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
        element: <AppsPage />,
      },
      {
        path: "history",
        element: <HistoryPage />,
      },
      {
        path: "settings",
        element: <SettingsPage />,
      },
      {
        path: "*",
        element: <PlaceholderPage title="页面未找到" description="这个页面暂时不存在，请从左侧导航返回。" />,
      },
    ],
  },
];
