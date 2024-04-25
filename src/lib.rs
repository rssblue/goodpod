#![doc = include_str!("../README.md")]

use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use yaserde_derive::YaSerialize as Serialize;

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
        writer
            .write(xml::writer::XmlEvent::start_element("rss").attr("version", "2.0"))
            .map_err(|e| e.to_string())?;

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

#[derive(Serialize)]
#[yaserde(rename = "channel")]
pub struct Channel {
    pub title: String,
    pub description: String,
}
