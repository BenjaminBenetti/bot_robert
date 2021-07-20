use rand;
use crate::*;
use crate::slash_command::model::Joke;
use rand::Rng;

/// produce a joke.
/// ### params
/// `args` additional joke args that one day could effect the joke
/// ### return
/// the joke text.
pub fn tell_joke(_args: &String) -> String {

    if let Ok(jokes) = serde_json::from_str::<Vec<Joke>>(resources::DAD_JOKES) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..(jokes.len() - 1));

        if let Some(joke) = jokes.get(index) {
            format!("{}\n{}", joke.setup,joke.punchline)
        }
        else
        {
            String::from("Ya.... so ... I would have just have thrown ArrayIndexOutOfBounds \
             but the POWER of Rust has prevented that... I still don't have a joke for you though :(")
        }
    }
    else
    {
        String::from("RIP JSON")
    }
}