use super::{context::SummaryContext, SummaryBundle};

pub struct LocalSummaryProvider {
    seed: u64,
}

impl LocalSummaryProvider {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn generate(&self, context: &SummaryContext) -> SummaryBundle {
        SummaryBundle {
            summary: self.generate_summary(context),
            encouragement: self.generate_encouragement(context),
            source: "local".into(),
        }
    }

    fn generate_summary(&self, context: &SummaryContext) -> String {
        if context.yesterday_total_seconds <= 0 {
            return pick_line(
                self.seed,
                &[
                    "昨天还没有记录到有效前台时长，今天先别急着追求完美，先让第一段专注发生。",
                    "昨天暂时没有形成可分析的数据，不过没关系，今天开始动起来，这张时间账本就会慢慢清晰。",
                ],
            )
            .to_string();
        }

        let yesterday_minutes = context.yesterday_total_seconds / 60;
        let learning_minutes = context.yesterday_learning_seconds / 60;
        let development_minutes = context.yesterday_development_seconds / 60;
        let delta_minutes = (context.yesterday_total_seconds - context.previous_day_total_seconds) / 60;
        let trend_line = match delta_minutes.cmp(&0) {
            std::cmp::Ordering::Greater => format!("比前天多了 {} 分钟", delta_minutes),
            std::cmp::Ordering::Less => format!("比前天少了 {} 分钟", delta_minutes.abs()),
            std::cmp::Ordering::Equal => "和前天基本持平".to_string(),
        };

        let focus_line = if learning_minutes > development_minutes {
            format!("节奏更偏向学习吸收，学习时长大约 {} 分钟", learning_minutes)
        } else if development_minutes > 0 {
            format!("节奏更偏向输出推进，开发相关时长大约 {} 分钟", development_minutes)
        } else {
            "整体更像是在维持日常使用节奏".to_string()
        };

        format!(
            "昨天你累计活跃了 {yesterday_minutes} 分钟，{trend_line}。{focus_line}。最近一周平均每天约 {} 分钟。",
            context.current_week_average_seconds / 60
        )
    }

    fn generate_encouragement(&self, context: &SummaryContext) -> String {
        let lines = if context.yesterday_total_seconds <= 0 {
            &[
                "今天先拿下一个 25 分钟的小目标，状态会自己跟上来。",
                "不用等到很有感觉再开始，开始本身就会把感觉带回来。",
                "你只要先坐进节奏里，后面的专注会慢慢接管一切。",
            ][..]
        } else if context.yesterday_learning_seconds >= context.yesterday_development_seconds {
            &[
                "昨天吸收了不少内容，今天挑一小块立刻复盘或实践，效果会更扎实。",
                "学习不是堆时长，今天把昨天最关键的一点真正用出来，就已经很强了。",
                "你昨天已经在往前走了，今天只要把那股势头续上就很好。",
            ][..]
        } else {
            &[
                "昨天已经有推进了，今天继续把最重要的一件事再往前拱一点。",
                "别低估连续推进的力量，你现在是在给结果一点点铺地基。",
                "保持这种能落地的节奏，哪怕只是再完成一个小闭环，也很值。",
            ][..]
        };

        pick_line(self.seed + context.current_week_total_seconds as u64, lines).to_string()
    }
}

fn pick_line<'a>(seed: u64, lines: &'a [&'a str]) -> &'a str {
    let index = (seed as usize) % lines.len();
    lines[index]
}
