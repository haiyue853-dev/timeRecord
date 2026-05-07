type PlaceholderPageProps = {
  title: string;
  description: string;
};

export function PlaceholderPage({
  title,
  description,
}: PlaceholderPageProps) {
  return (
    <section className="dashboard-page">
      <h2>{title}</h2>
      <p>{description}</p>
    </section>
  );
}
