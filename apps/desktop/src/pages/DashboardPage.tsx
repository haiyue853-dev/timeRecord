import { appSections } from "../navigation";

const dashboardSection = appSections[0];

export function DashboardPage() {
  return (
    <section className="dashboard-page">
      <h2>{dashboardSection.title}</h2>
      <p>{dashboardSection.description}</p>
    </section>
  );
}
