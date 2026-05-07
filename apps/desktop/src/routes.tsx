import type { RouteObject } from "react-router-dom";
import { appSections } from "./navigation";
import { AppFrame } from "./components/layout/AppFrame";
import { DashboardPage } from "./pages/DashboardPage";
import { PlaceholderPage } from "./pages/PlaceholderPage";

export const routes: RouteObject[] = [
  {
    path: "/",
    element: <AppFrame />,
    children: appSections.map((section) =>
      section.to === "/"
        ? {
            index: true,
            element: <DashboardPage />,
          }
        : {
            path: section.to.slice(1),
            element: (
              <PlaceholderPage
                title={section.title}
                description={section.description}
              />
            ),
          },
    ),
  },
];
