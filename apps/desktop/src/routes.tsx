import type { RouteObject } from "react-router-dom";
import { AppFrame } from "./components/layout/AppFrame";
import { DashboardPage } from "./pages/DashboardPage";

export const routes: RouteObject[] = [
  {
    path: "/",
    element: <AppFrame />,
    children: [
      {
        index: true,
        element: <DashboardPage />,
      },
    ],
  },
];
