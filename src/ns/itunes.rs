use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;

pub struct Image {
    pub href: String,
}

impl yaserde::YaSerialize for Image {
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
