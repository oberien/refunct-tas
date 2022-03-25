static RANDOMIZER_COMPONENT = Component {
    draw_hud: randomizer_draw_hud,
    tick: fn() {},
    on_new_game: randomizer_new_game_function,
    on_level_change: randomizer_on_level_change_function,
    on_reset: randomizer_on_reset_function,
    on_component_exit: fn() {},
};

enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}
enum NewGameNewSeed {
    /// On for Random Seed, Off for Set Seed
    Auto,
    On,
    Off,
}
struct RandomizerState {
    new_game_new_seed: NewGameNewSeed,
    prev: Option<RandomizerStateKind>,
    current: Option<RandomizerStateKind>,
    queue: List<RandomizerStateKind>,
    seq_index: int,
}
enum RandomizerStateKind {
    // seed, difficulty, sequence
    RandomSeed(int, Difficulty, List<int>),
    // seed, difficulty, sequence
    SetSeed(int, Difficulty, List<int>),
    // sequence
    SetSequence(List<int>),
}
impl RandomizerStateKind {
    fn sequence(self) -> List<int> {
        match self {
            RandomizerStateKind::RandomSeed(seed, diff, seq) => seq,
            RandomizerStateKind::SetSeed(seed, diff, seq) => seq,
            RandomizerStateKind::SetSequence(seq) => seq,
        }
    }
    fn sequence_string(self) -> string {
        let mut seq = "";
        for platform in self.sequence() {
            seq = f"{seq}{platform}, ";
        }
        seq.slice(0, -2)
    }
    fn hud_string(self, with_info: bool) -> string {
        match self {
            RandomizerStateKind::RandomSeed(seed, difficulty, seq) => if with_info {
                f"Random Seed: {seed} ({difficulty}) [{self.sequence_string()}]"
            } else {
                f"Random Seed ({difficulty})"
            },
            RandomizerStateKind::SetSeed(seed, difficulty, seq) => if with_info {
                f"Set Seed: {seed} ({difficulty}) [{self.sequence_string()}]"
            } else {
                f"Set Seed: {seed} ({difficulty})"
            },
            RandomizerStateKind::SetSequence(seq) => {
                f"Set Sequence: [{self.sequence_string()}]"
            },
        }
    }
}

static mut RANDOMIZER_STATE = RandomizerState {
    new_game_new_seed: NewGameNewSeed::Auto,
    prev: Option::None,
    current: Option::None,
    queue: List::new(),
    seq_index: 1,
};
fn randomizer_random_seed(difficulty: Difficulty) {
    let seed = Rng::set_random_seed();
    let sequence = generate_sequence(difficulty);
    RANDOMIZER_STATE.queue.clear();
    RANDOMIZER_STATE.queue.push(RandomizerStateKind::RandomSeed(seed, difficulty, sequence));
    RANDOMIZER_STATE.seq_index = 1;
}
fn randomizer_set_seed(seed: int, difficulty: Difficulty) {
    Rng::set_seed(seed);
    let sequence = generate_sequence(difficulty);
    RANDOMIZER_STATE.queue.clear();
    RANDOMIZER_STATE.queue.push(RandomizerStateKind::SetSeed(seed, difficulty, sequence));
    RANDOMIZER_STATE.seq_index = 1;
}
fn randomizer_set_sequence(seq: List<int>) {
    RANDOMIZER_STATE.queue.clear();
    RANDOMIZER_STATE.queue.push(RandomizerStateKind::SetSequence(seq));
    RANDOMIZER_STATE.seq_index = 1;
}
fn randomizer_copy_prev_seed() {
    let prev = match RANDOMIZER_STATE.prev {
        Option::Some(prev) => prev,
        Option::None => return,
    };
    match prev {
        RandomizerStateKind::RandomSeed(seed, diff, seq) => {
            Tas::set_clipboard(f"{seed}");
        },
        RandomizerStateKind::SetSeed(seed, diff, seq) => {
            Tas::set_clipboard(f"{seed}");
        },
        RandomizerStateKind::SetSequence(seq) => (),
    }
}
fn randomizer_copy_prev_sequence() {
    let prev = match RANDOMIZER_STATE.prev {
        Option::Some(prev) => prev,
        Option::None => return,
    };
    Tas::set_clipboard(prev.sequence_string());
}

// runtime functions
fn randomizer_draw_hud(text: string) -> string {
    let text = match RANDOMIZER_STATE.prev {
        Option::Some(prev) => f"{text}\nPrevious: {prev.hud_string(true)}",
        Option::None => text,
    };
    let text = match RANDOMIZER_STATE.current {
        Option::Some(current) => f"{text}\nCurrent: {current.hud_string(false)}",
        Option::None => f"{text}\nPress \"New Game\" to start",
    };
    let text = match RANDOMIZER_STATE.queue.get(0) {
        Option::Some(next) => f"{text}\nNext: {next.hud_string(false)}",
        Option::None => {
            match RANDOMIZER_STATE.current {
                Option::Some(current) => {
                    let next = match current {
                        RandomizerStateKind::RandomSeed(seed, diff, seq) => match RANDOMIZER_STATE.new_game_new_seed {
                            NewGameNewSeed::Auto => "new seed",
                            NewGameNewSeed::On => "new seed",
                            NewGameNewSeed::Off => "same seed",
                        },
                        RandomizerStateKind::SetSeed(seed, diff, seq) => match RANDOMIZER_STATE.new_game_new_seed {
                            NewGameNewSeed::Auto => "same seed",
                            NewGameNewSeed::On => "new seed",
                            NewGameNewSeed::Off => "same seed",
                        },
                        RandomizerStateKind::SetSequence(seq) => "same sequence",
                    };
                    f"{text}\nNext: {next}"
                },
                Option::None => text,
            }
        },
    };
    match RANDOMIZER_STATE.current {
        Option::Some(current) => {
            let prog = RANDOMIZER_STATE.seq_index-2;
            let prog = prog.max(0);
            f"{text}\nProgress: {prog:02}/{current.sequence().len():02}"
        },
        Option::None => text
    }
}
fn randomizer_new_game_function() {
    RANDOMIZER_STATE.seq_index = 1;
    RANDOMIZER_STATE.prev = RANDOMIZER_STATE.current;
    match RANDOMIZER_STATE.queue.remove(0) {
        Option::Some(next) => {
            RANDOMIZER_STATE.current = Option::Some(next);
            return;
        },
        Option::None => (),
    }

    // nothing in queue, add element to queue, then set current to queue

    let current = match RANDOMIZER_STATE.current {
        Option::Some(current) => current,
        Option::None => return,
    };
    match current {
        RandomizerStateKind::RandomSeed(seed, difficulty, sequence) => match RANDOMIZER_STATE.new_game_new_seed {
            NewGameNewSeed::Auto => randomizer_random_seed(difficulty),
            NewGameNewSeed::On => randomizer_random_seed(difficulty),
            NewGameNewSeed::Off => RANDOMIZER_STATE.queue.push(current),
        },
        RandomizerStateKind::SetSeed(seed, difficulty, sequence) => match RANDOMIZER_STATE.new_game_new_seed {
            NewGameNewSeed::Auto => RANDOMIZER_STATE.queue.push(current),
            NewGameNewSeed::On => randomizer_random_seed(difficulty),
            NewGameNewSeed::Off => RANDOMIZER_STATE.queue.push(current),
        },
        RandomizerStateKind::SetSequence(sequence) => {
            RANDOMIZER_STATE.queue.push(current)
        },
    }
    
    RANDOMIZER_STATE.current = RANDOMIZER_STATE.queue.remove(0);
}
fn next_level() {
    let sequence = match RANDOMIZER_STATE.current {
        Option::Some(current) => current.sequence(),
        Option::None => return,
    };
    if RANDOMIZER_STATE.seq_index < sequence.len() {
        let new_level = sequence.get(RANDOMIZER_STATE.seq_index).unwrap();
        Tas::set_level(new_level - 2);
    }
    RANDOMIZER_STATE.seq_index += 1;
}
fn randomizer_on_level_change_function(level: int) {
    if level <= 0 {
        return;
    }
    next_level();
}
fn randomizer_on_reset_function(reset: int) {
    RANDOMIZER_STATE.seq_index = 1;
    next_level();
}

// helper functions
fn randomizer_convert_difficulty(difficulty: int) -> Difficulty {
    match difficulty {
        0 => Difficulty::Beginner,
        1 => Difficulty::Intermediate,
        2 => Difficulty::Advanced,
        _ => panic(f"unknown difficulty {difficulty}"),
    }
}
fn convert_new_game_new_seed(new_game_new_seed: int) -> NewGameNewSeed {
    match new_game_new_seed {
        0 => NewGameNewSeed::Auto,
        1 => NewGameNewSeed::On,
        2 => NewGameNewSeed::Off,
        _ => panic(f"unknown new_game_new_seed {new_game_new_seed}"),
    }
}
fn generate_sequence(difficulty: Difficulty) -> List<int> {
    let mut dependencies = Map::new();
    match difficulty {
        Difficulty::Beginner => {
            dependencies.insert(13, List::of(3, 11, 14, 15, 23, 24, 27));
            dependencies.insert(16, List::of(2, 17, 28));
            dependencies.insert(18, List::of(8));
            dependencies.insert(22, List::of(3, 11, 12, 20, 30));
        },
        Difficulty::Intermediate => {
            dependencies.insert(13, List::of(3, 11, 14, 15, 23, 24, 27));
            dependencies.insert(18, List::of(8));
            dependencies.insert(22, List::of(3, 11, 12, 20, 30));
        },
        Difficulty::Advanced => {
            dependencies.insert(13, List::of(2, 3, 10, 11, 14, 15, 23, 24, 27));
            dependencies.insert(22, List::of(3, 10, 11, 12, 20, 30));
        },
    }
    let mut dependants = Map::new();
    for level in dependencies.keys() {
        for requirement in dependencies.get(level).unwrap() {
            let mut list = match dependants.get(requirement) {
                Option::Some(list) => list,
                Option::None => {
                    let list = List::new();
                    dependants.insert(requirement, list);
                    list
                }
            };
            list.push(level);
        }
    }

    // TODO: use ranges once they are a thing
    let mut levels = List::of(
                 2,  3,  4,  5,  6,  7,  8,  9,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30,
    );

    let mut workset = List::new();
    for level in levels {
        if dependencies.get(level).is_none() {
            workset.push(level);
        }
    }

    let mut sequence = List::of(1);
    while !workset.is_empty() {
        let random_index = Rng::gen_int_range(0, workset.len());
        let next_level = workset.remove(random_index).unwrap();
        sequence.push(next_level);
        if dependants.get(next_level).is_some() {
            for dependant in dependants.get(next_level).unwrap() {
                if !workset.contains(dependant) && !sequence.contains(dependant) {
                    workset.push(dependant);
                }
            }
        }
    }
    sequence.push(31);
    sequence
}

fn randomizer_parse_seed(seed: string) -> Result<int, string> {
    match seed.parse_int() {
        Result::Ok(seed) => Result::Ok(seed),
        Result::Err(err) => match err {
            ParseIntError::Empty => Result::Err("empty seed"),
            ParseIntError::TooLarge => Result::Err("seed too large"),
            ParseIntError::TooSmall => Result::Err("seed too small"),
            ParseIntError::InvalidDigit => Result::Err("seed is not a number"),
        }
    }
}

fn randomizer_parse_sequence(seq: string) -> Result<List<int>, string> {
    let matches = seq.find_matches("\\d+");
    let mut nums = List::new();
    for m in matches {
        let num = m.parse_int().unwrap();
        if num < 1 || num > 31 {
            return Result::Err(f"invalid platform {num}");
        }
        if nums.contains(num) {
            return Result::Err(f"duplicate platform {num}");
        }
        nums.push(num);
    }
    if matches.len() == 0 {
        Result::Err("no sequence found")
    } else if nums.get(0).unwrap() != 1 {
        Result::Err("needs to start with 1")
    } else if nums.last().unwrap() != 31 {
        Result::Err("needs to end with 31")
    } else {
        Result::Ok(nums)
    }
}

