/*!
  App messages are a specific type of message that developers can generate with their apps.
  Some built-in functionality also uses App Messages, like Apple Pay or Handwriting.
*/

use plist::Value;

use crate::{
    error::plist::PlistParseError,
    message_types::variants::BalloonProvider,
    util::plist::{get_string_from_dict, get_string_from_nested_dict},
};

/// This struct represents Apple's [`MSMessageTemplateLayout`](https://developer.apple.com/documentation/messages/msmessagetemplatelayout).
#[derive(Debug, PartialEq, Eq)]
pub struct AppMessage<'a> {
    /// An image used to represent the message in the transcript
    pub image: Option<&'a str>,
    /// A media file used to represent the message in the transcript
    pub url: Option<&'a str>,
    /// The title for the image or media file
    pub title: Option<&'a str>,
    /// The subtitle for the image or media file
    pub subtitle: Option<&'a str>,
    /// A left-aligned caption for the message bubble
    pub caption: Option<&'a str>,
    /// A left-aligned subcaption for the message bubble
    pub subcaption: Option<&'a str>,
    /// A right-aligned caption for the message bubble
    pub trailing_caption: Option<&'a str>,
    /// A right-aligned subcaption for the message bubble
    pub trailing_subcaption: Option<&'a str>,
    /// The name of the app that created this message
    pub app_name: Option<&'a str>,
    /// This property is set only for Apple Pay system messages
    /// It represents the text that displays in the center of the bubble
    pub ldtext: Option<&'a str>,
}

impl<'a> BalloonProvider<'a> for AppMessage<'a> {
    fn from_map(payload: &'a Value) -> Result<Self, PlistParseError> {
        let user_info = payload
            .as_dictionary()
            .ok_or(PlistParseError::InvalidType(
                "root".to_string(),
                "dictionary".to_string(),
            ))?
            .get("userInfo")
            .ok_or(PlistParseError::MissingKey("userInfo".to_string()))?;
        Ok(AppMessage {
            image: get_string_from_dict(payload, "image"),
            url: get_string_from_nested_dict(payload, "URL"),
            title: get_string_from_dict(user_info, "image-title"),
            subtitle: get_string_from_dict(user_info, "image-subtitle"),
            caption: get_string_from_dict(user_info, "caption"),
            subcaption: get_string_from_dict(user_info, "subcaption"),
            trailing_caption: get_string_from_dict(user_info, "secondary-subcaption"),
            trailing_subcaption: get_string_from_dict(user_info, "tertiary-subcaption"),
            app_name: get_string_from_dict(payload, "an"),
            ldtext: get_string_from_dict(payload, "ldtext"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        message_types::{app::AppMessage, variants::BalloonProvider},
        util::plist::parse_plist,
    };
    use plist::Value;
    use std::env::current_dir;
    use std::fs::File;

    #[test]
    fn test_parse_apple_pay_sent_265() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Sent265.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        let parsed = parse_plist(&plist).unwrap();

        let balloon = AppMessage::from_map(&parsed).unwrap();
        let expected = AppMessage {
            image: None,
            url: Some("data:application/vnd.apple.pkppm;base64,FAKE_BASE64_DATA="),
            title: None,
            subtitle: None,
            caption: Some("Apple\u{a0}Cash"),
            subcaption: Some("$265\u{a0}Payment"),
            trailing_caption: None,
            trailing_subcaption: None,
            app_name: Some("Apple\u{a0}Pay"),
            ldtext: Some("Sent $265 with Apple\u{a0}Pay."),
        };

        assert_eq!(balloon, expected);
    }

    #[test]
    fn test_parse_opentable_invite() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/OpenTableInvited.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        let parsed = parse_plist(&plist).unwrap();

        let balloon = AppMessage::from_map(&parsed).unwrap();
        let expected = AppMessage {
            image: None,
            url: Some("https://www.opentable.com/book/view?rid=0000000&confnumber=00000&invitationId=1234567890-abcd-def-ghij-4u5t1sv3ryc00l"),
            title: Some("Rusty Grill - Boise"),
            subtitle: Some("Reservation Confirmed"),
            caption: Some("Table for 4 people\nSunday, October 17 at 7:45 PM"),
            subcaption: Some("You're invited! Tap to accept."),
            trailing_caption: None,
            trailing_subcaption: None,
            app_name: Some("OpenTable"),
            ldtext: None,
        };
        println!("{:?}", balloon);

        assert_eq!(balloon, expected);
    }

    #[test]
    fn test_parse_slideshow() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Slideshow.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        let parsed = parse_plist(&plist).unwrap();

        let balloon = AppMessage::from_map(&parsed).unwrap();
        let expected = AppMessage {
            image: None,
            url: Some("https://share.icloud.com/photos/1337h4x0r_jk#Home"),
            title: None,
            subtitle: None,
            caption: Some("Home"),
            subcaption: Some("37 Photos"),
            trailing_caption: None,
            trailing_subcaption: None,
            app_name: Some("Photos"),
            ldtext: Some("Home - 37 Photos"),
        };
        println!("{:?}", balloon);

        assert_eq!(balloon, expected);
    }

    #[test]
    fn test_parse_game() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Game.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        let parsed = parse_plist(&plist).unwrap();

        let balloon = AppMessage::from_map(&parsed).unwrap();
        let expected = AppMessage {
            image: None,
            url: Some("data:?ver=48&data=pr3t3ndth3r3154b10b0fd4t4h3re=3"),
            title: None,
            subtitle: None,
            caption: Some("Your move."),
            subcaption: None,
            trailing_caption: None,
            trailing_subcaption: None,
            app_name: Some("GamePigeon"),
            ldtext: Some("Dots & Boxes"),
        };
        println!("{:?}", balloon);

        assert_eq!(balloon, expected);
    }

    #[test]
    fn test_parse_business() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Business.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        let parsed = parse_plist(&plist).unwrap();

        let balloon = AppMessage::from_map(&parsed).unwrap();
        let expected = AppMessage {
            image: None,
            url: Some("?receivedMessage=33c309ab520bc2c76e99c493157ed578&replyMessage=6a991da615f2e75d4aa0de334e529024"),
            title: None,
            subtitle: None,
            caption: Some("Yes, connect me with Goldman Sachs."),
            subcaption: None,
            trailing_caption: None,
            trailing_subcaption: None,
            app_name: Some("Business"),
            ldtext: Some("Yes, connect me with Goldman Sachs."),
        };
        println!("{:?}", balloon);

        assert_eq!(balloon, expected);
    }
}
