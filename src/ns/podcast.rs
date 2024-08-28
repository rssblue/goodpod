use url::Url;
use uuid::Uuid;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;

pub struct RemoteItem {
    pub feed_guid: Uuid,
    pub item_guid: Option<String>,
    pub feed_url: Option<Url>,
    pub medium: Option<Medium>,
}

impl yaserde::YaSerialize for RemoteItem {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let mut el = xml::writer::XmlEvent::start_element("podcast:remoteItem");

        let binding = self.feed_guid.to_string();
        el = el.attr("feedGuid", &binding);

        if let Some(item_guid) = &self.item_guid {
            el = el.attr("itemGuid", item_guid);
        }

        if let Some(feed_url) = &self.feed_url {
            el = el.attr("feedUrl", feed_url.as_str());
        }

        if let Some(medium) = &self.medium {
            el = el.attr("medium", medium.as_ref());
        }

        writer.write(el).map_err(|e| e.to_string())?;

        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

pub struct Locked {
    pub value: bool,
    pub owner_email: Option<String>,
}

impl yaserde::YaSerialize for Locked {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let mut el = xml::writer::XmlEvent::start_element("podcast:locked");

        if let Some(owner_email) = &self.owner_email {
            el = el.attr("owner", owner_email);
        }

        writer.write(el).map_err(|e| e.to_string())?;

        writer
            .write(xml::writer::XmlEvent::characters(match self.value {
                true => "yes",
                false => "no",
            }))
            .map_err(|e| e.to_string())?;

        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

pub enum Medium {
    Podcast,
    Music,
    Video,
    Film,
    Audiobook,
    Newsletter,
    Blog,
    Publisher,

    PodcastList,
    MusicList,
    VideoList,
    FilmList,
    AudiobookList,
    NewsletterList,
    BlogList,
    PublisherList,

    Mixed,
}

impl std::fmt::Display for Medium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Medium {
    fn as_ref(&self) -> &str {
        match self {
            Medium::Podcast => "podcast",
            Medium::Music => "music",
            Medium::Video => "video",
            Medium::Film => "film",
            Medium::Audiobook => "audiobook",
            Medium::Newsletter => "newsletter",
            Medium::Blog => "blog",
            Medium::Publisher => "publisher",
            Medium::PodcastList => "podcastL",
            Medium::MusicList => "musicL",
            Medium::VideoList => "videoL",
            Medium::FilmList => "filmL",
            Medium::AudiobookList => "audiobookL",
            Medium::NewsletterList => "newsletterL",
            Medium::BlogList => "blogL",
            Medium::PublisherList => "publisherL",
            Medium::Mixed => "mixed",
        }
    }
}

impl yaserde::YaSerialize for Medium {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        writer
            .write(xml::writer::XmlEvent::start_element("podcast:medium"))
            .map_err(|e| e.to_string())?;

        writer
            .write(xml::writer::XmlEvent::characters(self.as_ref()))
            .map_err(|e| e.to_string())?;

        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

pub fn serialize_guid<W: std::io::Write>(
    writer: &mut yaserde::ser::Serializer<W>,
    value: Uuid,
) -> Result<(), String> {
    writer
        .write(xml::writer::XmlEvent::start_element("podcast:guid"))
        .map_err(|e| e.to_string())?;

    writer
        .write(xml::writer::XmlEvent::characters(&value.to_string()))
        .map_err(|e| e.to_string())?;

    writer
        .write(xml::writer::XmlEvent::end_element())
        .map_err(|e| e.to_string())
}

pub struct Funding {
    pub display_text: String,
    pub url: Url,
}

impl yaserde::YaSerialize for Funding {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let mut el = xml::writer::XmlEvent::start_element("podcast:funding");

        el = el.attr("url", self.url.as_str());

        writer.write(el).map_err(|e| e.to_string())?;

        writer
            .write(xml::writer::XmlEvent::characters(&self.display_text))
            .map_err(|e| e.to_string())?;

        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}
