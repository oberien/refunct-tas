fn randomizer_test() {}

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
    difficulty: Difficulty,
    new_game_new_seed: NewGameNewSeed,
    sequence: List<int>,
    seq_index: int,
    kind: RandomizerStateKind,
}
enum RandomizerStateKind {
    Disabled,
    // seed
    RandomSeed(int),
    // seed
    SetSeed(int),
    SetSequence,
}

static mut RANDOMIZER_STATE = RandomizerState {
    difficulty: Difficulty::Beginner,
    new_game_new_seed: NewGameNewSeed::Auto,
    sequence: List::new(),
    seq_index: 0,
    kind: RandomizerStateKind::Disabled,
};
fn randomizer_disable() {
    RANDOMIZER_STATE.kind = RandomizerStateKind::Disabled;
}
fn randomizer_random_seed(difficulty: int, new_game_new_seed: int) {
    let difficulty = convert_difficulty(difficulty);
    let new_game_new_seed = convert_new_game_new_seed(new_game_new_seed);
    let seed = Rng::set_random_seed();
    let sequence = generate_sequence(difficulty);
    RANDOMIZER_STATE = RandomizerState {
        difficulty: difficulty,
        new_game_new_seed: new_game_new_seed,
        sequence: sequence,
        seq_index: 0,
        kind: RandomizerStateKind::RandomSeed(seed),
    };
}
fn randomizer_set_seed(seed: int, difficulty: int, new_game_new_seed: int) {
    let difficulty = convert_difficulty(difficulty);
    let new_game_new_seed = convert_new_game_new_seed(new_game_new_seed);
    Rng::set_seed(seed);
    let sequence = generate_sequence(difficulty);
    RANDOMIZER_STATE = RandomizerState {
        difficulty: difficulty,
        new_game_new_seed: new_game_new_seed,
        sequence: sequence,
        seq_index: 0,
        kind: RandomizerStateKind::SetSeed(seed),
    };
}
fn randomizer_set_sequence(seq: List<int>, difficulty: int, new_game_new_seed: int) {
    let difficulty = convert_difficulty(difficulty);
    let new_game_new_seed = convert_new_game_new_seed(new_game_new_seed);
    RANDOMIZER_STATE = RandomizerState {
        difficulty: difficulty,
        new_game_new_seed: new_game_new_seed,
        sequence: seq,
        seq_index: 0,
        kind: RandomizerStateKind::SetSequence,
    };
}

// runtime functions
fn randomizer_new_game_function() {
    fn new_seed(kind_fn: fn(int) -> RandomizerStateKind) {
        let seed = Rng::set_random_seed();
        RANDOMIZER_STATE = RandomizerState {
            difficulty: RANDOMIZER_STATE.difficulty,
            new_game_new_seed: RANDOMIZER_STATE.new_game_new_seed,
            sequence: generate_sequence(RANDOMIZER_STATE.difficulty),
            seq_index: 0,
            kind: kind_fn(seed),
        };
    }
    fn same_seed(seed: int, kind_fn: fn(int) -> RandomizerStateKind) {
        Rng::set_seed(seed);
        RANDOMIZER_STATE = RandomizerState {
            difficulty: RANDOMIZER_STATE.difficulty,
            new_game_new_seed: RANDOMIZER_STATE.new_game_new_seed,
            sequence: generate_sequence(RANDOMIZER_STATE.difficulty),
            seq_index: 0,
            kind: kind_fn(seed),
        };
    }
    match RANDOMIZER_STATE.kind {
        RandomizerStateKind::Disabled => panic("randomizer_new_game_function called with RandomizerStateKind::Disabled"),
        RandomizerStateKind::RandomSeed(seed) => match RANDOMIZER_STATE.new_game_new_seed {
            NewGameNewSeed::Auto => new_seed(RandomizerStateKind::RandomSeed),
            NewGameNewSeed::On => new_seed(RandomizerStateKind::RandomSeed),
            NewGameNewSeed::Off => same_seed(seed, RandomizerStateKind::RandomSeed),
        },
        RandomizerStateKind::SetSeed(seed) => match RANDOMIZER_STATE.new_game_new_seed {
            NewGameNewSeed::Auto => same_seed(seed, RandomizerStateKind::SetSeed),
            NewGameNewSeed::On => new_seed(RandomizerStateKind::SetSeed),
            NewGameNewSeed::Off => same_seed(seed, RandomizerStateKind::SetSeed),
        },
        RandomizerStateKind::SetSequence => {
            RANDOMIZER_STATE.seq_index = 0;
        },
    }
}
fn next_level() {
    if RANDOMIZER_STATE.seq_index >= RANDOMIZER_STATE.sequence.len() {
        return;
    }
    let new_level = RANDOMIZER_STATE.sequence.get(RANDOMIZER_STATE.seq_index).unwrap();
    Tas::set_level(new_level);
    RANDOMIZER_STATE.seq_index += 1;
}
fn randomizer_on_level_change_function(level: int) {
    if level <= 0 {
        return;
    }
    next_level();
}
fn randomizer_on_reset_function(reset: int) {
    RANDOMIZER_STATE.seq_index = 0;
    next_level();
}

// helper functions
fn convert_difficulty(difficulty: int) -> Difficulty {
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
                None => {
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
         0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28
    );

    let mut workset = List::new();
    for level in levels {
        if dependencies.get(level).is_none() {
            workset.push(level);
        }
    }

    let mut sequence = List::new();
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
    sequence.push(29);
    print(f"sequence: {sequence}");
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
        nums.push(num-2);
    }
    if matches.len() == 0 {
        Result::Err("no sequence found")
    } else if nums.get(0).unwrap() != -1 {
        Result::Err("needs to start with 1")
    } else if nums.last().unwrap() != 29 {
        Result::Err("needs to end with 31")
    } else {
        // remove first platform as it always rises first
        nums.remove(0);
        Result::Ok(nums)
    }
}

