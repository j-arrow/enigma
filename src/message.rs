use chrono::{DateTime, Local};
use crate::enigma::EncodingResult;
use std::fmt::{Display, Formatter, Error};

pub(crate) struct Message {
    message_time: String,
    receiver: String,
    sender: String,
    message_length: usize,
    basic_position: String,
    encoded_message_key: String,
    encoded_message: String
}

impl Message {

    pub(crate) fn compose(message_time: DateTime<Local>, receiver: String,
               sender: String, encoding_result: EncodingResult) -> Self {
        Message {
            message_time: message_time.format("%H%M").to_string(),
            receiver,
            sender,
            message_length: encoding_result.message_length,
            basic_position: encoding_result.basic_position,
            encoded_message_key: encoding_result.encoded_message_key,
            encoded_message: encoding_result.encoded_message
        }
    }
}

impl Display for Message {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // for example: U6Z DE C 1510 = 49 = EHZ TBS = TVEXS QBLTW LDAHH YEOEF
        // means: message sent from C to D6Z on 15:10, message is 49 characters long; basic position is EHZ,
        // encrypted message key is TBS, identification group for a day is EVEXS and the rest is encoded
        // message split with space

        let encoded_message = self.encoded_message.chars()
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f,
               "{receiver} DE {sender} {sending_time} = {message_length} = {basic_position} {encrypted_message_key} = {identification_group} {encoded_message}",
               receiver = self.receiver,
               sender = self.sender,
               sending_time = self.message_time,
               message_length = self.message_length,
               basic_position = self.basic_position,
               encrypted_message_key = self.encoded_message_key,
               identification_group = "ABCDE",
               encoded_message = encoded_message
        )
    }
}
