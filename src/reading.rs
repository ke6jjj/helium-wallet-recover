/// Different ways that humans might read a list of words that
/// they have written down.
///
/// Typically a seed phrase is written down in two columns of
/// equal length, like this:
///
/// 1. ONE      7. SEVEN
/// 2. TWO      8. EIGHT
/// 3. THREE    9. NINE
/// 4. FOUR    10. TEN
/// 5. FIVE    11. ELEVEN
/// 6. SIX     12. TWELVE
///
/// Given this traditional order, this module computes various ways
/// in which humans have been observed to write the words down in
/// the wrong order.
///
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadingError {
    #[error("number of words in reading must be even")]
    WordsUneven,
}

/// Produce a reading where the columns are written in the wrong order
/// (second column first, first column second)
fn transpose_columns(v: &Vec<u8>) -> Vec<u8> {
    let len = v.len() as u8;
    let column_len = len / 2;
    let res: Vec<u8> = v.iter().map(|x| ((*x) + column_len) % len).collect();
    res
}

/// Produce a reading where the columns are written down row-wise.
fn read_rowise(v: &Vec<u8>) -> Vec<u8> {
    let len = v.len();
    let column_len = len / 2;
    let column_a = &v[0..column_len];
    let column_b = &v[column_len..len];
    let res: Vec<u8> = column_a
        .iter()
        .interleave(column_b.iter()).copied()
        .collect();
    res
}

pub fn generate_readings(num_words: u8) -> Result<Vec<Vec<u8>>, ReadingError> {
    if num_words % 2 != 0 {
        return Err(ReadingError::WordsUneven);
    }
    let base: Vec<u8> = (0..num_words).collect();
    let mut res: Vec<Vec<u8>> = Vec::new();
    res.push(base.to_owned());
    res.push(transpose_columns(&base));
    res.push(read_rowise(&base));
    res.push(read_rowise(&transpose_columns(&base)));
    Ok(res)
}

#[cfg(test)]
mod test {

    use crate::reading::ReadingError;

    use super::generate_readings;

    #[test]
    fn test_it() {
        generate_readings(12).expect("should have worked");
    }

    #[test]
    fn test_another() {
        let r = generate_readings(12).expect("work");
        for vec in r {
            print!("[");
            for v in vec {
                print!("{}, ", v)
            }
            println!("]")
        }
    }

    #[test]
    fn angry() {
        assert!(match generate_readings(11) {
            Err(ReadingError::WordsUneven) => true,
            Ok(_) => false,
        });
    }
}
