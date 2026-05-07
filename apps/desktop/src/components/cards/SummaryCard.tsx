export function SummaryCard({ summary }: { summary: string }) {
  return (
    <section className="summary-card">
      <p className="summary-card__eyebrow">今日小结</p>
      <p className="summary-card__body">{summary}</p>
    </section>
  );
}
