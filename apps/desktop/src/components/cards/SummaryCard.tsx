export function SummaryCard({
  summary,
  encouragement,
  source,
}: {
  summary: string;
  encouragement: string;
  source: string;
}) {
  return (
    <section className="summary-card">
      <div className="summary-card__section">
        <p className="summary-card__eyebrow">昨日小结</p>
        <p className="summary-card__body">{summary}</p>
      </div>
      <div className="summary-card__section">
        <p className="summary-card__eyebrow">鼓励语</p>
        <p className="summary-card__body">{encouragement}</p>
      </div>
      <p className="summary-card__source">来源：{source === "deepseek" ? "DeepSeek" : "本地模拟"}</p>
    </section>
  );
}
