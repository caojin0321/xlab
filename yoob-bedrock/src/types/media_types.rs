use aws_sdk_bedrockruntime::types::DocumentFormat;
use yoob::{
    completion::CompletionError,
    message::{DocumentMediaType, MimeType},
};

pub struct yoobDocumentMediaType(pub DocumentMediaType);

impl TryFrom<yoobDocumentMediaType> for DocumentFormat {
    type Error = CompletionError;

    fn try_from(value: yoobDocumentMediaType) -> Result<Self, Self::Error> {
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

impl TryFrom<DocumentFormat> for yoobDocumentMediaType {
    type Error = CompletionError;

    fn try_from(value: DocumentFormat) -> Result<Self, Self::Error> {
        match value {
            DocumentFormat::Csv => Ok(yoobDocumentMediaType(DocumentMediaType::CSV)),
            DocumentFormat::Html => Ok(yoobDocumentMediaType(DocumentMediaType::HTML)),
            DocumentFormat::Md => Ok(yoobDocumentMediaType(DocumentMediaType::MARKDOWN)),
            DocumentFormat::Pdf => Ok(yoobDocumentMediaType(DocumentMediaType::PDF)),
            DocumentFormat::Txt => Ok(yoobDocumentMediaType(DocumentMediaType::TXT)),
            e => Err(CompletionError::ProviderError(format!(
                "Unsupported media type {e}"
            ))),
        }
    }
}
