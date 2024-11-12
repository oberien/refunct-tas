static mut LOG = Log {
    messages: List::new(),
    message_expire_delay: 5000,
};

struct Log {
    messages: List<LogMessage>,
    message_expire_delay: int,
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
    let mut i = 0;
    while i < LOG.messages.len() {
        if (current_time_millis() - LOG.messages.get(i).unwrap().added_timestamp) > LOG.message_expire_delay {
            LOG.messages.remove(i);
        } else {
            i += 1;
        }
    }
}

fn draw_log_messages() {
    let mut i = 0;
    while i < LOG.messages.len() {
        let viewport = Tas::get_viewport_size();
        let message = LOG.messages.get(i).unwrap();
        Tas::draw_text(DrawText {
            text: message.content,
            color: Color { red: 1., green: 0., blue: 0., alpha: 1.},
            x: 10.,
            // 51 denotes the amount of vertical space from the bottom of the screen.
            // 48 denotes the amount of vertical space between each log message.
            y: viewport.height.to_float() - ((51. * SETTINGS.ui_scale) + ((48. * SETTINGS.ui_scale) * i.to_float())),
            scale: SETTINGS.ui_scale,
            scale_position: false,
        });
        i += 1;
    }:
    clean_expired_messages();
}
