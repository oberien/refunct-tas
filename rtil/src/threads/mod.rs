mod listener;
mod stream_read;
mod stream_write;
mod lua;
mod ue;

enum ListenerToStream {
    KillYourself,
}

enum StreamToListener {
    ImDead,
}

enum StreamToLua {
    Start(String),
    Stop,
    Config([i32; 7]),
}

enum LuaToStream {
    Print(String),
    ImDone,
}

enum LuaToUe {
    Stop,
    AdvanceFrame,
    Resume,
}

enum UeToLua {
    Tick,
    NewGame,
}
