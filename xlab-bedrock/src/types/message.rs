use aws_sdk_bedrockruntime::types as aws_bedrock;

use xlab::{
    OneOrMany,
    completion::CompletionError,
    message::{AssistantContent, Message, UserContent},
};

use super::{assistant_content::xlabAssistantContent, user_content::xlabUserContent};

pub struct xlabMessage(pub Message);

impl TryFrom<xlabMessage> for aws_bedrock::Message {
    type Error = CompletionError;

    fn try_from(value: xlabMessage) -> Result<Self, Self::Error> {
        let result = match value.0 {
            Message::User { content } => {
                let message_content = content
                    .into_iter()
                    .map(|user_content| xlabUserContent(user_content).try_into())
                    .collect::<Result<Vec<Vec<_>>, _>>()
                    .map_err(|e| CompletionError::RequestError(Box::new(e)))
                    .map(|nested| nested.into_iter().flatten().collect())?;

                aws_bedrock::Message::builder()
                    .role(aws_bedrock::ConversationRole::User)
                    .set_content(Some(message_content))
                    .build()
                    .map_err(|e| CompletionError::RequestError(Box::new(e)))?
            }
            Message::Assistant { content, .. } => aws_bedrock::Message::builder()
                .role(aws_bedrock::ConversationRole::Assistant)
                .set_content(Some(
                    content
                        .into_iter()
                        .map(|content| xlabAssistantContent(content).try_into())
                        .collect::<Result<Vec<aws_bedrock::ContentBlock>, _>>()?,
                ))
                .build()
                .map_err(|e| CompletionError::RequestError(Box::new(e)))?,
        };
        Ok(result)
    }
}

impl TryFrom<aws_bedrock::Message> for xlabMessage {
    type Error = CompletionError;

    fn try_from(message: aws_bedrock::Message) -> Result<Self, Self::Error> {
        match message.role {
            aws_bedrock::ConversationRole::Assistant => {
                let assistant_content = message
                    .content
                    .into_iter()
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<xlabAssistantContent>, _>>()?
                    .into_iter()
                    .map(|xlab_assistant_content| xlab_assistant_content.0)
                    .collect::<Vec<AssistantContent>>();

                let content = OneOrMany::many(assistant_content)
                    .map_err(|e| CompletionError::RequestError(Box::new(e)))?;

                Ok(xlabMessage(Message::Assistant { content, id: None }))
            }
            aws_bedrock::ConversationRole::User => {
                let user_content = message
                    .content
                    .into_iter()
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<xlabUserContent>, _>>()?
                    .into_iter()
                    .map(|user_content| user_content.0)
                    .collect::<Vec<UserContent>>();

                let content = OneOrMany::many(user_content)
                    .map_err(|e| CompletionError::RequestError(Box::new(e)))?;
                Ok(xlabMessage(Message::User { content }))
            }
            _ => Err(CompletionError::ProviderError(
                "AWS Bedrock returned unsupported ConversationRole".into(),
            )),
        }
    }
}

impl TryFrom<super::converse_output::Message> for xlabMessage {
    type Error = CompletionError;

    fn try_from(message: super::converse_output::Message) -> Result<Self, Self::Error> {
        let message = aws_bedrock::Message::try_from(message)
            .map_err(|x| CompletionError::ProviderError(format!("Type conversion error: {x}")))?;

        Self::try_from(message)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::message::xlabMessage;
    use aws_sdk_bedrockruntime::types as aws_bedrock;
    use xlab::{
        OneOrMany,
        message::{Message, UserContent},
    };

    #[test]
    fn message_to_aws_message() {
        let message = Message::User {
            content: OneOrMany::one(UserContent::Text("text".into())),
        };
        let aws_message: Result<aws_bedrock::Message, _> = xlabMessage(message).try_into();
        assert!(aws_message.is_ok());
        let aws_message = aws_message.unwrap();
        assert_eq!(aws_message.role, aws_bedrock::ConversationRole::User);
        assert_eq!(
            aws_message.content,
            vec![aws_bedrock::ContentBlock::Text("text".into())]
        );
    }
}
