import type { RouteObject } from "react-router-dom";
import { AppFrame } from "./components/layout/AppFrame";
import { AppsPage } from "./pages/AppsPage";
import { DashboardPage } from "./pages/DashboardPage";
import { HistoryPage } from "./pages/HistoryPage";
import { PlaceholderPage } from "./pages/PlaceholderPage";
import { SettingsPage } from "./pages/SettingsPage";
import { WindowsPage } from "./pages/WindowsPage";

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
        path: "windows",
        element: <WindowsPage />,
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
        element: <PlaceholderPage title="未找到页面" description="这个页面还没有接入。" />,
      },
    ],
  },
];
