use anyhow::Result;
use regex::Regex;
use serde_json::Value;

use crate::schema::Schema;

enum PQH {
    Param,
    Query,
    Header,
}

pub struct Converter {
    gap: u8,
    offset: u8,
    comment: String,
    swag_schema: String,
}

impl Converter {
    pub fn new(gap: u8, offset: u8, comment: &str) -> Self {
        let c = if comment.is_empty() {
            "".to_string()
        } else {
            format!("{} ", comment)
        };
        Self {
            gap,
            offset,
            comment: c,
            swag_schema: "".to_string(),
        }
    }

    pub fn convert_schema(&mut self, schema: &Schema) -> Result<&str> {
        self.write_line(&format!("{}:", schema.path));
        self.add_offset(1);
        self.write_line(&format!("{}:", schema.method.to_lowercase()));
        self.add_offset(1);
        self.write_line("description: unknown");
        self.write_line("tags:");
        self.add_offset(1);
        self.write_line("- unknown");
        self.add_offset(-1);
        self.write_line("parameters:");
        self.add_offset(1);

        for key in schema.get_param_keys() {
            self.pqh_convert(key, PQH::Param);
        }

        for key in schema.get_query_keys() {
            self.pqh_convert(key, PQH::Query);
        }

        for (key, _) in schema.get_headers().unwrap().iter() {
            self.pqh_convert(key.as_str(), PQH::Header);
        }
        self.add_offset(-1);

        if let Some(body) = schema.body {
            self.req_convert(body)?;
        }

        self.res_convert(&schema.res)?;

        Ok(&self.swag_schema)
    }

    fn req_convert(&mut self, json_str: &str) -> Result<()> {
        self.write_line("requestBody:");
        self.add_offset(1);
        self.write_line("content:");
        self.add_offset(1);
        self.write_line("application/json:");
        self.add_offset(1);
        self.write_line("schema:");
        self.add_offset(1);
        self.convert_json(json_str)?;
        self.add_offset(-5);
        Ok(())
    }

    fn res_convert(&mut self, json_str: &str) -> Result<()> {
        self.write_line("responses:");
        self.add_offset(1);
        self.write_line("200:");
        self.add_offset(1);
        // self.write_line("description: unknown");
        self.write_line("content:");
        self.add_offset(1);
        self.write_line("application/json:");
        self.add_offset(1);
        self.write_line("schema:");
        self.add_offset(1);
        self.convert_json(json_str)?;
        Ok(())
    }

    fn convert_json(&mut self, json_str: &str) -> Result<()> {
        self.object_convert(json_str)?;
        Ok(())
    }

    fn offsetter(&self) -> String {
        let offset = self.offset * self.gap;
        " ".repeat(offset as usize)
    }

    fn add_offset(&mut self, num: i8) {
        self.offset = (self.offset as i16 + num as i16) as u8;
    }

    fn write_line(&mut self, content: &str) {
        self.swag_schema.push_str(&format!(
            "{}{}{}\n",
            self.comment,
            self.offsetter(),
            content
        ));
    }

    fn object_convert(&mut self, json_str: &str) -> Result<()> {
        let json_value: Value = serde_json::from_str(json_str)?;
        let item: Option<&Value> = if is_array(&json_value) {
            Some(&json_value[0])
        } else {
            Some(&json_value)
        };
        match item {
            Some(item) => match item {
                Value::String(_) => self.write_line("type: string"),
                Value::Number(_) => self.write_line("type: number"),
                Value::Bool(_) => self.write_line("type: boolean"),
                Value::Object(map) => {
                    self.write_line("type: object");
                    self.write_line("properties:");
                    self.add_offset(1);
                    for (key, value) in map.iter() {
                        self.add_property(key, value);
                    }
                }
                _ => self.write_line("type: unknow"),
            },
            None => self.write_line("type: undefined"),
        };
        Ok(())
    }

    fn add_property(&mut self, key: &str, value: &Value) {
        self.write_line(&format!("{}:", key));
        self.add_offset(1);
        match value {
            Value::Number(_) => {
                self.write_line("type: number");
                self.add_offset(-1);
            }
            Value::Bool(_) => {
                self.write_line("type: boolean");
                self.add_offset(-1);
            }
            Value::Object(_) => {
                self.object_convert(&value.to_string()).unwrap();
                self.add_offset(-2);
            }
            Value::String(_) => {
                self.write_line("type: string");
                self.add_offset(-1);
                if is_date(&value.to_string()) {
                    self.add_offset(1);
                    self.write_line("format: date");
                    self.add_offset(-1);
                }
            }
            Value::Array(_) => {
                self.write_line("type: array");
                self.write_line("items:");
                self.add_offset(1);
                self.object_convert(&value.to_string()).unwrap();
                self.add_offset(-2);
            }
            _ => {
                self.write_line("type: undefined");
                self.add_offset(-1);
            }
        }
    }

    fn pqh_convert(&mut self, key: &str, pqh_type: PQH) {
        self.write_line(&format!("- name: {}", key));
        self.add_offset(1);
        match pqh_type {
            PQH::Param => {
                self.write_line("in: path");
            }
            PQH::Query => {
                self.write_line("in: query");
            }
            PQH::Header => {
                self.write_line("in: header");
            }
        }
        self.write_line("required: true");
        self.write_line("schema:");
        self.add_offset(1);
        self.write_line("type: string");
        self.add_offset(-2);
    }
}

fn is_array(json_value: &Value) -> bool {
    match json_value {
        Value::Array(_) => true,
        _ => false,
    }
}

fn is_date(s: &str) -> bool {
    Regex::new(r"^\d{4}[-/]\d{2}[-/]\d{2}(?:[ T]\d{2}:\d{2}:\d{2}(?:Z)?)?$")
        .unwrap()
        .is_match(&s.replace("\"", ""))
}
