use std::time::Duration;

#[derive(PartialEq, PartialOrd, Debug)]
struct Cue {
    start: Duration,
    end: Duration,
    text: String,
}

enum ParserState {
    InHeader,
    BlankLineAfterHeader,
    BetweenCues,
    InCue,
}

#[derive(Debug)]
enum Error {
    InvalidHeader,
    InvalidCueTime,
}

fn parse_duration(text: &str) -> Result<Duration, Error> {
    let parts: Vec<_> = text.splitn(3, ':').collect();

    let mut secs: u64 = 0;
    let mut nanosecs: u32 = 0;

    for (i, part) in parts.iter().enumerate() {
        let mut part = *part;
        if i == parts.len() - 1 {
            // this is the last part of the duration string
            // it may contain a millisecond thing like `0:27.512`

            // split it by the dot
            let part_split: Vec<_> = part.splitn(2, '.').collect();

            // if it has a dot, then handle the millisecond part
            if part_split.len() == 2 {
                let part2: u32 = part_split[1].parse().map_err(|_| Error::InvalidCueTime)?;

                // to convert it to nanoseconds, we need to handle many cases, e.g.:
                // "27.51" - 2 decimal places, need to multiply 52 by 10000000
                // "27.512" - 3 decimal places, need to multiply 52 by 1000000
                // "27.5129999999999999" - 16 decimal places, need to trim by 7 places
                let places_to_move = 9 - part_split[1].len() as i32;
                nanosecs += if places_to_move > 0 {
                    part2 * u32::pow(10, places_to_move.abs() as u32)
                } else {
                    part2 / u32::pow(10, places_to_move.abs() as u32)
                };
            }

            part = part_split[0];
        }

        // parse the part into a positive integer
        let part: u64 = part.parse().map_err(|_| Error::InvalidCueTime)?;

        // multiply that integer by a power of 60, then add to `secs`
        secs += u64::pow(60, (parts.len() - 1).abs_diff(i) as u32) * part;
    }

    Ok(Duration::new(secs, nanosecs))
}

fn from_lines<'a, T>(lines: T) -> Result<Vec<Cue>, Error>
    where
        T: Iterator<Item=&'a str>
{
    let mut cues = vec![];
    let mut current_cue_lines = vec![];
    let mut state = ParserState::InHeader;

    for line in lines {
        match state {
            ParserState::InHeader => {
                if line != "WEBVTT" { return Err(Error::InvalidHeader); }

                state = ParserState::BlankLineAfterHeader;
            }
            ParserState::BlankLineAfterHeader => {
                if line.len() != 0 { return Err(Error::InvalidHeader); }

                state = ParserState::BetweenCues;
            }
            ParserState::BetweenCues => {
                if line.len() == 0 { continue; }

                let timestamps: Result<Vec<_>, _> = line
                    .splitn(2, " --> ")
                    .map(parse_duration)
                    .collect();
                let timestamps = match timestamps {
                    Ok(dur) => dur,
                    Err(x) => return Err(x),
                };
                if timestamps.len() != 2 { return Err(Error::InvalidCueTime); }

                let cue = Cue { start: timestamps[0], end: timestamps[1], text: String::from("") };
                cues.push(cue);

                state = ParserState::InCue;
            }
            ParserState::InCue => {
                if line.len() == 0 {
                    // end of cue, combine the strings and set the last cue's text to it
                    let current_cue = cues.last_mut().expect("there should be an existing empty cue in the vec, but none is found");
                    let text = current_cue_lines.join("\n");
                    current_cue.text = text;
                    current_cue_lines = vec![];
                    state = ParserState::BetweenCues;
                } else {
                    // more text in this cue, add them to the list
                    current_cue_lines.push(line);
                }
            }
        }
    }

    // there are still lines in the list
    // this means the last line didn't terminate with a newline
    // the loop above didn't received any blank lines to terminate the current cue
    // we'll just copy the code from the `ParserState::InCue` && `line.len() == 0` block
    if current_cue_lines.len() > 0 {
        // end of cue, combine the strings and set the last cue's text to it
        let current_cue = cues.last_mut().expect("there should be an existing empty cue in the vec, but none is found");
        let text = current_cue_lines.join("\n");
        current_cue.text = text;
        // current_cue_lines = vec![];  // this variable isn't used at this point
        // state = ParserState::BetweenCues;  // no need for state now
    }

    Ok(cues)
}

fn from_str(text: &str) -> Result<Vec<Cue>, Error> {
    from_lines(text.lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod duration {
        use super::*;

        #[test]
        fn seconds() {
            let parsed = parse_duration("0:5").unwrap();
            let expected = Duration::new(5, 0);
            assert_eq!(parsed, expected);
        }

        #[test]
        fn milliseconds() {
            let parsed = parse_duration("0:00:05.135").unwrap();
            let expected = Duration::new(5, 135 * 1000000);
            assert_eq!(parsed, expected);
        }

        #[test]
        fn minutes() {
            let parsed = parse_duration("0:02:37.742").unwrap();
            let expected = Duration::new(2 * 60 + 37, 742 * 1000000);
            assert_eq!(parsed, expected);
        }

        #[test]
        fn hours() {
            let parsed = parse_duration("2:16:52.052").unwrap();
            let expected = Duration::new(2 * 60 * 60 + 16 * 60 + 52, 052 * 1000000);
            assert_eq!(parsed, expected);
        }
    }

    mod vtt {
        use indoc::indoc;
        use super::*;

        #[test]
        fn basic() {
            let text = indoc! {"
                WEBVTT

                00:19.920 --> 00:21.120
                Hello.

                00:21.120 --> 00:23.680
                Hello, hi everyone.

                00:23.680 --> 00:27.800
                We'll just wait a couple of minutes

            "};
            let result = from_str(text).unwrap();
            let expected = vec![
                Cue {
                    start: Duration::new(19, 920 * 1000000),
                    end: Duration::new(21, 120 * 1000000),
                    text: String::from("Hello."),
                },
                Cue {
                    start: Duration::new(21, 120 * 1000000),
                    end: Duration::new(23, 680 * 1000000),
                    text: String::from("Hello, hi everyone."),
                },
                Cue {
                    start: Duration::new(23, 680 * 1000000),
                    end: Duration::new(27, 800 * 1000000),
                    text: String::from("We'll just wait a couple of minutes"),
                },
            ];
            assert_eq!(result, expected);
        }

        #[test]
        fn multiline() {
            let text = indoc! {"
                WEBVTT

                00:19.920 --> 00:28.150
                Hello.
                Hello, hi everyone.

                00:28.150 --> 05:27.800
                We'll just wait a couple of minutes.
                And a couple more minutes...
            "};
            let result = from_str(text).unwrap();
            let expected = vec![
                Cue {
                    start: Duration::new(19, 920 * 1000000),
                    end: Duration::new(28, 150 * 1000000),
                    text: String::from("Hello.\nHello, hi everyone."),
                },
                Cue {
                    start: Duration::new(28, 150 * 1000000),
                    end: Duration::new(5 * 60 + 27, 800 * 1000000),
                    text: String::from("We'll just wait a couple of minutes.\nAnd a couple more minutes..."),
                },
            ];
            assert_eq!(result, expected);
        }
    }
}
