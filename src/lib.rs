#![doc = include_str!("../README.md")]

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

        if self.channel.itunes_image.is_some() {
            rss_el = rss_el.ns("itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd");
        }

        if self.channel.podcast_guid.is_some()
            || self.channel.podcast_medium.is_some()
            || self.channel.podcast_remote_items.len() > 0
        {
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

pub struct Channel {
    pub description: Option<String>,
    pub generator: Option<String>,
    pub last_build_date: Option<chrono::DateTime<chrono::Utc>>,
    pub title: Option<String>,
    pub itunes_image: Option<ItunesImage>,
    pub podcast_guid: Option<Uuid>,
    pub podcast_medium: Option<String>,
    pub podcast_remote_items: Vec<PodcastRemoteItem>,
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
            writer
                .write(xml::writer::XmlEvent::start_element("podcast:guid"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(&podcast_guid.to_string()))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
        }

        if let Some(podcast_medium) = &self.podcast_medium {
            writer
                .write(xml::writer::XmlEvent::start_element("podcast:medium"))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::characters(podcast_medium))
                .map_err(|e| e.to_string())?;
            writer
                .write(xml::writer::XmlEvent::end_element())
                .map_err(|e| e.to_string())?;
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

impl Default for Channel {
    fn default() -> Self {
        Self {
            description: None,
            generator: None,
            last_build_date: None,
            title: None,
            itunes_image: None,
            podcast_guid: None,
            podcast_medium: None,
            podcast_remote_items: Vec::new(),
        }
    }
}

pub struct ItunesImage {
    pub href: String,
}

impl yaserde::YaSerialize for ItunesImage {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let mut el = xml::writer::XmlEvent::start_element("itunes:image");
        el = el.attr("href", &self.href);

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

pub struct PodcastRemoteItem {
    pub feed_guid: Uuid,
    pub item_guid: Option<String>,
    pub medium: Option<String>,
}

impl yaserde::YaSerialize for PodcastRemoteItem {
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

        if let Some(medium) = &self.medium {
            el = el.attr("medium", medium);
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
