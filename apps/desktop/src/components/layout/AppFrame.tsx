import { NavLink, Outlet } from "react-router-dom";
import { appSections } from "../../navigation";
import { TitleBar } from "./TitleBar";

export function AppFrame() {
  return (
    <div className="window-shell">
      <TitleBar />
      <div className="window-shell__surface">
        <aside className="sidebar">
          <div className="sidebar__panel">
            <div className="sidebar__hero">
              <p className="sidebar__eyebrow">Desktop Console</p>
              <h1 className="sidebar__title">TimeRecord</h1>
              <p className="sidebar__subtitle">本次开机的前台活动、窗口明细和趋势一站式查看。</p>
            </div>
            <nav aria-label="主导航" className="sidebar__nav">
              {appSections.map((section) => (
                <NavLink
                  key={section.to}
                  to={section.to}
                  end={section.end}
                  className={({ isActive }) =>
                    isActive ? "sidebar__link is-active" : "sidebar__link"
                  }
                >
                  <span className="sidebar__link-title">{section.label}</span>
                  <small className="sidebar__link-copy">{section.description}</small>
                </NavLink>
              ))}
            </nav>
          </div>
        </aside>
        <main className="content-shell">
          <section className="content-shell__panel">
            <Outlet />
          </section>
        </main>
      </div>
    </div>
  );
}
