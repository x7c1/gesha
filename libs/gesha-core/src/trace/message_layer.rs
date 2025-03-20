use chrono::Local;
use nu_ansi_term::{Color, Style};
use std::fmt::Debug;
use std::io::Write;
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

pub struct MessageLayer<W> {
    writer: W,
}

impl<S, W> Layer<S> for MessageLayer<W>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    W: for<'a> MakeWriter<'a> + 'static,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut fields = LogFields::default();
        let mut buffer = self.serialize(event, ctx, &mut fields).unwrap();
        buffer.write_all(b"\n").unwrap();

        self.writer.make_writer().write_all(&buffer).unwrap();
    }
}

impl<W> MessageLayer<W>
where
    W: for<'a> MakeWriter<'a> + 'static,
{
    pub fn new(writer: W) -> MessageLayer<W> {
        MessageLayer { writer }
    }

    fn serialize<S>(
        &self,
        event: &Event<'_>,
        _ctx: Context<'_, S>,
        fields: &mut LogFields,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let mut buffer = vec![];
        let metadata = event.metadata();
        if metadata.fields().field("message").is_none() {
            return Ok(buffer);
        };
        event.record(fields);

        write!(
            buffer,
            "{time} {level} {message}",
            time = {
                /*
                   // notes:
                   // use this if need non-abbreviated format like "2023-02-24T02:58:21.512142+09:00"
                   let now = Local::now().to_rfc3339_opts(SecondsFormat::Micros, true);
                */
                // e.g. 02:57:40.528372
                let now = Local::now().format("%H:%M:%S%.6f").to_string();
                Style::new().fg(Color::Default).dimmed().paint(now)
            },
            level = {
                let level = metadata.level();
                to_style(*level).bold().paint(level.as_str())
            },
            message = &fields.message,
        )?;
        Ok(buffer)
    }
}

#[derive(Default)]
struct LogFields {
    message: String,
}

impl Visit for LogFields {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        let name = field.name();
        if name == "message" {
            self.message = format!("{value:?}");
        }
    }
}

fn to_style(level: Level) -> Style {
    let color = match level {
        Level::TRACE => Color::Purple,
        Level::DEBUG => Color::Blue,
        Level::INFO => Color::Green,
        Level::WARN => Color::Yellow,
        Level::ERROR => Color::Red,
    };
    Style::new().fg(color)
}
