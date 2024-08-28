use crate::itunes;
use crate::podcast;
use crate::Language;

use uuid::Uuid;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;

pub struct Rss {
    pub channel: Channel,
}

impl yaserde::YaSerialize for Rss {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let data_event = xml::writer::XmlEvent::characters("\n");
        writer.write(data_event).map_err(|e| e.to_string())?;

        let mut rss_el = xml::writer::XmlEvent::start_element("rss");
        rss_el = rss_el.attr("version", "2.0");

        if self.channel.has_itunes_tags() {
            rss_el = rss_el.ns("itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd");
        }

        if self.channel.has_podcast_tags() {
            rss_el = rss_el.ns("podcast", "https://podcastindex.org/namespace/1.0");
        }

        writer.write(rss_el).map_err(|e| e.to_string())?;

        self.channel.serialize(writer)?;

        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

#[derive(Default)]
pub struct Channel {
    pub description: Option<String>,
    pub generator: Option<String>,
    pub language: Option<Language>,
    pub last_build_date: Option<chrono::DateTime<chrono::Utc>>,
    pub title: Option<String>,

    pub itunes_image: Option<itunes::Image>,

    pub podcast_funding: Vec<podcast::Funding>,
    pub podcast_guid: Option<Uuid>,
    pub podcast_locked: Option<podcast::Locked>,
    pub podcast_medium: Option<podcast::Medium>,
    pub podcast_remote_items: Vec<podcast::RemoteItem>,
}

impl Channel {
    fn has_itunes_tags(&self) -> bool {
        self.itunes_image.is_some()
    }

    fn has_podcast_tags(&self) -> bool {
        !self.podcast_funding.is_empty()
            || self.podcast_guid.is_some()
            || self.podcast_locked.is_some()
            || self.podcast_medium.is_some()
            || !self.podcast_remote_items.is_empty()
    }
}

impl yaserde::YaSerialize for Channel {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        writer
            .write(xml::writer::XmlEvent::start_element("channel"))
            .map_err(|e| e.to_string())?;

        if let Some(description) = &self.description {
            writer
                .write(xml::writer::XmlEvent::start_element("description"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(description))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
        }

        if let Some(generator) = &self.generator {
            writer
                .write(xml::writer::XmlEvent::start_element("generator"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(generator))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
        }

        if let Some(language) = &self.language {
            writer
                .write(xml::writer::XmlEvent::start_element("language"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(
                    language.to_string().as_str(),
                ))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
        }

        if let Some(last_build_date) = &self.last_build_date {
            writer
                .write(xml::writer::XmlEvent::start_element("lastBuildDate"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(
                    &last_build_date
                        .to_rfc2822()
                        .to_string()
                        .replace("+0000", "GMT"),
                ))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
        }

        if let Some(title) = &self.title {
            writer
                .write(xml::writer::XmlEvent::start_element("title"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(title))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
        }

        if let Some(itunes_image) = &self.itunes_image {
            itunes_image.serialize(writer)?;
        }

        if let Some(podcast_guid) = &self.podcast_guid {
            crate::podcast::serialize_guid(writer, *podcast_guid)?;
        }

        for funding in &self.podcast_funding {
            funding.serialize(writer)?;
        }

        if let Some(podcast_locked) = &self.podcast_locked {
            podcast_locked.serialize(writer)?;
        }

        if let Some(podcast_medium) = &self.podcast_medium {
            podcast_medium.serialize(writer)?;
        }

        for remote_item in &self.podcast_remote_items {
            remote_item.serialize(writer)?;
        }

        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}
