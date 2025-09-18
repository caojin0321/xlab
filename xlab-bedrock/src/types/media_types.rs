use aws_sdk_bedrockruntime::types::DocumentFormat;
use xlab::{
    completion::CompletionError,
    message::{DocumentMediaType, MimeType},
};

pub struct xlabDocumentMediaType(pub DocumentMediaType);

impl TryFrom<xlabDocumentMediaType> for DocumentFormat {
    type Error = CompletionError;

    fn try_from(value: xlabDocumentMediaType) -> Result<Self, Self::Error> {
        match value.0 {
            DocumentMediaType::PDF => Ok(DocumentFormat::Pdf),
            DocumentMediaType::TXT => Ok(DocumentFormat::Txt),
            DocumentMediaType::HTML => Ok(DocumentFormat::Html),
            DocumentMediaType::MARKDOWN => Ok(DocumentFormat::Md),
            DocumentMediaType::CSV => Ok(DocumentFormat::Csv),
            e => Err(CompletionError::ProviderError(format!(
                "Unsupported media type {}",
                e.to_mime_type()
            ))),
        }
    }
}

impl TryFrom<DocumentFormat> for xlabDocumentMediaType {
    type Error = CompletionError;

    fn try_from(value: DocumentFormat) -> Result<Self, Self::Error> {
        match value {
            DocumentFormat::Csv => Ok(xlabDocumentMediaType(DocumentMediaType::CSV)),
            DocumentFormat::Html => Ok(xlabDocumentMediaType(DocumentMediaType::HTML)),
            DocumentFormat::Md => Ok(xlabDocumentMediaType(DocumentMediaType::MARKDOWN)),
            DocumentFormat::Pdf => Ok(xlabDocumentMediaType(DocumentMediaType::PDF)),
            DocumentFormat::Txt => Ok(xlabDocumentMediaType(DocumentMediaType::TXT)),
            e => Err(CompletionError::ProviderError(format!(
                "Unsupported media type {e}"
            ))),
        }
    }
}
