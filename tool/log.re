static mut LOG = Log {
    messages: List::new(),
};

struct Log {
    messages: List<LogMessage>,
}

struct LogMessage {
    content: string,
    added_timestamp: int,
}

fn log(content: string) {
    let message = LogMessage {
        content: content,
        added_timestamp: current_time_millis(),
    };
    LOG.messages.push(message);
}
fn clean_expired_messages() {
    let millis = current_time_millis();
    let mut i = 0;
    while i < LOG.messages.len() {
        if (millis - LOG.messages.get(i).unwrap().added_timestamp) > SETTINGS.log_message_duration {
            LOG.messages.remove(i);
        } else {
            i += 1;
        }
    }
}

fn draw_log_messages() {
    let viewport = Tas::get_viewport_size();
    let mut messages = "";
    for message in LOG.messages {
        messages = f"{message.content}\n{messages}";
    }
    let text_size = Tas::get_text_size(messages, SETTINGS.ui_scale);
    Tas::draw_text(DrawText {
        text: messages,
        color: COLOR_RED,
        x: 10.,
        y: viewport.height.to_float() - (text_size.height * LOG.messages.len().to_float()),
        scale: SETTINGS.ui_scale,
        scale_position: false,
    });
    clean_expired_messages();
}
