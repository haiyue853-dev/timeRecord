import { NavLink, Outlet } from "react-router-dom";

const navItems = [
  { to: "/", label: "仪表盘", end: true },
  { to: "/apps", label: "应用" },
  { to: "/windows", label: "窗口" },
  { to: "/history", label: "历史" },
  { to: "/settings", label: "设置" },
];

export function AppFrame() {
  return (
    <div className="app-shell">
      <header className="app-shell__header">
        <div>
          <p className="app-shell__eyebrow">Desktop</p>
          <h1 className="app-shell__title">TimeRecord</h1>
        </div>
        <nav aria-label="主导航" className="app-shell__nav">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              end={item.end}
              className={({ isActive }) =>
                isActive ? "app-shell__nav-link is-active" : "app-shell__nav-link"
              }
            >
              {item.label}
            </NavLink>
          ))}
        </nav>
      </header>

      <main className="app-shell__content">
        <Outlet />
      </main>
    </div>
  );
}
