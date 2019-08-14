use crate::errors;
use crate::random;

pub enum Command {
    Help,
    Number,
    Choice,
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Command::Help => "help",
            Command::Number => "num",
            Command::Choice => "choice",
        }
    }

    pub fn handle(&self, options: &Vec<String>) -> Result<String, errors::UserError> {
        match *self {
            Command::Help => Command::help(),
            Command::Number => Command::random_number(&options),
            Command::Choice => Command::random_choice(&options),
        }
    }

    fn help() -> Result<String, errors::UserError> {
        Ok("Random supports these options:\n\
            `/rand help` : Show help\n\
            `/rand num [low=0, high=10]`: Get random number between low and high inclusively\n\
            `/rand choice option1 [option2 option3 ...]`: Get random choice from a list".to_string())
    }

    fn random_number(options: &Vec<String>) -> Result<String, errors::UserError> {
        let (low, high): (i64, i64) =
            if options.len() < 2 {
                (0, 10)
            } else {
                (options[0].parse().map_err(|_e| errors::UserError::InputError)?,
                 options[1].parse().map_err(|_e| errors::UserError::InputError)?)
            };

        let rand_number = random::gen_random_range(low, high + 1)
            .map_err(|_e| errors::UserError::InputError)?;

        Ok(format!("*{}*", rand_number))
    }

    fn random_choice(options: &Vec<String>) -> Result<String, errors::UserError> {
        let rand_choice = random::select_random(options)
            .map_err(|_e| errors::UserError::InputError)?;

        Ok(format!("*{}*", rand_choice))
    }
}

