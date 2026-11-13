const ANIMAL_NAMES: &[&str] = &[
    "ant", "eel", "mole", "sloth", "ape", "emu", "monkey", "snail", "bat", "falcon", "mouse",
    "snake", "bear", "fish", "otter", "spider", "bee", "fly", "parrot", "squid", "bird", "fox",
    "panda", "swan", "bison", "frog", "pig", "tiger", "camel", "gecko", "pigeon", "toad",
    "cat", "goat", "pony", "turkey", "cobra", "goose", "pug", "turtle", "crow", "hawk",
    "rabbit", "viper", "deer", "horse", "rat", "wasp", "dog", "jaguar", "raven", "whale",
    "dove", "koala", "seal", "wolf", "duck", "lion", "shark", "worm", "eagle", "lizard",
    "sheep", "zebra", "aardvark", "alpaca", "anteater", "antelope", "armadillo", "baboon",
    "badger", "beaver", "bobcat", "buffalo", "bull", "boar", "capybara", "caribou", "cheetah",
    "chimp", "chinchilla", "chipmunk", "cougar", "coyote", "dingo", "dolphin", "donkey",
    "elephant", "elk", "ferret", "gazelle", "gerbil", "gibbon", "giraffe", "gnu", "gopher",
    "gorilla", "groundhog", "guanaco", "hamster", "hare", "hedgehog", "hippo", "hyena", "ibex",
    "impala", "jackal", "jerboa", "kangaroo", "lemming", "lemur", "leopard", "llama", "lynx",
    "macaque", "manatee", "marmoset", "marmot", "meerkat", "mink", "mongoose", "moose", "mule",
    "muskox", "newt", "ocelot", "opossum", "orangutan", "oryx", "ox", "panther", "platypus",
    "polecat", "porcupine", "possum", "puma", "quokka", "raccoon", "ram", "reindeer", "rhino",
    "salamander", "serval", "shrew", "skunk", "squirrel", "stoat", "tapir", "vole", "walrus",
    "warthog", "weasel", "wildebeest", "wolverine", "wombat", "yak", "blackbird", "bluejay",
    "canary", "cardinal", "chickadee", "condor", "crane", "cuckoo", "egret", "finch",
    "flamingo", "gull", "heron", "hummingbird", "ibis", "jay", "kestrel", "kingfisher", "kite",
    "lark", "loon", "macaw", "magpie", "mockingbird", "nightingale", "ostrich", "owl",
    "parakeet", "partridge", "peacock", "pelican", "penguin", "pheasant", "puffin", "quail",
    "robin", "rooster", "sparrow", "starling", "stork", "swallow", "swift", "toucan",
    "vulture", "woodpecker", "wren", "anchovy", "bass", "carp", "catfish", "clam", "cod",
    "crab", "dolphinfish", "flounder", "grouper", "guppy", "halibut", "herring", "jellyfish",
    "lobster", "mackerel", "marlin", "minnow", "mussel", "octopus", "oyster", "perch", "pike",
    "piranha", "ray", "salmon", "sardine", "snapper", "sole", "starfish", "stingray",
    "swordfish", "trout", "tuna", "alligator", "anaconda", "asp", "boa", "chameleon",
    "crocodile", "iguana", "python", "skink", "tadpole", "tortoise", "beetle", "butterfly",
    "caterpillar", "cicada", "cockroach", "cricket", "dragonfly", "firefly", "flea",
];
const ANIMAL_COUNT: u64 = ANIMAL_NAMES.len() as u64;

pub fn to_animal_names(number: u64) -> String {
    let mut result: Vec<&str> = Vec::new();

    if number == 0 {
        return ANIMAL_NAMES[0].parse().unwrap();
    }

    let mut value = number;
    while value != 0 {
        let digit = (value % ANIMAL_COUNT) as usize;
        value /= ANIMAL_COUNT;
        result.push(ANIMAL_NAMES[digit]);
    }

    // We calculated the numbers in Little-Endian,
    // now convert to Big-Endian for backwards compatibility with old data.
    result.reverse();

    result.join("-")
}

#[test]
fn test_to_animal_names() {
    // The animal list was widened from 64 to 256 entries (8 bits/word
    // instead of 6) so a random u32 ID typically needs only 4 words
    // instead of up to 6. Note this changes the meaning of any
    // multi-word slug generated under the old 64-animal scheme - see the
    // commit message for details.
    assert_eq!(to_animal_names(0), "ant");
    assert_eq!(to_animal_names(1), "eel");
    assert_eq!(to_animal_names(256), "eel-ant");
    assert_eq!(to_animal_names(70000), "eel-fly-llama");
}

pub fn to_u64(animal_names: &str) -> Result<u64, &str> {
    let mut result: u64 = 0;

    for animal in animal_names.split('-') {
        let animal_index = ANIMAL_NAMES.iter().position(|&r| r == animal);
        match animal_index {
            None => return Err("Failed to convert animal name to u64!"),
            Some(idx) => {
                result = result * ANIMAL_COUNT + (idx as u64);
            }
        }
    }

    Ok(result)
}

#[test]
fn test_animal_name_to_u64() {
    assert_eq!(to_u64("ant"), Ok(0));
    assert_eq!(to_u64("eel"), Ok(1));
    assert_eq!(to_u64("eel-ant"), Ok(256));
    assert_eq!(to_u64("eel-fly-llama"), Ok(70000));
}
