mod yaml_array;
pub use yaml_array::YamlArray;

mod yaml_map;
pub use yaml_map::YamlMap;

mod yaml_value;
pub use yaml_value::YamlValue;

pub fn load_map_from_str(contents: &str) -> crate::Result<YamlMap> {
    // TODO: remove unwrap
    let mut yamls = yaml_rust::YamlLoader::load_from_str(contents).unwrap();
    let underlying = yamls.swap_remove(0);
    let value: YamlValue = underlying.try_into()?;
    value.try_into()
}
