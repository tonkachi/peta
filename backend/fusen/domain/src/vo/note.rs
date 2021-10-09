use crate::vo::ValueObject;
use anyhow::Error;
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FusenNote(String);

impl ValueObject for FusenNote {}

impl FromStr for FusenNote {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl ToString for FusenNote {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fusen_note() {
        assert!("note".parse::<FusenNote>().is_ok());
        assert!("1234567890abcdefg".parse::<FusenNote>().is_ok());
        assert!("くぁｗせｄｒｆｔｇｙふじこｌｐ"
            .parse::<FusenNote>()
            .is_ok());
        assert!("くぁｗせ\nｄｒｆｔｇｙ\nふじこｌｐ"
            .parse::<FusenNote>()
            .is_ok());
        assert!("".parse::<FusenNote>().is_ok());
        assert!("    ".parse::<FusenNote>().is_ok());
    }

    #[test]
    fn test_fusen_note_to_string() {
        assert_eq!(
            "くぁｗせｄｒｆｔｇｙふじこｌｐ"
                .parse::<FusenNote>()
                .unwrap()
                .to_string(),
            "くぁｗせｄｒｆｔｇｙふじこｌｐ".to_string(),
        );
        assert_eq!(
            "くぁｗせ\nｄｒｆｔｇｙ\nふじこｌｐ"
                .parse::<FusenNote>()
                .unwrap()
                .to_string(),
            "くぁｗせ\nｄｒｆｔｇｙ\nふじこｌｐ".to_string(),
        );
    }

    #[test]
    fn test_fusen_note_eq() {
        let vo = "note".parse::<FusenNote>().unwrap();
        let cloned = vo.clone();
        let another = "note\ne".parse::<FusenNote>().unwrap();

        assert_eq!(vo, cloned);
        assert_ne!(vo, another);
    }
}
