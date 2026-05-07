import { NavLink, Outlet } from "react-router-dom";
import { appSections } from "../../navigation";

export function AppFrame() {
  return (
    <div className="app-shell">
      <header className="app-shell__header">
        <div>
          <p className="app-shell__eyebrow">Desktop</p>
          <h1 className="app-shell__title">TimeRecord</h1>
        </div>
        <nav aria-label="主导航" className="app-shell__nav">
          {appSections.map((section) => (
            <NavLink
              key={section.to}
              to={section.to}
              end={section.end}
              className={({ isActive }) =>
                isActive ? "app-shell__nav-link is-active" : "app-shell__nav-link"
              }
            >
              {section.label}
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
