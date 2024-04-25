use uuid::Uuid;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;

pub struct RemoteItem {
    pub feed_guid: Uuid,
    pub item_guid: Option<String>,
    pub medium: Option<String>,
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
