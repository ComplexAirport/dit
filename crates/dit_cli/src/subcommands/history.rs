use crate::subcommands::HandleSubcommand;
use crate::error::CliResult;
use chrono::{DateTime, Local, TimeZone, Utc};
use comfy_table::{presets::UTF8_FULL, Table, ContentArrangement};
use clap::Args;

#[derive(Args)]
pub struct HistorySubcommand {
    #[arg(
        short, long,
        default_value = "5",
        help = "Number of history entries to show. -1 for all entries.")]
    count: isize,
}


impl HandleSubcommand for HistorySubcommand {
    fn handle(&self) -> CliResult<()> {
        let mut dit = Self::require_dit()?;

        let branch_name = dit.branch();
        let commits = dit.history(self.count)?;
        let title = if let Some(branch_name) = branch_name {
            format!("History for branch '{branch_name}'")
        } else {
            String::from("History (detached HEAD)")
        };

        let mut table = Table::new();

        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(["No", "Time", "Message", "Author",  "Hash"]);

        for (idx, commit) in commits.into_iter().enumerate() {
            let timestamp = format_timestamp_local(commit.timestamp);
            table.add_row([
                (idx + 1).to_string(),
                timestamp,
                commit.message,
                commit.author,
                commit.hash
            ]);
        }

        println!("{title}");
        println!("{table}");

        Ok(())
    }
}

fn format_timestamp_local(epoch: u64) -> String {
    Local.timestamp_opt(epoch as i64, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| {
            // default to unix epoch in case something goes wrong
            DateTime::<Utc>::from_timestamp(0, 0).unwrap().to_string()
        })
}
