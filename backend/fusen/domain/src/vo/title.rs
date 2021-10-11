use crate::vo::ValueObject;
use anyhow::bail;
use anyhow::Error;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FusenTitle(String);

impl ValueObject for FusenTitle {}

impl FromStr for FusenTitle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Regex::new(r"^\w[\w\s]*\w$")?.is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            bail!("invalid string {}", s)
        }
    }
}

impl ToString for FusenTitle {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fusen_title() {
        assert!("title".parse::<FusenTitle>().is_ok());
        assert!("ti".parse::<FusenTitle>().is_ok());
        assert!("ti    tle".parse::<FusenTitle>().is_ok());
        assert!("ti    t    le".parse::<FusenTitle>().is_ok());
        assert!("1234567890abcdefg".parse::<FusenTitle>().is_ok());
        assert!("くぁｗせｄｒｆｔｇｙふじこｌｐ"
            .parse::<FusenTitle>()
            .is_ok());

        assert!("".parse::<FusenTitle>().is_err());
        assert!("t".parse::<FusenTitle>().is_err());
        assert!("         ".parse::<FusenTitle>().is_err());
        assert!("  title  ".parse::<FusenTitle>().is_err());
        assert!("title    ".parse::<FusenTitle>().is_err());
        assert!("    title".parse::<FusenTitle>().is_err());
    }

    #[test]
    fn test_fusen_title_to_string() {
        assert_eq!(
            "くぁｗせｄｒｆｔｇｙふじこｌｐ"
                .parse::<FusenTitle>()
                .unwrap()
                .to_string(),
            "くぁｗせｄｒｆｔｇｙふじこｌｐ".to_string(),
        );
    }

    #[test]
    fn test_fusen_note_eq() {
        let vo = "title".parse::<FusenTitle>().unwrap();
        let cloned = vo.clone();
        let another = "hoge".parse::<FusenTitle>().unwrap();

        assert_eq!(vo, cloned);
        assert_ne!(vo, another);
    }
}
